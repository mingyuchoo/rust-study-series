// domain/services/repositories/mod.rs - 저장소 인터페이스 (내부 계층이 정의)
//

use crate::domain::services::repositories::models::user::User;

pub trait UserRepository {
    fn find_by_id(&self, id: &str) -> Option<User>;
    fn save(&self, user: &User) -> Result<(), String>;
    fn delete(&self, id: &str) -> Result<(), String>;
    fn find_all(&self) -> Vec<User>;
}
