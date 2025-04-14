// main.rs - 애플리케이션 진입점
//

// Import the crate itself
use onion_arch_example::infrastructure::api::user_api_controller::UserApiController;

fn main() -> Result<(), String> {
    // SQLite DB 파일 경로
    let db_path = "users.db";

    // 컨트롤러 생성 (db_path만 넘김)
    let controller = UserApiController::new_with_db_path(db_path)?;

    // 1. CREATE
    println!(
        "{}",
        controller.register_user("1".to_string(), "alice".to_string(), "alice@email.com".to_string())?
    );
    println!("{}", controller.register_user("2".to_string(), "bob".to_string(), "bob@email.com".to_string())?);

    // 2. READ (전체)
    println!("{}", controller.list_all_users()?);
    // 3. READ (단일)
    println!("{}", controller.get_user("1")?);

    // 4. UPDATE (비활성화)
    println!("{}", controller.deactivate_user("1")?);
    println!("{}", controller.get_user("1")?);

    // 5. DELETE
    println!("{}", controller.delete_user("2")?);
    println!("{}", controller.list_all_users()?);

    Ok(())
}
