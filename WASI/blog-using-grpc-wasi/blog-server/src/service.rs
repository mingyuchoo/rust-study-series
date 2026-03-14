use std::sync::Arc;

use tonic::{Request, Response, Status};
use tracing::{info, instrument, warn};

use crate::auth;
use crate::db::{thing_to_id, Database};
use crate::proto::blog_service_server::BlogService;
use crate::proto::{
    AuthResponse, Comment, CommentResponse, CreateCommentRequest, CreatePostRequest,
    DeleteCommentRequest, DeletePostRequest, DeleteResponse, GetPostRequest, ListCommentsRequest,
    ListCommentsResponse, ListPostsRequest, ListPostsResponse, LoginRequest, Post, PostResponse,
    RegisterRequest, UpdatePostRequest, UserInfo, VersionRequest, VersionResponse,
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

    fn authenticate(&self, token: &str) -> Result<String, Status> {
        auth::verify_token(token).map_err(|e| Status::unauthenticated(e.to_string()))
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
        })
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

        if req.username.trim().is_empty() || req.email.trim().is_empty() || req.password.is_empty()
        {
            return Err(Status::invalid_argument(
                "사용자명, 이메일, 비밀번호는 필수 입력입니다.",
            ));
        }

        if req.password.len() < 8 {
            return Err(Status::invalid_argument(
                "비밀번호는 8자 이상이어야 합니다.",
            ));
        }

        let password_hash = auth::hash_password(&req.password)
            .map_err(|e| Status::internal(e.to_string()))?;

        let user = self
            .db
            .create_user(&req.username.trim(), &req.email.trim(), &password_hash)
            .await
            .map_err(|e| {
                warn!("회원가입 실패: {}", e);
                Status::already_exists("이미 존재하는 사용자명 또는 이메일입니다.")
            })?;

        let user_id = thing_to_id(&user.id);
        let token =
            auth::create_token(&user_id).map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(AuthResponse {
            token,
            user: Some(UserInfo {
                id: user_id,
                username: user.username,
                email: user.email,
                created_at: user.created_at,
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
        let token =
            auth::create_token(&user_id).map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(AuthResponse {
            token,
            user: Some(UserInfo {
                id: user_id,
                username: user.username,
                email: user.email,
                created_at: user.created_at,
            }),
        }))
    }

    // ── Posts ─────────────────────────────────────────────

    #[instrument(skip(self, request))]
    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        let req = request.into_inner();
        let user_id = self.authenticate(&req.token)?;

        // WASI 컴포넌트를 통한 유효성 검사
        let wasm = self.wasm.clone();
        let title = req.title.clone();
        let title_err =
            tokio::task::spawn_blocking(move || wasm.call_validate_title(&title))
                .await
                .map_err(|e| Status::internal(e.to_string()))?
                .map_err(|e| Status::internal(e.to_string()))?;
        if !title_err.is_empty() {
            return Err(Status::invalid_argument(title_err));
        }

        let wasm = self.wasm.clone();
        let content = req.content.clone();
        let content_err =
            tokio::task::spawn_blocking(move || wasm.call_validate_content(&content))
                .await
                .map_err(|e| Status::internal(e.to_string()))?
                .map_err(|e| Status::internal(e.to_string()))?;
        if !content_err.is_empty() {
            return Err(Status::invalid_argument(content_err));
        }

        let user = self.get_user_info(&user_id).await?;
        let post = self
            .db
            .create_post(&user_id, &user.username, &req.title, &req.content)
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
            }),
        }))
    }

    #[instrument(skip(self))]
    async fn get_post(
        &self,
        request: Request<GetPostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        let req = request.into_inner();

        let post = self
            .db
            .get_post(&req.id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("포스트를 찾을 수 없습니다."))?;

        let post_id = thing_to_id(&post.id);
        let comment_count = self
            .db
            .count_comments(&post_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let author = self.get_user_info(&post.author_id).await.unwrap_or(UserInfo {
            id: post.author_id.clone(),
            username: post.author_username.clone(),
            email: String::new(),
            created_at: String::new(),
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
            }),
        }))
    }

    #[instrument(skip(self))]
    async fn list_posts(
        &self,
        request: Request<ListPostsRequest>,
    ) -> Result<Response<ListPostsResponse>, Status> {
        let req = request.into_inner();
        let page = if req.page == 0 { 1 } else { req.page };
        let per_page = if req.per_page == 0 { 10 } else { req.per_page.min(50) };

        let (posts, total) = self
            .db
            .list_posts(page, per_page)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let mut result = Vec::with_capacity(posts.len());
        for post in posts {
            let post_id = thing_to_id(&post.id);
            let comment_count = self
                .db
                .count_comments(&post_id)
                .await
                .unwrap_or(0);

            result.push(Post {
                id: post_id,
                title: post.title,
                content: post.content,
                author: Some(UserInfo {
                    id: post.author_id,
                    username: post.author_username,
                    email: String::new(),
                    created_at: String::new(),
                }),
                created_at: post.created_at,
                updated_at: post.updated_at,
                comment_count,
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
        let user_id = self.authenticate(&req.token)?;

        // WASI 컴포넌트를 통한 유효성 검사
        let wasm = self.wasm.clone();
        let title = req.title.clone();
        let title_err =
            tokio::task::spawn_blocking(move || wasm.call_validate_title(&title))
                .await
                .map_err(|e| Status::internal(e.to_string()))?
                .map_err(|e| Status::internal(e.to_string()))?;
        if !title_err.is_empty() {
            return Err(Status::invalid_argument(title_err));
        }

        let wasm = self.wasm.clone();
        let content = req.content.clone();
        let content_err =
            tokio::task::spawn_blocking(move || wasm.call_validate_content(&content))
                .await
                .map_err(|e| Status::internal(e.to_string()))?
                .map_err(|e| Status::internal(e.to_string()))?;
        if !content_err.is_empty() {
            return Err(Status::invalid_argument(content_err));
        }

        let post = self
            .db
            .update_post(&req.id, &user_id, &req.title, &req.content)
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
            }),
        }))
    }

    #[instrument(skip(self, request))]
    async fn delete_post(
        &self,
        request: Request<DeletePostRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        let req = request.into_inner();
        let user_id = self.authenticate(&req.token)?;

        let success = self
            .db
            .delete_post(&req.id, &user_id)
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
        let user_id = self.authenticate(&req.token)?;

        // WASI 컴포넌트를 통한 유효성 검사
        let wasm = self.wasm.clone();
        let content = req.content.clone();
        let content_err =
            tokio::task::spawn_blocking(move || wasm.call_validate_comment(&content))
                .await
                .map_err(|e| Status::internal(e.to_string()))?
                .map_err(|e| Status::internal(e.to_string()))?;
        if !content_err.is_empty() {
            return Err(Status::invalid_argument(content_err));
        }

        // 포스트 존재 확인
        self.db
            .get_post(&req.post_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("포스트를 찾을 수 없습니다."))?;

        let user = self.get_user_info(&user_id).await?;
        let comment = self
            .db
            .create_comment(&req.post_id, &user_id, &user.username, &req.content)
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
            }),
        }))
    }

    #[instrument(skip(self))]
    async fn list_comments(
        &self,
        request: Request<ListCommentsRequest>,
    ) -> Result<Response<ListCommentsResponse>, Status> {
        let req = request.into_inner();

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
                }),
                post_id: c.post_id,
                created_at: c.created_at,
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
        let user_id = self.authenticate(&req.token)?;

        let success = self
            .db
            .delete_comment(&req.id, &user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(DeleteResponse { success }))
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
