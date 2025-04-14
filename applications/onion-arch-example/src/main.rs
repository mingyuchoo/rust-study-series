// main.rs - 애플리케이션 진입점
//

// Import the crate itself
use onion_arch_example::{// Application layer
                         application::services::user_application_service::UserApplicationService,
                         // Infrastructure layer
                         infrastructure::{api::UserApiController, api::repositories::SqliteUserRepository}};
use uuid::Uuid;

fn main() -> Result<(), String> {
    println!("Starting Onion Architecture Demo with SQLite");

    // 인프라스트럭처 계층 초기화 (SQLite 사용)
    let repository = SqliteUserRepository::new("users.db")?;

    // SQLite 연결 테스트
    repository.test_connection()?;
    println!("SQLite connection established successfully");

    // 애플리케이션 서비스 초기화
    let app_service = UserApplicationService::new(repository);

    // API 컨트롤러 초기화
    let api_controller = UserApiController::new(app_service);

    // 사용자 등록
    let user_id = Uuid::new_v4().to_string();
    let result = api_controller.register_user(user_id.clone(), "john_doe".to_string(), "john@example.com".to_string());
    println!("{:?}", result);

    // 두 번째 사용자 등록
    let user_id2 = Uuid::new_v4().to_string();
    let result = api_controller.register_user(user_id2.clone(), "jane_smith".to_string(), "jane@example.com".to_string());
    println!("{:?}", result);

    // 사용자 조회
    let user_info = api_controller.get_user(&user_id);
    println!("{:?}", user_info);

    // 사용자 비활성화
    let deactivate_result = api_controller.deactivate_user(&user_id);
    println!("{:?}", deactivate_result);

    // 비활성화 후 사용자 조회
    let user_info_after = api_controller.get_user(&user_id);
    println!("{:?}", user_info_after);

    // 모든 사용자 목록
    let all_users = api_controller.list_all_users();
    println!("{}", all_users.unwrap_or_else(|e| format!("Error: {}", e)));

    println!("Demo completed successfully");
    Ok(())
}

// 통합 테스트 예시
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_sqlite_repository() -> Result<(), String> {
        // 테스트용 임시 DB 파일
        let test_db = "test_users.db";

        // 테스트 시작 전 이전 테스트 DB 삭제
        let _ = fs::remove_file(test_db);

        // 저장소 생성
        let repo = SqliteUserRepository::new(test_db)?;

        // 사용자 생성
        let user = User::new("test-123".to_string(), "testuser".to_string(), "test@example.com".to_string());

        // 저장 및 조회 테스트
        repo.save(&user)?;
        let found_user = repo.find_by_id("test-123")?;
        assert!(found_user.is_some());

        let found_user = found_user.unwrap();
        assert_eq!(found_user.username, "testuser");
        assert_eq!(found_user.email, "test@example.com");
        assert_eq!(found_user.active, true);

        // 사용자 목록 테스트
        let all_users = repo.find_all()?;
        assert_eq!(all_users.len(), 1);

        // 사용자 삭제 테스트
        repo.delete("test-123")?;
        let not_found = repo.find_by_id("test-123")?;
        assert!(not_found.is_none());

        // 테스트 종료 후 테스트 DB 삭제
        let _ = fs::remove_file(test_db);

        Ok(())
    }

    #[test]
    fn test_user_service() -> Result<(), String> {
        // 테스트용 임시 DB 파일
        let test_db = "test_service.db";
        let _ = fs::remove_file(test_db);

        // 저장소 및 서비스 생성
        let repo = SqliteUserRepository::new(test_db)?;
        let service = UserService::new(repo);

        // 사용자 생성 테스트
        let user = service.create_user("test-456".to_string(), "servicetest".to_string(), "service@example.com".to_string())?;

        assert_eq!(user.username, "servicetest");
        assert!(user.active);

        // 사용자 비활성화 테스트
        let deactivated = service.deactivate_user("test-456")?;
        assert!(!deactivated.active);

        // 테스트 종료 후 테스트 DB 삭제
        let _ = fs::remove_file(test_db);

        Ok(())
    }
}
