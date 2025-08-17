use crate::domain::services::repositories::entities::doc::{Doc, DocForm};
use std::error::Error;

#[async_trait::async_trait(?Send)]
pub trait DocRepository {
    async fn fetch_all(&self) -> Result<Vec<Doc>, Box<dyn Error>>;
    #[allow(dead_code)]
    async fn fetch_by_id(&self, id: i32) -> Result<Doc, Box<dyn Error>>;
    async fn create(&self, doc: DocForm) -> Result<Doc, Box<dyn Error>>;
    async fn update(&self, id: i32, doc: DocForm) -> Result<Doc, Box<dyn Error>>;
    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>>;
}
