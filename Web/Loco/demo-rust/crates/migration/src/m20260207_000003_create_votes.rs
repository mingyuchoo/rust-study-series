// [REQ-F002] votes 테이블 마이그레이션 (2026-02-07)
use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "votes",
            &[
                ("id", ColType::PkAuto),
                ("track_id", ColType::Integer),
                ("user_id", ColType::Integer),
                ("vote_type", ColType::Integer),
            ],
            &[("tracks", ""), ("users", "")],
        )
        .await?;

        // UNIQUE(track_id, user_id) — 사용자당 트랙 1표
        m.create_index(
            Index::create()
                .name("idx_votes_track_user")
                .table(Alias::new("votes"))
                .col(Alias::new("track_id"))
                .col(Alias::new("user_id"))
                .unique()
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "votes").await?;
        Ok(())
    }
}
