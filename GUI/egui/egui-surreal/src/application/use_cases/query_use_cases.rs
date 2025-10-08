use crate::domain::QueryRepository;
use anyhow::Result;
use std::sync::Arc;

pub struct QueryUseCases {
    repository: Arc<dyn QueryRepository + Send + Sync>,
}

impl QueryUseCases {
    pub fn new(repository: Arc<dyn QueryRepository + Send + Sync>) -> Self {
        Self {
            repository,
        }
    }

    pub async fn execute_query(&self, query: String) -> Result<String> { self.repository.execute_raw_query(query).await }
}
