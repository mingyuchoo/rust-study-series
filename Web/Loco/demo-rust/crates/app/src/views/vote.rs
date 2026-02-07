// [REQ-F002] Vote 뷰 응답 구조체 (2026-02-07)
use serde::{Deserialize, Serialize};

use crate::models::_entities::votes;

#[derive(Debug, Deserialize, Serialize)]
pub struct VoteResponse {
    pub id: i32,
    pub track_id: i32,
    pub user_id: i32,
    pub vote_type: i32,
}

impl VoteResponse {
    #[must_use]
    pub fn new(vote: &votes::Model) -> Self {
        Self {
            id: vote.id,
            track_id: vote.track_id,
            user_id: vote.user_id,
            vote_type: vote.vote_type,
        }
    }
}
