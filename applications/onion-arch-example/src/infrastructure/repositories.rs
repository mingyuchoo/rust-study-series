//
// infrastructure/repositories.rs - 저장소 구현
//
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct InMemoryUserRepository {
    users: Arc<Mutex<HashMap<String, User>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl UserRepository for InMemoryUserRepository {
    fn find_by_id(&self, id: &str) -> Option<User> {
        let users = self.users.lock().unwrap();
        users.get(id).cloned()
    }

    fn save(&self, user: &User) -> Result<(), String> {
        let mut users = self.users.lock().unwrap();
        users.insert(user.id.clone(), user.clone());
        Ok(())
    }

    fn delete(&self, id: &str) -> Result<(), String> {
        let mut users = self.users.lock().unwrap();
        users.remove(id);
        Ok(())
    }

    fn find_all(&self) -> Vec<User> {
        let users = self.users.lock().unwrap();
        users.values().cloned().collect()
    }
}
