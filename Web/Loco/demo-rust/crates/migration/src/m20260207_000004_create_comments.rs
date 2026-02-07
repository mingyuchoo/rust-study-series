// [REQ-F003] comments 테이블 마이그레이션 (2026-02-07)
use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "comments",
            &[
                ("id", ColType::PkAuto),
                ("track_id", ColType::Integer),
                ("user_id", ColType::Integer),
                ("content", ColType::Text),
            ],
            &[("tracks", ""), ("users", "")],
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "comments").await?;
        Ok(())
    }
}
