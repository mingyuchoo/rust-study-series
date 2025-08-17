use crate::domain::services::doc_service::DocService;
use crate::domain::services::repositories::doc_repository::DocRepository;
use crate::domain::services::repositories::entities::doc::{Doc, DocForm};

pub struct DocApplicationService<R: DocRepository> {
    doc_service: DocService<R>,
}

impl<R: DocRepository> DocApplicationService<R> {
    pub fn new(doc_service: DocService<R>) -> Self {
        Self {
            doc_service,
        }
    }

    // Original methods
    pub async fn create(&self, doc_form: DocForm) -> Result<Doc, Box<dyn std::error::Error>> { self.doc_service.create(doc_form).await }

    pub async fn update(&self, id: i32, doc_form: DocForm) -> Result<Doc, Box<dyn std::error::Error>> { self.doc_service.update(id, doc_form).await }

    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> { self.doc_service.delete(id).await }

    pub async fn find_by_id(&self, id: i32) -> Result<Doc, Box<dyn std::error::Error>> { self.doc_service.find_by_id(id).await }

    pub async fn find_all(&self) -> Result<Vec<Doc>, Box<dyn std::error::Error>> { self.doc_service.find_all().await }

    // Additional methods called from the controller
    pub fn delete_doc(&self, id: &str) -> Result<(), String> {
        // Convert string id to i32
        let id_int = id.parse::<i32>().map_err(|e| format!("Invalid ID format: {}", e))?;

        // Call the async delete method with a block_on
        futures::executor::block_on(self.delete(id_int)).map_err(|e| format!("Failed to delete doc: {}", e))
    }

    pub fn register_doc(&self, title: String, contents: String) -> Result<DocDto, String> {
        let doc_form = DocForm {
            title,
            contents,
            archived: false,
        };

        let doc = futures::executor::block_on(self.create(doc_form)).map_err(|e| format!("Failed to create doc: {}", e))?;

        Ok(DocDto::from(doc))
    }

    pub fn get_doc_details(&self, id: &str) -> Option<DocDto> {
        let id_int = match id.parse::<i32>() {
            | Ok(id) => id,
            | Err(_) => return None,
        };

        match futures::executor::block_on(self.find_by_id(id_int)) {
            | Ok(doc) => Some(DocDto::from(doc)),
            | Err(_) => None,
        }
    }

    pub fn deactivate_doc(&self, id: &str) -> Result<(), String> {
        let id_int = id.parse::<i32>().map_err(|e| format!("Invalid ID format: {}", e))?;

        // Get the doc first
        let doc = futures::executor::block_on(self.find_by_id(id_int)).map_err(|e| format!("Failed to find doc: {}", e))?;

        // Update with archived set to true
        let doc_form = DocForm {
            title: doc.title,
            contents: doc.contents,
            archived: true,
        };

        futures::executor::block_on(self.update(id_int, doc_form)).map_err(|e| format!("Failed to update doc: {}", e))?;

        Ok(())
    }

    pub fn list_all_docs(&self) -> Result<Vec<DocDto>, String> {
        let docs = futures::executor::block_on(self.find_all()).map_err(|e| format!("Failed to list docs: {}", e))?;

        Ok(docs.into_iter().map(DocDto::from).collect())
    }
}

#[derive(Clone, Debug)]
pub struct DocDto {
    pub id: i32,
    pub title: String,
    pub contents: String,
    pub archived: bool,
}
impl From<Doc> for DocDto {
    fn from(doc: Doc) -> Self {
        Self {
            id: doc.id,
            title: doc.title,
            contents: doc.contents,
            archived: doc.archived,
        }
    }
}
