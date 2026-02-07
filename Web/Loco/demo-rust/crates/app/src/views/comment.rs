// [REQ-F003] Comment 뷰 응답 구조체 (2026-02-07)
use serde::{Deserialize, Serialize};

use crate::models::_entities::comments;

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentResponse {
    pub id: i32,
    pub track_id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: String,
}

impl CommentResponse {
    #[must_use]
    pub fn new(comment: &comments::Model) -> Self {
        Self {
            id: comment.id,
            track_id: comment.track_id,
            user_id: comment.user_id,
            content: comment.content.clone(),
            created_at: comment.created_at.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentListResponse {
    pub comments: Vec<CommentResponse>,
}

impl CommentListResponse {
    #[must_use]
    pub fn new(comments: &[comments::Model]) -> Self {
        Self {
            comments: comments.iter().map(CommentResponse::new).collect(),
        }
    }
}
