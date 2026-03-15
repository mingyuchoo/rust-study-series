use anyhow::Result;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRecord {
    pub id: Option<Thing>,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostRecord {
    pub id: Option<Thing>,
    pub title: String,
    pub content: String,
    pub author_id: String,
    pub author_username: String,
    pub visibility: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommentRecord {
    pub id: Option<Thing>,
    pub content: String,
    pub post_id: String,
    pub author_id: String,
    pub author_username: String,
    pub visibility: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
struct CountResult {
    count: u64,
}

pub fn thing_to_id(thing: &Option<Thing>) -> String {
    thing.as_ref().map(|t| t.id.to_raw()).unwrap_or_default()
}

pub struct Database {
    client: Surreal<Client>,
}

impl Database {
    pub async fn new(addr: &str, username: &str, password: &str) -> Result<Self> {
        let client = Surreal::new::<Ws>(addr).await?;
        client.signin(Root { username, password }).await?;
        client.use_ns("blog").use_db("blog").await?;
        Ok(Self { client })
    }

    pub async fn init_schema(&self) -> Result<()> {
        self.client
            .query(
                "
                DEFINE TABLE user SCHEMAFULL;
                DEFINE FIELD username ON TABLE user TYPE string;
                DEFINE FIELD email ON TABLE user TYPE string;
                DEFINE FIELD password_hash ON TABLE user TYPE string;
                DEFINE FIELD role ON TABLE user TYPE string;
                DEFINE FIELD created_at ON TABLE user TYPE string;
                DEFINE INDEX idx_user_username ON TABLE user COLUMNS username UNIQUE;
                DEFINE INDEX idx_user_email ON TABLE user COLUMNS email UNIQUE;

                DEFINE TABLE post SCHEMAFULL;
                DEFINE FIELD title ON TABLE post TYPE string;
                DEFINE FIELD content ON TABLE post TYPE string;
                DEFINE FIELD author_id ON TABLE post TYPE string;
                DEFINE FIELD author_username ON TABLE post TYPE string;
                DEFINE FIELD visibility ON TABLE post TYPE string;
                DEFINE FIELD created_at ON TABLE post TYPE string;
                DEFINE FIELD updated_at ON TABLE post TYPE string;

                DEFINE TABLE comment SCHEMAFULL;
                DEFINE FIELD content ON TABLE comment TYPE string;
                DEFINE FIELD post_id ON TABLE comment TYPE string;
                DEFINE FIELD author_id ON TABLE comment TYPE string;
                DEFINE FIELD author_username ON TABLE comment TYPE string;
                DEFINE FIELD visibility ON TABLE comment TYPE string;
                DEFINE FIELD created_at ON TABLE comment TYPE string;
                ",
            )
            .await?;
        Ok(())
    }

    /// admin 사용자가 없으면 기본 관리자를 생성합니다.
    pub async fn seed_admin(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<bool> {
        let mut result = self
            .client
            .query("SELECT count() AS count FROM user WHERE role = 'admin' GROUP ALL")
            .await?;
        let counts: Vec<CountResult> = result.take(0)?;
        let admin_count = counts.first().map(|c| c.count).unwrap_or(0);

        if admin_count > 0 {
            return Ok(false);
        }

        let now = chrono::Utc::now().to_rfc3339();
        self.client
            .query("CREATE user SET username = $username, email = $email, password_hash = $password_hash, role = 'admin', created_at = $now")
            .bind(("username", username))
            .bind(("email", email))
            .bind(("password_hash", password_hash))
            .bind(("now", &now))
            .await?;

        Ok(true)
    }

    /// 포스트가 없을 때 샘플 사용자/포스트/댓글을 시딩합니다.
    pub async fn seed_sample_data(
        &self,
        users: &[crate::seed::UserSeed],
        posts: &[crate::seed::PostSeed],
    ) -> Result<u32> {
        // 이미 포스트가 존재하면 시딩하지 않음
        let mut count_result = self
            .client
            .query("SELECT count() AS count FROM post GROUP ALL")
            .await?;
        let counts: Vec<CountResult> = count_result.take(0)?;
        let existing = counts.first().map(|c| c.count).unwrap_or(0);
        if existing > 0 {
            return Ok(0);
        }

        // 샘플 사용자 생성
        for user in users {
            let hash = crate::auth::hash_password(&user.password)?;
            let _ = self.create_user(&user.username, &user.email, &hash).await;
        }

        // username -> user_id 매핑
        let mut user_map = std::collections::HashMap::new();
        let mut all_users = self.client.query("SELECT * FROM user").await?;
        let user_records: Vec<UserRecord> = all_users.take(0)?;
        for u in &user_records {
            user_map.insert(u.username.clone(), thing_to_id(&u.id));
        }

        let mut seeded = 0u32;
        for post_seed in posts {
            let author_id = match user_map.get(&post_seed.author) {
                Some(id) => id.clone(),
                None => continue,
            };

            let post = self
                .create_post(
                    &author_id,
                    &post_seed.author,
                    &post_seed.title,
                    &post_seed.content,
                    &post_seed.visibility,
                )
                .await?;
            let post_id = thing_to_id(&post.id);
            seeded += 1;

            for comment_seed in &post_seed.comments {
                let comment_author_id = match user_map.get(&comment_seed.author) {
                    Some(id) => id.clone(),
                    None => continue,
                };
                let _ = self
                    .create_comment(
                        &post_id,
                        &comment_author_id,
                        &comment_seed.author,
                        &comment_seed.content,
                        &comment_seed.visibility,
                    )
                    .await;
            }
        }

        Ok(seeded)
    }

    // ── User ──────────────────────────────────────────────

    pub async fn create_user(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<UserRecord> {
        let now = chrono::Utc::now().to_rfc3339();
        let mut result = self
            .client
            .query("CREATE user SET username = $username, email = $email, password_hash = $password_hash, role = 'user', created_at = $now")
            .bind(("username", username))
            .bind(("email", email))
            .bind(("password_hash", password_hash))
            .bind(("now", &now))
            .await?;
        let user: Option<UserRecord> = result.take(0)?;
        user.ok_or_else(|| anyhow::anyhow!("사용자 생성 실패"))
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<UserRecord>> {
        let mut result = self
            .client
            .query("SELECT * FROM user WHERE email = $email LIMIT 1")
            .bind(("email", email))
            .await?;
        let users: Vec<UserRecord> = result.take(0)?;
        Ok(users.into_iter().next())
    }

    pub async fn get_user_by_id(&self, id: &str) -> Result<Option<UserRecord>> {
        let record: Option<UserRecord> = self.client.select(("user", id)).await?;
        Ok(record)
    }

    pub async fn list_users(&self, page: u32, per_page: u32) -> Result<(Vec<UserRecord>, u32)> {
        let offset = page.saturating_sub(1) * per_page;
        let mut result = self
            .client
            .query("SELECT * FROM user ORDER BY created_at ASC LIMIT $limit START $offset")
            .bind(("limit", per_page))
            .bind(("offset", offset))
            .await?;
        let users: Vec<UserRecord> = result.take(0)?;

        let mut count_result = self
            .client
            .query("SELECT count() AS count FROM user GROUP ALL")
            .await?;
        let counts: Vec<CountResult> = count_result.take(0)?;
        let total = counts.first().map(|c| c.count as u32).unwrap_or(0);

        Ok((users, total))
    }

    pub async fn update_user_role(&self, user_id: &str, role: &str) -> Result<Option<UserRecord>> {
        let mut result = self
            .client
            .query("UPDATE type::thing('user', $id) SET role = $role RETURN AFTER")
            .bind(("id", user_id))
            .bind(("role", role))
            .await?;
        let users: Vec<UserRecord> = result.take(0)?;
        Ok(users.into_iter().next())
    }

    /// 사용자 삭제 (관련 포스트의 댓글, 포스트, 사용자 모두 삭제)
    pub async fn delete_user(&self, user_id: &str) -> Result<bool> {
        let user = self.get_user_by_id(user_id).await?;
        if user.is_none() {
            return Ok(false);
        }

        // 사용자의 포스트에 달린 댓글 삭제
        self.client
            .query("DELETE comment WHERE post_id IN (SELECT VALUE id FROM post WHERE author_id = $uid).map(|v| v.id.to_raw())")
            .bind(("uid", user_id))
            .await
            .ok();
        // 직접 쿼리로 사용자의 포스트에 달린 댓글 삭제
        let mut post_result = self
            .client
            .query("SELECT * FROM post WHERE author_id = $uid")
            .bind(("uid", user_id))
            .await?;
        let posts: Vec<PostRecord> = post_result.take(0)?;
        for post in &posts {
            let post_id = thing_to_id(&post.id);
            self.client
                .query("DELETE comment WHERE post_id = $pid")
                .bind(("pid", &post_id))
                .await?;
        }

        // 사용자가 작성한 댓글 삭제
        self.client
            .query("DELETE comment WHERE author_id = $uid")
            .bind(("uid", user_id))
            .await?;

        // 사용자의 포스트 삭제
        self.client
            .query("DELETE post WHERE author_id = $uid")
            .bind(("uid", user_id))
            .await?;

        // 사용자 삭제
        self.client
            .query("DELETE type::thing('user', $uid)")
            .bind(("uid", user_id))
            .await?;

        Ok(true)
    }

    // ── Post ──────────────────────────────────────────────

    pub async fn create_post(
        &self,
        author_id: &str,
        author_username: &str,
        title: &str,
        content: &str,
        visibility: &str,
    ) -> Result<PostRecord> {
        let now = chrono::Utc::now().to_rfc3339();
        let mut result = self
            .client
            .query("CREATE post SET title = $title, content = $content, author_id = $author_id, author_username = $author_username, visibility = $visibility, created_at = $now, updated_at = $now")
            .bind(("title", title))
            .bind(("content", content))
            .bind(("author_id", author_id))
            .bind(("author_username", author_username))
            .bind(("visibility", visibility))
            .bind(("now", &now))
            .await?;
        let post: Option<PostRecord> = result.take(0)?;
        post.ok_or_else(|| anyhow::anyhow!("포스트 생성 실패"))
    }

    pub async fn get_post(&self, id: &str) -> Result<Option<PostRecord>> {
        let record: Option<PostRecord> = self.client.select(("post", id)).await?;
        Ok(record)
    }

    /// 포스트 목록 조회 (가시성 필터링 적용)
    /// - admin: 전체 조회
    /// - user: public + 자신의 private 포스트
    /// - 비인증: public만
    pub async fn list_posts_filtered(
        &self,
        page: u32,
        per_page: u32,
        caller_id: Option<&str>,
        is_admin: bool,
    ) -> Result<(Vec<PostRecord>, u32)> {
        let offset = page.saturating_sub(1) * per_page;

        let (query, count_query) = if is_admin {
            (
                "SELECT * FROM post ORDER BY created_at DESC LIMIT $limit START $offset",
                "SELECT count() AS count FROM post GROUP ALL",
            )
        } else if let Some(uid) = caller_id {
            let mut result = self
                .client
                .query("SELECT * FROM post WHERE visibility = 'public' OR author_id = $uid ORDER BY created_at DESC LIMIT $limit START $offset")
                .bind(("uid", uid))
                .bind(("limit", per_page))
                .bind(("offset", offset))
                .await?;
            let posts: Vec<PostRecord> = result.take(0)?;

            let mut count_result = self
                .client
                .query("SELECT count() AS count FROM post WHERE visibility = 'public' OR author_id = $uid GROUP ALL")
                .bind(("uid", uid))
                .await?;
            let counts: Vec<CountResult> = count_result.take(0)?;
            let total = counts.first().map(|c| c.count as u32).unwrap_or(0);

            return Ok((posts, total));
        } else {
            (
                "SELECT * FROM post WHERE visibility = 'public' ORDER BY created_at DESC LIMIT $limit START $offset",
                "SELECT count() AS count FROM post WHERE visibility = 'public' GROUP ALL",
            )
        };

        let mut result = self
            .client
            .query(query)
            .bind(("limit", per_page))
            .bind(("offset", offset))
            .await?;
        let posts: Vec<PostRecord> = result.take(0)?;

        let mut count_result = self.client.query(count_query).await?;
        let counts: Vec<CountResult> = count_result.take(0)?;
        let total = counts.first().map(|c| c.count as u32).unwrap_or(0);

        Ok((posts, total))
    }

    /// 포스트 수정 (admin이면 소유자 확인 생략)
    pub async fn update_post(
        &self,
        id: &str,
        author_id: &str,
        title: &str,
        content: &str,
        visibility: &str,
        is_admin: bool,
    ) -> Result<Option<PostRecord>> {
        let post = self.get_post(id).await?;
        match post {
            Some(p) if is_admin || p.author_id == author_id => {}
            Some(_) => return Err(anyhow::anyhow!("권한이 없습니다.")),
            None => return Ok(None),
        }

        let now = chrono::Utc::now().to_rfc3339();
        let mut result = self
            .client
            .query("UPDATE type::thing('post', $id) SET title = $title, content = $content, visibility = $visibility, updated_at = $now RETURN AFTER")
            .bind(("id", id))
            .bind(("title", title))
            .bind(("content", content))
            .bind(("visibility", visibility))
            .bind(("now", &now))
            .await?;
        let posts: Vec<PostRecord> = result.take(0)?;
        Ok(posts.into_iter().next())
    }

    /// 포스트 삭제 (admin이면 소유자 확인 생략)
    pub async fn delete_post(&self, id: &str, author_id: &str, is_admin: bool) -> Result<bool> {
        let post = self.get_post(id).await?;
        match post {
            Some(p) if is_admin || p.author_id == author_id => {}
            _ => return Ok(false),
        }

        self.client
            .query("DELETE comment WHERE post_id = $post_id")
            .bind(("post_id", id))
            .await?;

        self.client
            .query("DELETE type::thing('post', $id)")
            .bind(("id", id))
            .await?;

        Ok(true)
    }

    pub async fn update_post_visibility(
        &self,
        id: &str,
        visibility: &str,
    ) -> Result<Option<PostRecord>> {
        let now = chrono::Utc::now().to_rfc3339();
        let mut result = self
            .client
            .query("UPDATE type::thing('post', $id) SET visibility = $visibility, updated_at = $now RETURN AFTER")
            .bind(("id", id))
            .bind(("visibility", visibility))
            .bind(("now", &now))
            .await?;
        let posts: Vec<PostRecord> = result.take(0)?;
        Ok(posts.into_iter().next())
    }

    // ── Comment ───────────────────────────────────────────

    pub async fn create_comment(
        &self,
        post_id: &str,
        author_id: &str,
        author_username: &str,
        content: &str,
        visibility: &str,
    ) -> Result<CommentRecord> {
        let now = chrono::Utc::now().to_rfc3339();
        let mut result = self
            .client
            .query("CREATE comment SET content = $content, post_id = $post_id, author_id = $author_id, author_username = $author_username, visibility = $visibility, created_at = $now")
            .bind(("content", content))
            .bind(("post_id", post_id))
            .bind(("author_id", author_id))
            .bind(("author_username", author_username))
            .bind(("visibility", visibility))
            .bind(("now", &now))
            .await?;
        let comment: Option<CommentRecord> = result.take(0)?;
        comment.ok_or_else(|| anyhow::anyhow!("댓글 생성 실패"))
    }

    pub async fn list_comments(&self, post_id: &str) -> Result<Vec<CommentRecord>> {
        let mut result = self
            .client
            .query("SELECT * FROM comment WHERE post_id = $post_id ORDER BY created_at ASC")
            .bind(("post_id", post_id))
            .await?;
        let comments: Vec<CommentRecord> = result.take(0)?;
        Ok(comments)
    }

    pub async fn get_comment(&self, id: &str) -> Result<Option<CommentRecord>> {
        let record: Option<CommentRecord> = self.client.select(("comment", id)).await?;
        Ok(record)
    }

    /// 댓글 수정 (admin이면 소유자 확인 생략)
    pub async fn update_comment(
        &self,
        id: &str,
        author_id: &str,
        content: &str,
        visibility: &str,
        is_admin: bool,
    ) -> Result<Option<CommentRecord>> {
        let record: Option<CommentRecord> = self.client.select(("comment", id)).await?;
        match record {
            Some(c) if is_admin || c.author_id == author_id => {}
            Some(_) => return Err(anyhow::anyhow!("권한이 없습니다.")),
            None => return Ok(None),
        }

        let mut result = self
            .client
            .query("UPDATE type::thing('comment', $id) SET content = $content, visibility = $visibility RETURN AFTER")
            .bind(("id", id))
            .bind(("content", content))
            .bind(("visibility", visibility))
            .await?;
        let comments: Vec<CommentRecord> = result.take(0)?;
        Ok(comments.into_iter().next())
    }

    /// 댓글 삭제 (admin이면 소유자 확인 생략)
    pub async fn delete_comment(&self, id: &str, author_id: &str, is_admin: bool) -> Result<bool> {
        let record: Option<CommentRecord> = self.client.select(("comment", id)).await?;
        match record {
            Some(c) if is_admin || c.author_id == author_id => {}
            _ => return Ok(false),
        }

        self.client
            .query("DELETE type::thing('comment', $id)")
            .bind(("id", id))
            .await?;

        Ok(true)
    }

    pub async fn count_comments(&self, post_id: &str) -> Result<u32> {
        let mut result = self
            .client
            .query("SELECT count() AS count FROM comment WHERE post_id = $post_id GROUP ALL")
            .bind(("post_id", post_id))
            .await?;
        let counts: Vec<CountResult> = result.take(0)?;
        Ok(counts.first().map(|c| c.count as u32).unwrap_or(0))
    }
}
