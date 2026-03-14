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
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostRecord {
    pub id: Option<Thing>,
    pub title: String,
    pub content: String,
    pub author_id: String,
    pub author_username: String,
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
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
struct CountResult {
    count: u64,
}

pub fn thing_to_id(thing: &Option<Thing>) -> String {
    thing
        .as_ref()
        .map(|t| t.id.to_raw())
        .unwrap_or_default()
}

pub struct Database {
    client: Surreal<Client>,
}

impl Database {
    pub async fn new(addr: &str, username: &str, password: &str) -> Result<Self> {
        let client = Surreal::new::<Ws>(addr).await?;
        client
            .signin(Root {
                username,
                password,
            })
            .await?;
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
                DEFINE FIELD created_at ON TABLE user TYPE string;
                DEFINE INDEX idx_user_username ON TABLE user COLUMNS username UNIQUE;
                DEFINE INDEX idx_user_email ON TABLE user COLUMNS email UNIQUE;

                DEFINE TABLE post SCHEMAFULL;
                DEFINE FIELD title ON TABLE post TYPE string;
                DEFINE FIELD content ON TABLE post TYPE string;
                DEFINE FIELD author_id ON TABLE post TYPE string;
                DEFINE FIELD author_username ON TABLE post TYPE string;
                DEFINE FIELD created_at ON TABLE post TYPE string;
                DEFINE FIELD updated_at ON TABLE post TYPE string;

                DEFINE TABLE comment SCHEMAFULL;
                DEFINE FIELD content ON TABLE comment TYPE string;
                DEFINE FIELD post_id ON TABLE comment TYPE string;
                DEFINE FIELD author_id ON TABLE comment TYPE string;
                DEFINE FIELD author_username ON TABLE comment TYPE string;
                DEFINE FIELD created_at ON TABLE comment TYPE string;
                ",
            )
            .await?;
        Ok(())
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
            .query("CREATE user SET username = $username, email = $email, password_hash = $password_hash, created_at = $now")
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

    // ── Post ──────────────────────────────────────────────

    pub async fn create_post(
        &self,
        author_id: &str,
        author_username: &str,
        title: &str,
        content: &str,
    ) -> Result<PostRecord> {
        let now = chrono::Utc::now().to_rfc3339();
        let mut result = self
            .client
            .query("CREATE post SET title = $title, content = $content, author_id = $author_id, author_username = $author_username, created_at = $now, updated_at = $now")
            .bind(("title", title))
            .bind(("content", content))
            .bind(("author_id", author_id))
            .bind(("author_username", author_username))
            .bind(("now", &now))
            .await?;
        let post: Option<PostRecord> = result.take(0)?;
        post.ok_or_else(|| anyhow::anyhow!("포스트 생성 실패"))
    }

    pub async fn get_post(&self, id: &str) -> Result<Option<PostRecord>> {
        let record: Option<PostRecord> = self.client.select(("post", id)).await?;
        Ok(record)
    }

    pub async fn list_posts(&self, page: u32, per_page: u32) -> Result<(Vec<PostRecord>, u32)> {
        let offset = page.saturating_sub(1) * per_page;
        let mut result = self
            .client
            .query("SELECT * FROM post ORDER BY created_at DESC LIMIT $limit START $offset")
            .bind(("limit", per_page))
            .bind(("offset", offset))
            .await?;
        let posts: Vec<PostRecord> = result.take(0)?;

        let mut count_result = self
            .client
            .query("SELECT count() AS count FROM post GROUP ALL")
            .await?;
        let counts: Vec<CountResult> = count_result.take(0)?;
        let total = counts.first().map(|c| c.count as u32).unwrap_or(0);

        Ok((posts, total))
    }

    pub async fn update_post(
        &self,
        id: &str,
        author_id: &str,
        title: &str,
        content: &str,
    ) -> Result<Option<PostRecord>> {
        let post = self.get_post(id).await?;
        match post {
            Some(p) if p.author_id == author_id => {}
            Some(_) => return Err(anyhow::anyhow!("권한이 없습니다.")),
            None => return Ok(None),
        }

        let now = chrono::Utc::now().to_rfc3339();
        let mut result = self
            .client
            .query("UPDATE type::thing('post', $id) SET title = $title, content = $content, updated_at = $now RETURN AFTER")
            .bind(("id", id))
            .bind(("title", title))
            .bind(("content", content))
            .bind(("now", &now))
            .await?;
        let posts: Vec<PostRecord> = result.take(0)?;
        Ok(posts.into_iter().next())
    }

    pub async fn delete_post(&self, id: &str, author_id: &str) -> Result<bool> {
        let post = self.get_post(id).await?;
        match post {
            Some(p) if p.author_id == author_id => {}
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

    // ── Comment ───────────────────────────────────────────

    pub async fn create_comment(
        &self,
        post_id: &str,
        author_id: &str,
        author_username: &str,
        content: &str,
    ) -> Result<CommentRecord> {
        let now = chrono::Utc::now().to_rfc3339();
        let mut result = self
            .client
            .query("CREATE comment SET content = $content, post_id = $post_id, author_id = $author_id, author_username = $author_username, created_at = $now")
            .bind(("content", content))
            .bind(("post_id", post_id))
            .bind(("author_id", author_id))
            .bind(("author_username", author_username))
            .bind(("now", &now))
            .await?;
        let comment: Option<CommentRecord> = result.take(0)?;
        comment.ok_or_else(|| anyhow::anyhow!("댓글 생성 실패"))
    }

    pub async fn list_comments(&self, post_id: &str) -> Result<Vec<CommentRecord>> {
        let mut result = self
            .client
            .query(
                "SELECT * FROM comment WHERE post_id = $post_id ORDER BY created_at ASC",
            )
            .bind(("post_id", post_id))
            .await?;
        let comments: Vec<CommentRecord> = result.take(0)?;
        Ok(comments)
    }

    pub async fn delete_comment(&self, id: &str, author_id: &str) -> Result<bool> {
        let record: Option<CommentRecord> = self.client.select(("comment", id)).await?;
        match record {
            Some(c) if c.author_id == author_id => {}
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
