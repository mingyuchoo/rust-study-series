// [REQ-F001] Track 모델 확장 — Validator + ActiveRecord (2026-02-07)
use loco_rs::prelude::*;
use sea_orm::{entity::prelude::*, QueryOrder};
use serde::{Deserialize, Serialize};

pub use super::_entities::tracks::{self, ActiveModel, Entity, Model};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTrackParams {
    pub title: String,
    pub artist: Option<String>,
    pub url: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTrackParams {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct Validator {
    #[validate(length(min = 1, message = "Title must not be empty."))]
    pub title: String,
    #[validate(url(message = "Invalid URL format."))]
    pub url: String,
}

impl Validatable for ActiveModel {
    fn validator(&self) -> Box<dyn Validate> {
        Box::new(Validator {
            title: self.title.as_ref().to_owned(),
            url: self.url.as_ref().to_owned(),
        })
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for super::_entities::tracks::ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.validate()?;
        if insert {
            let mut this = self;
            if this.is_public.is_not_set() {
                this.is_public = ActiveValue::Set(false);
            }
            if this.vote_score.is_not_set() {
                this.vote_score = ActiveValue::Set(0);
            }
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

impl Model {
    /// 공개된 트랙 목록을 조회한다 (vote_score 내림차순)
    pub async fn find_public(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        let tracks = tracks::Entity::find()
            .filter(tracks::Column::IsPublic.eq(true))
            .order_by_desc(tracks::Column::VoteScore)
            .all(db)
            .await?;
        Ok(tracks)
    }

    /// 특정 사용자의 트랙 목록을 조회한다
    pub async fn find_by_user(db: &DatabaseConnection, user_id: i32) -> ModelResult<Vec<Self>> {
        let tracks = tracks::Entity::find()
            .filter(tracks::Column::UserId.eq(user_id))
            .order_by_desc(tracks::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(tracks)
    }

    /// ID로 트랙을 조회한다
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        let track = tracks::Entity::find_by_id(id).one(db).await?;
        track.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// 트랙을 생성한다
    pub async fn create(
        db: &DatabaseConnection,
        user_id: i32,
        params: &CreateTrackParams,
    ) -> ModelResult<Self> {
        let track = tracks::ActiveModel {
            user_id: ActiveValue::Set(user_id),
            title: ActiveValue::Set(params.title.clone()),
            artist: ActiveValue::Set(params.artist.clone()),
            url: ActiveValue::Set(params.url.clone()),
            description: ActiveValue::Set(params.description.clone()),
            ..Default::default()
        }
        .insert(db)
        .await?;
        Ok(track)
    }
}

impl ActiveModel {
    /// 트랙 정보를 수정한다
    pub async fn update_track(
        mut self,
        db: &DatabaseConnection,
        params: &UpdateTrackParams,
    ) -> ModelResult<Model> {
        if let Some(title) = &params.title {
            self.title = ActiveValue::Set(title.clone());
        }
        if let Some(artist) = &params.artist {
            self.artist = ActiveValue::Set(Some(artist.clone()));
        }
        if let Some(url) = &params.url {
            self.url = ActiveValue::Set(url.clone());
        }
        if let Some(description) = &params.description {
            self.description = ActiveValue::Set(Some(description.clone()));
        }
        self.update(db).await.map_err(ModelError::from)
    }

    /// 공개/비공개 상태를 전환한다
    pub async fn toggle_public(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        let current = self.is_public.as_ref().clone();
        self.is_public = ActiveValue::Set(!current);
        self.update(db).await.map_err(ModelError::from)
    }
}
