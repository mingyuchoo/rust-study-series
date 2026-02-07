// [REQ-F001] Track 뷰 응답 구조체 (2026-02-07)
use serde::{Deserialize, Serialize};

use crate::models::_entities::tracks;

#[derive(Debug, Deserialize, Serialize)]
pub struct TrackResponse {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub artist: Option<String>,
    pub url: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub vote_score: i32,
    pub created_at: String,
    pub updated_at: String,
}

impl TrackResponse {
    #[must_use]
    pub fn new(track: &tracks::Model) -> Self {
        Self {
            id: track.id,
            user_id: track.user_id,
            title: track.title.clone(),
            artist: track.artist.clone(),
            url: track.url.clone(),
            description: track.description.clone(),
            is_public: track.is_public,
            vote_score: track.vote_score,
            created_at: track.created_at.to_string(),
            updated_at: track.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TrackListResponse {
    pub tracks: Vec<TrackResponse>,
}

impl TrackListResponse {
    #[must_use]
    pub fn new(tracks: &[tracks::Model]) -> Self {
        Self {
            tracks: tracks.iter().map(TrackResponse::new).collect(),
        }
    }
}
