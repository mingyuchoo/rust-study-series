use crate::domain::services::repositories::doc_repository::DocRepository;
use crate::domain::services::repositories::entities::doc::{Doc, DocForm};

pub struct DocService<R: DocRepository> {
    repository: R,
}

impl<R: DocRepository> DocService<R> {
    pub fn new(repository: R) -> Self {
        Self {
            repository,
        }
    }

    pub async fn create(&self, doc_form: DocForm) -> Result<Doc, Box<dyn std::error::Error>> { self.repository.create(doc_form).await }

    pub async fn update(&self, id: i32, doc_form: DocForm) -> Result<Doc, Box<dyn std::error::Error>> { self.repository.update(id, doc_form).await }

    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> { self.repository.delete(id).await }

    pub async fn find_by_id(&self, id: i32) -> Result<Doc, Box<dyn std::error::Error>> { self.repository.fetch_by_id(id).await }

    pub async fn find_all(&self) -> Result<Vec<Doc>, Box<dyn std::error::Error>> { self.repository.fetch_all().await }
}
