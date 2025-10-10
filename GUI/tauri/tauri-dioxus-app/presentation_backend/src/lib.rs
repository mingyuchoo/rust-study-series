mod models;
mod routes;

use application::*;
use infrastructure::SqliteContactRepository;
use routes::*;
use sqlx::SqlitePool;
use std::sync::Arc;
use tauri::Manager;

async fn setup_database() -> Result<SqlitePool, Box<dyn std::error::Error>> {
    let database_url = "sqlite::memory:";
    let pool = SqlitePool::connect(database_url).await?;

    let repository = SqliteContactRepository::new(pool.clone());
    repository.init().await?;

    Ok(pool)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                match setup_database().await {
                    Ok(pool) => {
                        let repository = Arc::new(SqliteContactRepository::new(pool));

                        let app_state = AppState {
                            create_contact_use_case: Arc::new(CreateContactUseCase::new(
                                repository.clone(),
                            )),
                            get_contact_use_case: Arc::new(GetContactUseCase::new(
                                repository.clone(),
                            )),
                            list_contacts_use_case: Arc::new(ListContactsUseCase::new(
                                repository.clone(),
                            )),
                            update_contact_use_case: Arc::new(UpdateContactUseCase::new(
                                repository.clone(),
                            )),
                            delete_contact_use_case: Arc::new(DeleteContactUseCase::new(
                                repository.clone(),
                            )),
                            search_contacts_use_case: Arc::new(SearchContactsUseCase::new(
                                repository,
                            )),
                        };

                        handle.manage(app_state);
                    }
                    Err(e) => {
                        eprintln!("Failed to setup database: {}", e);
                        std::process::exit(1);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_contact,
            get_contact,
            list_contacts,
            update_contact,
            delete_contact,
            search_contacts
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
