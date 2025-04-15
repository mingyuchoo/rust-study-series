// infrastructure/db/doc_db_controller.rs - REST API 구현 (예시)
//

use crate::application::services::doc_application_service::DocApplicationService;
use crate::domain::services::repositories::doc_repository::DocRepository;

pub struct DocDbController<R: DocRepository> {
    application_service: DocApplicationService<R>,
}

impl DocDbController<crate::infrastructure::db::repositories::doc_db_repository::DocDbRepository> {
    pub fn new_with_db_path(db_path: &str) -> Result<Self, String> {
        let repo = crate::infrastructure::db::repositories::doc_db_repository::DocDbRepository::new(db_path)?;
        Ok(DocDbController::new_with_repository(repo))
    }
}

impl<R: DocRepository> DocDbController<R> {
    pub fn delete_doc(&self, id: &str) -> Result<String, String> {
        self.application_service.delete_doc(id)?;
        Ok(format!("Doc {} deleted", id))
    }

    pub fn new(application_service: DocApplicationService<R>) -> Self {
        Self {
            application_service,
        }
    }

    pub fn new_with_repository(repository: R) -> Self {
        let doc_service = crate::domain::services::doc_service::DocService::new(repository);
        Self::new(DocApplicationService::new(doc_service))
    }

    pub fn register_doc(&self, title: String, contents: String) -> Result<String, String> {
        match self.application_service.register_doc(title, contents) {
            | Ok(doc_dto) => Ok(format!("Doc created: {}", doc_dto.title)),
            | Err(e) => Err(format!("Failed to create doc: {}", e)),
        }
    }

    pub fn get_doc(&self, id: &str) -> Result<String, String> {
        let doc = self
            .application_service
            .get_doc_details(id)
            .ok_or_else(|| format!("Doc with id {} not found", id))?;
        Ok(format!("Doc: {}, Contents: {}, Archived: {}", doc.title, doc.contents, doc.archived))
    }

    pub fn deactivate_doc(&self, id: &str) -> Result<String, String> {
        match self.application_service.deactivate_doc(id) {
            | Ok(_) => Ok(format!("Doc {} deactivated", id)),
            | Err(e) => Err(e),
        }
    }

    pub fn list_all_docs(&self) -> Result<String, String> {
        let docs = self.application_service.list_all_docs()?;
        if docs.is_empty() {
            return Ok("No docs found".to_string());
        }

        let mut result = String::from("Docs:\n");
        for doc in docs {
            result.push_str(&format!(
                "- {} ({}): {}\n",
                doc.title,
                doc.contents,
                if doc.archived { "archived" } else { "inactive" }
            ));
        }

        Ok(result)
    }

    pub fn list_all_docs_json(&self) -> Result<Vec<crate::application::services::doc_application_service::DocDto>, String> {
        self.application_service.list_all_docs()
    }

    pub fn get_doc_details_json(&self, id: &str) -> Option<crate::application::services::doc_application_service::DocDto> {
        self.application_service.get_doc_details(id)
    }
}
