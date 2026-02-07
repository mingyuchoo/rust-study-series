// [REQ-F002] Vote 모델 확장 — 투표 + vote_score 동기 갱신 (2026-02-07)
use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, QueryFilter, TransactionTrait};

pub use super::_entities::votes::{self, ActiveModel, Entity, Model};
use super::_entities::tracks;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct VoteParams {
    pub vote_type: i32,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for super::_entities::votes::ActiveModel {}

impl Model {
    /// 특정 트랙에 대한 사용자의 투표를 조회한다
    pub async fn find_by_track_and_user(
        db: &DatabaseConnection,
        track_id: i32,
        user_id: i32,
    ) -> ModelResult<Option<Self>> {
        let vote = votes::Entity::find()
            .filter(votes::Column::TrackId.eq(track_id))
            .filter(votes::Column::UserId.eq(user_id))
            .one(db)
            .await?;
        Ok(vote)
    }

    /// 투표한다 (신규 투표 또는 기존 투표 변경)
    /// vote_type: 1(upvote) 또는 -1(downvote)
    /// 트랜잭션으로 vote 레코드와 track.vote_score를 원자적으로 갱신한다.
    pub async fn vote(
        db: &DatabaseConnection,
        track_id: i32,
        user_id: i32,
        vote_type: i32,
    ) -> ModelResult<Self> {
        let txn = db.begin().await?;

        let existing = votes::Entity::find()
            .filter(votes::Column::TrackId.eq(track_id))
            .filter(votes::Column::UserId.eq(user_id))
            .one(&txn)
            .await?;

        let result = if let Some(existing_vote) = existing {
            let old_type = existing_vote.vote_type;
            if old_type == vote_type {
                // 동일한 투표 — 변경 없음
                txn.commit().await?;
                return Ok(existing_vote);
            }
            // 투표 변경: score 차이분 = new - old
            let diff = vote_type - old_type;
            let mut active: votes::ActiveModel = existing_vote.into();
            active.vote_type = ActiveValue::Set(vote_type);
            let updated = active.update(&txn).await?;

            Self::update_vote_score(&txn, track_id, diff).await?;
            updated
        } else {
            // 신규 투표
            let new_vote = votes::ActiveModel {
                track_id: ActiveValue::Set(track_id),
                user_id: ActiveValue::Set(user_id),
                vote_type: ActiveValue::Set(vote_type),
                ..Default::default()
            }
            .insert(&txn)
            .await?;

            Self::update_vote_score(&txn, track_id, vote_type).await?;
            new_vote
        };

        txn.commit().await?;
        Ok(result)
    }

    /// 투표를 취소한다
    pub async fn remove_vote(
        db: &DatabaseConnection,
        track_id: i32,
        user_id: i32,
    ) -> ModelResult<()> {
        let txn = db.begin().await?;

        let existing = votes::Entity::find()
            .filter(votes::Column::TrackId.eq(track_id))
            .filter(votes::Column::UserId.eq(user_id))
            .one(&txn)
            .await?;

        if let Some(vote) = existing {
            let old_type = vote.vote_type;
            let active: votes::ActiveModel = vote.into();
            active.delete(&txn).await?;

            Self::update_vote_score(&txn, track_id, -old_type).await?;
        }

        txn.commit().await?;
        Ok(())
    }

    /// track.vote_score를 delta만큼 증감한다
    async fn update_vote_score<C: ConnectionTrait>(
        db: &C,
        track_id: i32,
        delta: i32,
    ) -> ModelResult<()> {
        let track = tracks::Entity::find_by_id(track_id)
            .one(db)
            .await?
            .ok_or(ModelError::EntityNotFound)?;

        let mut active: tracks::ActiveModel = track.into();
        let current_score = active.vote_score.as_ref().clone();
        active.vote_score = ActiveValue::Set(current_score + delta);
        active.update(db).await?;
        Ok(())
    }
}
