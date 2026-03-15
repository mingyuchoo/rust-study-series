#![allow(clippy::result_large_err)]

use std::sync::Arc;

use tonic::{Request, Response, Status};
use tracing::{info, instrument, warn};

use crate::auth;
use crate::db::{thing_to_id, Database};
use crate::proto::blog_service_server::BlogService;
use crate::proto::{
    AuthResponse, ChangePasswordRequest, ChangePasswordResponse, Comment, CommentResponse,
    CreateCommentRequest, CreatePostRequest, DeleteCommentRequest, DeleteMyAccountRequest,
    DeletePostRequest, DeleteResponse, DeleteUserRequest, GetMyProfileRequest, GetPostRequest,
    GetUserRequest, ListCommentsRequest, ListCommentsResponse, ListPostsRequest, ListPostsResponse,
    ListUsersRequest, ListUsersResponse, LoginRequest, Post, PostResponse, RegisterRequest,
    SearchPostsRequest, UpdateCommentRequest, UpdatePostRequest, UpdatePostVisibilityRequest,
    UpdateProfileRequest, UpdateUserRoleRequest, UserInfo, UserResponse, VersionRequest,
    VersionResponse,
};
use crate::WasmRuntime;

pub struct BlogServiceImpl {
    db: Arc<Database>,
    wasm: Arc<WasmRuntime>,
}

impl BlogServiceImpl {
    pub fn new(db: Arc<Database>, wasm: Arc<WasmRuntime>) -> Self {
        Self { db, wasm }
    }

    /// 토큰을 검증하고 (user_id, role)을 반환합니다.
    fn authenticate(&self, token: &str) -> Result<(String, String), Status> {
        auth::verify_token(token).map_err(|e| Status::unauthenticated(e.to_string()))
    }

    /// 토큰이 빈 문자열이면 None, 아니면 인증을 시도합니다.
    fn try_authenticate(&self, token: &str) -> Result<Option<(String, String)>, Status> {
        if token.is_empty() {
            Ok(None)
        } else {
            self.authenticate(token).map(Some)
        }
    }

    fn require_admin(role: &str) -> Result<(), Status> {
        if role != "admin" {
            return Err(Status::permission_denied("관리자 권한이 필요합니다."));
        }
        Ok(())
    }

    async fn get_user_info(&self, user_id: &str) -> Result<UserInfo, Status> {
        let user = self
            .db
            .get_user_by_id(user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("사용자를 찾을 수 없습니다."))?;

        Ok(UserInfo {
            id: thing_to_id(&user.id),
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            role: user.role,
            bio: user.bio,
            website: user.website,
            theme: user.theme,
        })
    }

    /// 포스트 읽기 권한을 확인합니다.
    fn can_read_post(
        post_visibility: &str,
        post_author_id: &str,
        caller: &Option<(String, String)>,
    ) -> bool {
        if post_visibility == "public" {
            return true;
        }
        match caller {
            Some((uid, role)) => role == "admin" || uid == post_author_id,
            None => false,
        }
    }
}

#[tonic::async_trait]
impl BlogService for BlogServiceImpl {
    // ── Authentication ────────────────────────────────────

    #[instrument(skip(self, request))]
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        info!("회원가입 요청: {}", req.username);

        // WASI 컴포넌트를 통한 사용자명 유효성 검사
        let wasm = self.wasm.clone();
        let username = req.username.clone();
        let username_err =
            tokio::task::spawn_blocking(move || wasm.call_validate_username(&username))
                .await
                .map_err(|e| Status::internal(e.to_string()))?
                .map_err(|e| Status::internal(e.to_string()))?;
        if !username_err.is_empty() {
            return Err(Status::invalid_argument(username_err));
        }

        // WASI 컴포넌트를 통한 이메일 유효성 검사
        let wasm = self.wasm.clone();
        let email = req.email.clone();
        let email_err = tokio::task::spawn_blocking(move || wasm.call_validate_email(&email))
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .map_err(|e| Status::internal(e.to_string()))?;
        if !email_err.is_empty() {
            return Err(Status::invalid_argument(email_err));
        }

