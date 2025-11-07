// domain/models.rs - 도메인 엔티티 (가장 내부 계층)

#[derive(Clone, Debug)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub email: String,
    pub active: bool,
}

impl User {
    pub fn new(username: String, email: String) -> Self {
        Self {
            id: None,
            username,
            email,
            active: true,
        }
    }

    pub fn deactivate(&mut self) { self.active = false; }

    pub fn activate(&mut self) { self.active = true; }
}
