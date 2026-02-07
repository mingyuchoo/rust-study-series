// [REQ-N002] 시드 데이터 태스크 테스트 (2026-02-07)
use demo_app::app::App;
use demo_app::tasks::seed_data::SeedData;
use loco_rs::prelude::*;
use sea_orm::EntityTrait;
use serial_test::serial;

use demo_app::models::_entities::{comments, tracks, users};

#[tokio::test]
#[serial]
async fn can_seed_data() {
    let boot = boot_test::<App>()
        .await
        .expect("Failed to boot test application");

    let task = SeedData;
    let vars = task::Vars::default();
    task.run(&boot.app_context, &vars)
        .await
        .expect("Seed data task failed");

    // [REQ-N002] 생성된 데이터 수 검증
    let user_count = users::Entity::find().all(&boot.app_context.db).await.unwrap();
    let track_count = tracks::Entity::find().all(&boot.app_context.db).await.unwrap();
    let comment_count = comments::Entity::find().all(&boot.app_context.db).await.unwrap();

    assert_eq!(user_count.len(), 100, "Expected 100 users");
    assert_eq!(track_count.len(), 100, "Expected 100 tracks");
    assert_eq!(comment_count.len(), 100, "Expected 100 comments");
}

#[tokio::test]
#[serial]
async fn seed_data_tracks_are_public() {
    let boot = boot_test::<App>()
        .await
        .expect("Failed to boot test application");

    let task = SeedData;
    let vars = task::Vars::default();
    task.run(&boot.app_context, &vars)
        .await
        .expect("Seed data task failed");

    // [REQ-N002] 모든 트랙이 공개 상태인지 검증
    let all_tracks = tracks::Entity::find().all(&boot.app_context.db).await.unwrap();
    let public_count = all_tracks.iter().filter(|t| t.is_public).count();
    assert_eq!(public_count, 100, "All 100 tracks should be public");
}

#[tokio::test]
#[serial]
async fn seed_data_comments_on_valid_tracks() {
    let boot = boot_test::<App>()
        .await
        .expect("Failed to boot test application");

    let task = SeedData;
    let vars = task::Vars::default();
    task.run(&boot.app_context, &vars)
        .await
        .expect("Seed data task failed");

    // [REQ-N002] 모든 댓글이 존재하는 트랙에 연결되어 있는지 검증
    let all_tracks = tracks::Entity::find().all(&boot.app_context.db).await.unwrap();
    let track_ids: Vec<i32> = all_tracks.iter().map(|t| t.id).collect();

    let all_comments = comments::Entity::find().all(&boot.app_context.db).await.unwrap();
    for comment in &all_comments {
        assert!(
            track_ids.contains(&comment.track_id),
            "Comment {} references non-existent track {}",
            comment.id,
            comment.track_id
        );
    }
}
