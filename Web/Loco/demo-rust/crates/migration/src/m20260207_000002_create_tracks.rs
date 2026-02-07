// [REQ-F001] tracks 테이블 마이그레이션 (2026-02-07)
use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "tracks",
            &[
                ("id", ColType::PkAuto),
                ("user_id", ColType::Integer),
                ("title", ColType::String),
                ("artist", ColType::StringNull),
                ("url", ColType::String),
                ("description", ColType::TextNull),
                ("is_public", ColType::BooleanWithDefault(false)),
                ("vote_score", ColType::IntegerWithDefault(0)),
            ],
            &[("users", "")],
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "tracks").await?;
        Ok(())
    }
}
