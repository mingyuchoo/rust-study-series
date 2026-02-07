// [REQ-F003] Comment 모델 확장 — Validator + CRUD (2026-02-07)
use loco_rs::prelude::*;
use sea_orm::{entity::prelude::*, QueryOrder};

pub use super::_entities::comments::{self, ActiveModel, Entity, Model};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateCommentParams {
    pub content: String,
}

#[derive(Debug, Validate, serde::Deserialize)]
pub struct Validator {
    #[validate(length(min = 1, message = "Comment content must not be empty."))]
    pub content: String,
}

impl Validatable for ActiveModel {
    fn validator(&self) -> Box<dyn Validate> {
        Box::new(Validator {
            content: self.content.as_ref().to_owned(),
        })
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for super::_entities::comments::ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.validate()?;
        Ok(self)
    }
}

impl Model {
    /// 특정 트랙의 댓글 목록을 조회한다 (생성일 내림차순)
    pub async fn find_by_track(db: &DatabaseConnection, track_id: i32) -> ModelResult<Vec<Self>> {
        let items = comments::Entity::find()
            .filter(comments::Column::TrackId.eq(track_id))
            .order_by_desc(comments::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(items)
    }

    /// 댓글을 생성한다
    pub async fn create(
        db: &DatabaseConnection,
        track_id: i32,
        user_id: i32,
        params: &CreateCommentParams,
    ) -> ModelResult<Self> {
        let comment = comments::ActiveModel {
            track_id: ActiveValue::Set(track_id),
            user_id: ActiveValue::Set(user_id),
            content: ActiveValue::Set(params.content.clone()),
            ..Default::default()
        }
        .insert(db)
        .await?;
        Ok(comment)
    }

    /// ID로 댓글을 조회한다
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        let comment = comments::Entity::find_by_id(id).one(db).await?;
        comment.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// 댓글을 삭제한다
    pub async fn remove(db: &DatabaseConnection, id: i32) -> ModelResult<()> {
        let comment = Self::find_by_id(db, id).await?;
        comment.into_active_model().delete(db).await?;
        Ok(())
    }
}