        // WASI 컴포넌트를 통한 비밀번호 강도 검사
        let wasm = self.wasm.clone();
        let password = req.password.clone();
        let password_err =
            tokio::task::spawn_blocking(move || wasm.call_validate_password_strength(&password))
                .await
                .map_err(|e| Status::internal(e.to_string()))?
                .map_err(|e| Status::internal(e.to_string()))?;
        if !password_err.is_empty() {
            return Err(Status::invalid_argument(password_err));
        }

        let password_hash =
            auth::hash_password(&req.password).map_err(|e| Status::internal(e.to_string()))?;

        let user = self
            .db
            .create_user(req.username.trim(), req.email.trim(), &password_hash)
            .await
            .map_err(|e| {
                warn!("회원가입 실패: {}", e);
                Status::already_exists("이미 존재하는 사용자명 또는 이메일입니다.")
            })?;

        let user_id = thing_to_id(&user.id);
        let token = auth::create_token(&user_id, &user.role)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(AuthResponse {
            token,
            user: Some(UserInfo {
                id: user_id,
                username: user.username,
                email: user.email,
                created_at: user.created_at,
                role: user.role,
                bio: user.bio,
                website: user.website,
                theme: user.theme,
            }),
        }))
    }

    #[instrument(skip(self, request))]
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        info!("로그인 요청: {}", req.email);

        let user = self
            .db
            .get_user_by_email(&req.email)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::unauthenticated("이메일 또는 비밀번호가 올바르지 않습니다."))?;

        let valid = auth::verify_password(&req.password, &user.password_hash)
            .map_err(|e| Status::internal(e.to_string()))?;

        if !valid {
            return Err(Status::unauthenticated(
                "이메일 또는 비밀번호가 올바르지 않습니다.",
            ));
        }

        let user_id = thing_to_id(&user.id);
        let token = auth::create_token(&user_id, &user.role)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(AuthResponse {
            token,
            user: Some(UserInfo {
                id: user_id,
                username: user.username,
                email: user.email,
                created_at: user.created_at,
                role: user.role,
                bio: user.bio,
                website: user.website,
                theme: user.theme,
            }),
        }))
    }

    // ── Profile ────────────────────────────────────────────

    #[instrument(skip(self, request))]
    async fn get_my_profile(
        &self,
        request: Request<GetMyProfileRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();
        let (user_id, _role) = self.authenticate(&req.token)?;
        let user = self.get_user_info(&user_id).await?;
        Ok(Response::new(UserResponse { user: Some(user) }))
    }

    #[instrument(skip(self, request))]
    async fn update_profile(
        &self,
        request: Request<UpdateProfileRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();
        let (user_id, _role) = self.authenticate(&req.token)?;

        // WASI 컴포넌트를 통한 유효성 검사
        let wasm = self.wasm.clone();
        let bio = req.bio.clone();
        let bio_err = tokio::task::spawn_blocking(move || wasm.call_validate_bio(&bio))
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .map_err(|e| Status::internal(e.to_string()))?;
        if !bio_err.is_empty() {
            return Err(Status::invalid_argument(bio_err));
        }

        let wasm = self.wasm.clone();
        let website = req.website.clone();
        let website_err =
            tokio::task::spawn_blocking(move || wasm.call_validate_website(&website))
                .await
                .map_err(|e| Status::internal(e.to_string()))?
                .map_err(|e| Status::internal(e.to_string()))?;
        if !website_err.is_empty() {
            return Err(Status::invalid_argument(website_err));
        }

        let wasm = self.wasm.clone();
        let theme = req.theme.clone();
        let theme_val = if theme.is_empty() {
            "dark".to_string()
        } else {
            let theme_err =
                tokio::task::spawn_blocking(move || wasm.call_validate_theme(&theme))
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?
                    .map_err(|e| Status::internal(e.to_string()))?;
            if !theme_err.is_empty() {
                return Err(Status::invalid_argument(theme_err));
            }
            req.theme.clone()
        };

        let user = self
            .db
            .update_profile(&user_id, &req.bio, &req.website, &theme_val)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("사용자를 찾을 수 없습니다."))?;

        info!("프로필 업데이트: {}", user_id);
        Ok(Response::new(UserResponse {
            user: Some(UserInfo {
                id: thing_to_id(&user.id),
                username: user.username,
                email: user.email,
                created_at: user.created_at,
                role: user.role,
                bio: user.bio,
                website: user.website,
                theme: user.theme,
            }),
        }))
    }

    #[instrument(skip(self, request))]
    async fn change_password(
        &self,
        request: Request<ChangePasswordRequest>,
    ) -> Result<Response<ChangePasswordResponse>, Status> {
        let req = request.into_inner();
        let (user_id, _role) = self.authenticate(&req.token)?;

        // 현재 비밀번호 확인
        let user = self
            .db
            .get_user_by_id(&user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("사용자를 찾을 수 없습니다."))?;

        let valid = auth::verify_password(&req.current_password, &user.password_hash)
            .map_err(|e| Status::internal(e.to_string()))?;
        if !valid {
            return Err(Status::unauthenticated("현재 비밀번호가 올바르지 않습니다."));
        }

        // WASI 컴포넌트를 통한 새 비밀번호 강도 검사
        let wasm = self.wasm.clone();
        let new_pw = req.new_password.clone();
        let pw_err =
            tokio::task::spawn_blocking(move || wasm.call_validate_password_strength(&new_pw))
                .await
                .map_err(|e| Status::internal(e.to_string()))?
                .map_err(|e| Status::internal(e.to_string()))?;
        if !pw_err.is_empty() {
            return Err(Status::invalid_argument(pw_err));
        }

        let new_hash =
            auth::hash_password(&req.new_password).map_err(|e| Status::internal(e.to_string()))?;

        self.db
            .update_password(&user_id, &new_hash)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        info!("비밀번호 변경 완료: {}", user_id);
        Ok(Response::new(ChangePasswordResponse {
            success: true,
            message: "비밀번호가 변경되었습니다.".to_string(),
        }))
    }

    #[instrument(skip(self, request))]
    async fn delete_my_account(
        &self,
        request: Request<DeleteMyAccountRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        let req = request.into_inner();
        let (user_id, _role) = self.authenticate(&req.token)?;

        // 비밀번호 확인
        let user = self
            .db
            .get_user_by_id(&user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("사용자를 찾을 수 없습니다."))?;

        let valid = auth::verify_password(&req.password, &user.password_hash)
            .map_err(|e| Status::internal(e.to_string()))?;
        if !valid {
            return Err(Status::unauthenticated("비밀번호가 올바르지 않습니다."));
        }

        let success = self
            .db
            .delete_user(&user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if success {
            info!("회원 탈퇴: {}", user_id);
        }

        Ok(Response::new(DeleteResponse { success }))
    }

    // ── Posts ─────────────────────────────────────────────

    #[instrument(skip(self, request))]
    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        let req = request.into_inner();
        let (user_id, _role) = self.authenticate(&req.token)?;

        let visibility = if req.visibility.is_empty() {
            "private".to_string()
        } else {
            req.visibility.clone()
        };

        // WASI 컴포넌트를 통한 유효성 검사
        let wasm = self.wasm.clone();
        let title = req.title.clone();
        let title_err = tokio::task::spawn_blocking(move || wasm.call_validate_title(&title))
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .map_err(|e| Status::internal(e.to_string()))?;
        if !title_err.is_empty() {
            return Err(Status::invalid_argument(title_err));
        }

        let wasm = self.wasm.clone();
        let content = req.content.clone();
        let content_err = tokio::task::spawn_blocking(move || wasm.call_validate_content(&content))
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .map_err(|e| Status::internal(e.to_string()))?;
        if !content_err.is_empty() {
            return Err(Status::invalid_argument(content_err));
        }

        let wasm = self.wasm.clone();
        let vis = visibility.clone();
        let vis_err = tokio::task::spawn_blocking(move || wasm.call_validate_visibility(&vis))
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .map_err(|e| Status::internal(e.to_string()))?;
        if !vis_err.is_empty() {
            return Err(Status::invalid_argument(vis_err));
        }

        // WASI 컴포넌트를 통한 콘텐츠 정제 (XSS 방지)
        let wasm = self.wasm.clone();
        let raw_content = req.content.clone();
        let sanitized_content =
            tokio::task::spawn_blocking(move || wasm.call_sanitize_content(&raw_content))
                .await
                .map_err(|e| Status::internal(e.to_string()))?
                .map_err(|e| Status::internal(e.to_string()))?;

        let user = self.get_user_info(&user_id).await?;
        let post = self
            .db
            .create_post(
                &user_id,
                &user.username,
                &req.title,
                &sanitized_content,
                &visibility,
            )
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        info!("포스트 생성 완료: {}", req.title);
        Ok(Response::new(PostResponse {
            post: Some(Post {
                id: thing_to_id(&post.id),
                title: post.title,
                content: post.content,
                author: Some(user),
                created_at: post.created_at,
                updated_at: post.updated_at,
                comment_count: 0,
                visibility: post.visibility,
            }),
        }))
    }

    #[instrument(skip(self))]
    async fn get_post(
        &self,
        request: Request<GetPostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        let req = request.into_inner();
        let caller = self.try_authenticate(&req.token)?;

        let post = self
            .db
            .get_post(&req.id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("포스트를 찾을 수 없습니다."))?;

        if !Self::can_read_post(&post.visibility, &post.author_id, &caller) {
            return Err(Status::permission_denied("이 포스트를 볼 권한이 없습니다."));
        }

        let post_id = thing_to_id(&post.id);
        let comment_count = self
            .db
            .count_comments(&post_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let author = self
            .get_user_info(&post.author_id)
            .await
            .unwrap_or(UserInfo {
                id: post.author_id.clone(),
                username: post.author_username.clone(),
                email: String::new(),
                created_at: String::new(),
                role: String::new(),
                bio: String::new(),
                website: String::new(),
                theme: String::new(),
            });

        Ok(Response::new(PostResponse {
            post: Some(Post {
                id: post_id,
                title: post.title,
                content: post.content,
                author: Some(author),
                created_at: post.created_at,
                updated_at: post.updated_at,
                comment_count,
                visibility: post.visibility,
            }),
        }))
    }

    #[instrument(skip(self))]
    async fn list_posts(
        &self,
        request: Request<ListPostsRequest>,
    ) -> Result<Response<ListPostsResponse>, Status> {
        let req = request.into_inner();
        let caller = self.try_authenticate(&req.token)?;

        let page = if req.page == 0 { 1 } else { req.page };
        let per_page = if req.per_page == 0 {
            10
        } else {
            req.per_page.min(50)
        };

        let is_admin = caller.as_ref().is_some_and(|(_, r)| r == "admin");
        let caller_id = caller.as_ref().map(|(uid, _)| uid.as_str());

        let (posts, total) = self
            .db
            .list_posts_filtered(page, per_page, caller_id, is_admin, &req.filter)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let mut result = Vec::with_capacity(posts.len());
        for post in posts {
            let post_id = thing_to_id(&post.id);
            let comment_count = self.db.count_comments(&post_id).await.unwrap_or(0);

            result.push(Post {
                id: post_id,
                title: post.title,
                content: post.content,
                author: Some(UserInfo {
                    id: post.author_id,
                    username: post.author_username,
                    email: String::new(),
                    created_at: String::new(),
                    role: String::new(),
                    bio: String::new(),
                    website: String::new(),
                    theme: String::new(),
                }),
                created_at: post.created_at,
                updated_at: post.updated_at,
                comment_count,
                visibility: post.visibility,
            });
        }

        Ok(Response::new(ListPostsResponse {
            posts: result,
            total,
        }))
    }

    #[instrument(skip(self, request))]
    async fn search_posts(
        &self,
        request: Request<SearchPostsRequest>,
    ) -> Result<Response<ListPostsResponse>, Status> {
        let req = request.into_inner();
        let caller = self.try_authenticate(&req.token)?;

        if req.query.trim().is_empty() {
            return Err(Status::invalid_argument("검색어를 입력해주세요."));
        }
        if req.query.len() > 100 {
            return Err(Status::invalid_argument(
                "검색어는 100자를 초과할 수 없습니다.",
            ));
        }

        let page = if req.page == 0 { 1 } else { req.page };
        let per_page = if req.per_page == 0 {
            10
        } else {
            req.per_page.min(50)
        };

        let is_admin = caller.as_ref().is_some_and(|(_, r)| r == "admin");
        let caller_id = caller.as_ref().map(|(uid, _)| uid.as_str());

        let (posts, total) = self
            .db
            .search_posts(&req.query, page, per_page, caller_id, is_admin)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let mut result = Vec::with_capacity(posts.len());
        for post in posts {
            let post_id = thing_to_id(&post.id);
            let comment_count = self.db.count_comments(&post_id).await.unwrap_or(0);

            result.push(Post {
                id: post_id,
                title: post.title,
                content: post.content,
                author: Some(UserInfo {
                    id: post.author_id,
                    username: post.author_username,
                    email: String::new(),
                    created_at: String::new(),
                    role: String::new(),
                    bio: String::new(),
                    website: String::new(),
                    theme: String::new(),
                }),
                created_at: post.created_at,
                updated_at: post.updated_at,
                comment_count,
                visibility: post.visibility,
            });
        }

        Ok(Response::new(ListPostsResponse {
            posts: result,
            total,
        }))
    }

    #[instrument(skip(self, request))]
    async fn update_post(
        &self,
        request: Request<UpdatePostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        let req = request.into_inner();
        let (user_id, role) = self.authenticate(&req.token)?;
        let is_admin = role == "admin";

        // WASI 컴포넌트를 통한 유효성 검사
        let wasm = self.wasm.clone();
        let title = req.title.clone();
        let title_err = tokio::task::spawn_blocking(move || wasm.call_validate_title(&title))
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .map_err(|e| Status::internal(e.to_string()))?;
        if !title_err.is_empty() {
            return Err(Status::invalid_argument(title_err));
        }

        let wasm = self.wasm.clone();
        let content = req.content.clone();
        let content_err = tokio::task::spawn_blocking(move || wasm.call_validate_content(&content))
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .map_err(|e| Status::internal(e.to_string()))?;
        if !content_err.is_empty() {
            return Err(Status::invalid_argument(content_err));
        }

        // visibility가 비어있으면 기존 값 유지
        let visibility = if req.visibility.is_empty() {
            let existing = self
                .db
                .get_post(&req.id)
                .await
                .map_err(|e| Status::internal(e.to_string()))?
                .ok_or_else(|| Status::not_found("포스트를 찾을 수 없습니다."))?;
            existing.visibility
        } else {
            let wasm = self.wasm.clone();
            let vis = req.visibility.clone();
            let vis_err = tokio::task::spawn_blocking(move || wasm.call_validate_visibility(&vis))
                .await
                .map_err(|e| Status::internal(e.to_string()))?
                .map_err(|e| Status::internal(e.to_string()))?;
            if !vis_err.is_empty() {
                return Err(Status::invalid_argument(vis_err));
            }
            req.visibility.clone()
        };

        // WASI 컴포넌트를 통한 콘텐츠 정제 (XSS 방지)
        let wasm = self.wasm.clone();
        let raw_content = req.content.clone();
        let sanitized_content =
            tokio::task::spawn_blocking(move || wasm.call_sanitize_content(&raw_content))
                .await
                .map_err(|e| Status::internal(e.to_string()))?
                .map_err(|e| Status::internal(e.to_string()))?;

        let post = self
            .db
            .update_post(
                &req.id,
                &user_id,
                &req.title,
                &sanitized_content,
                &visibility,
                is_admin,
            )
            .await
            .map_err(|e| Status::permission_denied(e.to_string()))?
            .ok_or_else(|| Status::not_found("포스트를 찾을 수 없습니다."))?;

        let post_id = thing_to_id(&post.id);
        let user = self.get_user_info(&user_id).await?;

        Ok(Response::new(PostResponse {
            post: Some(Post {
                id: post_id,
                title: post.title,
                content: post.content,
                author: Some(user),
                created_at: post.created_at,
                updated_at: post.updated_at,
                comment_count: 0,
                visibility: post.visibility,
            }),
        }))
    }

    #[instrument(skip(self, request))]
    async fn delete_post(
        &self,
        request: Request<DeletePostRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        let req = request.into_inner();
        let (user_id, role) = self.authenticate(&req.token)?;
        let is_admin = role == "admin";

        let success = self
            .db
            .delete_post(&req.id, &user_id, is_admin)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(DeleteResponse { success }))
    }

    // ── Comments ─────────────────────────────────────────

    #[instrument(skip(self, request))]
    async fn create_comment(
        &self,
        request: Request<CreateCommentRequest>,
    ) -> Result<Response<CommentResponse>, Status> {
        let req = request.into_inner();
        let (user_id, role) = self.authenticate(&req.token)?;

        // WASI 컴포넌트를 통한 유효성 검사
        let wasm = self.wasm.clone();
        let content = req.content.clone();
        let content_err = tokio::task::spawn_blocking(move || wasm.call_validate_comment(&content))
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .map_err(|e| Status::internal(e.to_string()))?;
        if !content_err.is_empty() {
            return Err(Status::invalid_argument(content_err));
        }

        // 포스트 존재 및 읽기 권한 확인
        let post = self
            .db
            .get_post(&req.post_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("포스트를 찾을 수 없습니다."))?;

        let caller = Some((user_id.clone(), role));
        if !Self::can_read_post(&post.visibility, &post.author_id, &caller) {
            return Err(Status::permission_denied(
                "이 포스트에 댓글을 달 권한이 없습니다.",
            ));
        }

        let visibility = if req.visibility.is_empty() {
            "private".to_string()
        } else {
            req.visibility.clone()
        };

        // WASI 컴포넌트를 통한 콘텐츠 정제 (XSS 방지)
        let wasm = self.wasm.clone();
        let raw_content = req.content.clone();
        let sanitized_content =
            tokio::task::spawn_blocking(move || wasm.call_sanitize_content(&raw_content))
                .await
                .map_err(|e| Status::internal(e.to_string()))?
                .map_err(|e| Status::internal(e.to_string()))?;

        let user = self.get_user_info(&user_id).await?;
        let comment = self
            .db
            .create_comment(
                &req.post_id,
                &user_id,
                &user.username,
                &sanitized_content,
                &visibility,
            )
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        info!("댓글 생성 완료: post_id={}", req.post_id);
        Ok(Response::new(CommentResponse {
            comment: Some(Comment {
                id: thing_to_id(&comment.id),
                content: comment.content,
                author: Some(user),
                post_id: comment.post_id,
                created_at: comment.created_at,
                visibility: comment.visibility,
            }),
        }))
    }

    #[instrument(skip(self))]
    async fn list_comments(
        &self,
        request: Request<ListCommentsRequest>,
    ) -> Result<Response<ListCommentsResponse>, Status> {
        let req = request.into_inner();
        let caller = self.try_authenticate(&req.token)?;

        // 포스트 존재 및 읽기 권한 확인
        let post = self
            .db
            .get_post(&req.post_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("포스트를 찾을 수 없습니다."))?;

        if !Self::can_read_post(&post.visibility, &post.author_id, &caller) {
            return Err(Status::permission_denied(
                "이 포스트의 댓글을 볼 권한이 없습니다.",
            ));
        }

        let comments = self
            .db
            .list_comments(&req.post_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let result: Vec<Comment> = comments
            .into_iter()
            .map(|c| Comment {
                id: thing_to_id(&c.id),
                content: c.content,
                author: Some(UserInfo {
                    id: c.author_id,
                    username: c.author_username,
                    email: String::new(),
                    created_at: String::new(),
                    role: String::new(),
                    bio: String::new(),
                    website: String::new(),
                    theme: String::new(),
                }),
                post_id: c.post_id,
                created_at: c.created_at,
                visibility: c.visibility,
            })
            .collect();

        Ok(Response::new(ListCommentsResponse { comments: result }))
    }

    #[instrument(skip(self, request))]
    async fn delete_comment(
        &self,
        request: Request<DeleteCommentRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        let req = request.into_inner();
        let (user_id, role) = self.authenticate(&req.token)?;
        let is_admin = role == "admin";

        let success = self
            .db
            .delete_comment(&req.id, &user_id, is_admin)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(DeleteResponse { success }))
    }

    #[instrument(skip(self, request))]
    async fn update_comment(
        &self,
        request: Request<UpdateCommentRequest>,
    ) -> Result<Response<CommentResponse>, Status> {
        let req = request.into_inner();
        let (user_id, role) = self.authenticate(&req.token)?;
        let is_admin = role == "admin";

        // WASI 컴포넌트를 통한 유효성 검사
        let wasm = self.wasm.clone();
        let content = req.content.clone();
        let content_err = tokio::task::spawn_blocking(move || wasm.call_validate_comment(&content))
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .map_err(|e| Status::internal(e.to_string()))?;
        if !content_err.is_empty() {
            return Err(Status::invalid_argument(content_err));
        }

        // visibility가 비어있으면 기존 값 유지
        let visibility = if req.visibility.is_empty() {
            let existing = self
                .db
                .get_comment(&req.id)
                .await
                .map_err(|e| Status::internal(e.to_string()))?
                .ok_or_else(|| Status::not_found("댓글을 찾을 수 없습니다."))?;
            existing.visibility
        } else {
            req.visibility.clone()
        };

        // WASI 컴포넌트를 통한 콘텐츠 정제 (XSS 방지)
        let wasm = self.wasm.clone();
        let raw_content = req.content.clone();
        let sanitized_content =
            tokio::task::spawn_blocking(move || wasm.call_sanitize_content(&raw_content))
                .await
                .map_err(|e| Status::internal(e.to_string()))?
                .map_err(|e| Status::internal(e.to_string()))?;

        let comment = self
            .db
            .update_comment(&req.id, &user_id, &sanitized_content, &visibility, is_admin)
            .await
            .map_err(|e| Status::permission_denied(e.to_string()))?
            .ok_or_else(|| Status::not_found("댓글을 찾을 수 없습니다."))?;

        let user = self.get_user_info(&user_id).await?;

        Ok(Response::new(CommentResponse {
            comment: Some(Comment {
                id: thing_to_id(&comment.id),
                content: comment.content,
                author: Some(user),
                post_id: comment.post_id,
                created_at: comment.created_at,
                visibility: comment.visibility,
            }),
        }))
    }

    // ── Admin ─────────────────────────────────────────────

    #[instrument(skip(self, request))]
    async fn list_users(
        &self,
        request: Request<ListUsersRequest>,
    ) -> Result<Response<ListUsersResponse>, Status> {
        let req = request.into_inner();
        let (_user_id, role) = self.authenticate(&req.token)?;
        Self::require_admin(&role)?;

        let page = if req.page == 0 { 1 } else { req.page };
        let per_page = if req.per_page == 0 {
            10
        } else {
            req.per_page.min(50)
        };

        let (users, total) = self
            .db
            .list_users(page, per_page)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let result: Vec<UserInfo> = users
            .into_iter()
            .map(|u| UserInfo {
                id: thing_to_id(&u.id),
                username: u.username,
                email: u.email,
                created_at: u.created_at,
                role: u.role,
                bio: u.bio,
                website: u.website,
                theme: u.theme,
            })
            .collect();

        Ok(Response::new(ListUsersResponse {
            users: result,
            total,
        }))
    }

    #[instrument(skip(self, request))]
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();
        let (_user_id, role) = self.authenticate(&req.token)?;
        Self::require_admin(&role)?;

        let user = self.get_user_info(&req.user_id).await?;

        Ok(Response::new(UserResponse { user: Some(user) }))
    }

    #[instrument(skip(self, request))]
    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        let req = request.into_inner();
        let (_user_id, role) = self.authenticate(&req.token)?;
        Self::require_admin(&role)?;

        let success = self
            .db
            .delete_user(&req.user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if success {
            info!("사용자 삭제: {}", req.user_id);
        }

        Ok(Response::new(DeleteResponse { success }))
    }

    #[instrument(skip(self, request))]
    async fn update_user_role(
        &self,
        request: Request<UpdateUserRoleRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();
        let (_user_id, role) = self.authenticate(&req.token)?;
        Self::require_admin(&role)?;

        // WASI 컴포넌트를 통한 역할 유효성 검사
        let wasm = self.wasm.clone();
        let new_role = req.role.clone();
        let role_err = tokio::task::spawn_blocking(move || wasm.call_validate_role(&new_role))
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .map_err(|e| Status::internal(e.to_string()))?;
        if !role_err.is_empty() {
            return Err(Status::invalid_argument(role_err));
        }

        let user = self
            .db
            .update_user_role(&req.user_id, &req.role)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("사용자를 찾을 수 없습니다."))?;

        info!("사용자 역할 변경: {} -> {}", req.user_id, req.role);
        Ok(Response::new(UserResponse {
            user: Some(UserInfo {
                id: thing_to_id(&user.id),
                username: user.username,
                email: user.email,
                created_at: user.created_at,
                role: user.role,
                bio: user.bio,
                website: user.website,
                theme: user.theme,
            }),
        }))
    }

    #[instrument(skip(self, request))]
    async fn update_post_visibility(
        &self,
        request: Request<UpdatePostVisibilityRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        let req = request.into_inner();
        let (user_id, role) = self.authenticate(&req.token)?;
        Self::require_admin(&role)?;

        // WASI 컴포넌트를 통한 공개범위 유효성 검사
        let wasm = self.wasm.clone();
        let vis = req.visibility.clone();
        let vis_err = tokio::task::spawn_blocking(move || wasm.call_validate_visibility(&vis))
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .map_err(|e| Status::internal(e.to_string()))?;
        if !vis_err.is_empty() {
            return Err(Status::invalid_argument(vis_err));
        }

        let post = self
            .db
            .update_post_visibility(&req.post_id, &req.visibility)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("포스트를 찾을 수 없습니다."))?;

        let post_id = thing_to_id(&post.id);
        let comment_count = self.db.count_comments(&post_id).await.unwrap_or(0);

        let author = self
            .get_user_info(&post.author_id)
            .await
            .unwrap_or(UserInfo {
                id: post.author_id.clone(),
                username: post.author_username.clone(),
                email: String::new(),
                created_at: String::new(),
                role: String::new(),
                bio: String::new(),
                website: String::new(),
                theme: String::new(),
            });

        info!(
            "포스트 공개범위 변경: {} -> {} (by admin {})",
            req.post_id, req.visibility, user_id
        );
        Ok(Response::new(PostResponse {
            post: Some(Post {
                id: post_id,
                title: post.title,
                content: post.content,
                author: Some(author),
                created_at: post.created_at,
                updated_at: post.updated_at,
                comment_count,
                visibility: post.visibility,
            }),
        }))
    }

    // ── System ───────────────────────────────────────────

    #[instrument(skip(self))]
    async fn get_version(
        &self,
        _request: Request<VersionRequest>,
    ) -> Result<Response<VersionResponse>, Status> {
        let wasm = self.wasm.clone();
        let version = tokio::task::spawn_blocking(move || wasm.call_get_version())
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(VersionResponse { version }))
    }
}
