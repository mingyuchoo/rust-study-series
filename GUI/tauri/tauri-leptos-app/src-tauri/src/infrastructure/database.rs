use crate::domain::entities::Contact;
use crate::domain::errors::{ContactError, ContactResult};
use crate::domain::repositories::ContactRepository;
use chrono::{DateTime, Utc};
use sqlx::Row;
use sqlx::sqlite::SqlitePool;
use std::path::Path;
use uuid::Uuid;

pub struct SqliteContactRepository {
    pool: SqlitePool,
}

impl SqliteContactRepository {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        // Create database file if it doesn't exist
        if let Some(parent) = Path::new(database_url.strip_prefix("sqlite:").unwrap_or(database_url)).parent() {
            tokio::fs::create_dir_all(parent).await.ok();
        }

        let pool = SqlitePool::connect(database_url).await?;

        // Create table if it doesn't exist
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS contacts (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                email TEXT,
                phone TEXT,
                address TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self {
            pool,
        })
    }
}

#[async_trait::async_trait]
impl ContactRepository for SqliteContactRepository {
    async fn create(&self, contact: Contact) -> ContactResult<Contact> {
        sqlx::query(
            r#"
            INSERT INTO contacts (id, name, email, phone, address, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(contact.id.to_string())
        .bind(&contact.name)
        .bind(&contact.email)
        .bind(&contact.phone)
        .bind(&contact.address)
        .bind(contact.created_at.to_rfc3339())
        .bind(contact.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| ContactError::DatabaseError {
            message: e.to_string(),
        })?;

        Ok(contact)
    }

    async fn get_by_id(&self, id: Uuid) -> ContactResult<Contact> {
        let row = sqlx::query("SELECT * FROM contacts WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ContactError::DatabaseError {
                message: e.to_string(),
            })?;

        match row {
            | Some(row) => {
                let contact = Contact {
                    id: Uuid::parse_str(&row.get::<String, _>("id")).unwrap(),
                    name: row.get("name"),
                    email: row.get("email"),
                    phone: row.get("phone"),
                    address: row.get("address"),
                    created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at")).unwrap().with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at")).unwrap().with_timezone(&Utc),
                };
                Ok(contact)
            },
            | None => Err(ContactError::NotFound {
                id,
            }),
        }
    }

    async fn get_all(&self) -> ContactResult<Vec<Contact>> {
        let rows = sqlx::query("SELECT * FROM contacts ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ContactError::DatabaseError {
                message: e.to_string(),
            })?;

        let contacts = rows
            .into_iter()
            .map(|row| Contact {
                id: Uuid::parse_str(&row.get::<String, _>("id")).unwrap(),
                name: row.get("name"),
                email: row.get("email"),
                phone: row.get("phone"),
                address: row.get("address"),
                created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at")).unwrap().with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at")).unwrap().with_timezone(&Utc),
            })
            .collect();

        Ok(contacts)
    }

    async fn update(&self, contact: Contact) -> ContactResult<Contact> {
        let result = sqlx::query(
            r#"
            UPDATE contacts 
            SET name = ?, email = ?, phone = ?, address = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&contact.name)
        .bind(&contact.email)
        .bind(&contact.phone)
        .bind(&contact.address)
        .bind(contact.updated_at.to_rfc3339())
        .bind(contact.id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| ContactError::DatabaseError {
            message: e.to_string(),
        })?;

        if result.rows_affected() == 0 {
            return Err(ContactError::NotFound {
                id: contact.id,
            });
        }

        Ok(contact)
    }

    async fn delete(&self, id: Uuid) -> ContactResult<()> {
        let result = sqlx::query("DELETE FROM contacts WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| ContactError::DatabaseError {
                message: e.to_string(),
            })?;

        if result.rows_affected() == 0 {
            return Err(ContactError::NotFound {
                id,
            });
        }

        Ok(())
    }

    async fn search(&self, query: &str) -> ContactResult<Vec<Contact>> {
        let search_pattern = format!("%{}%", query);
        let rows = sqlx::query(
            r#"
            SELECT * FROM contacts 
            WHERE name LIKE ? OR email LIKE ? OR phone LIKE ? OR address LIKE ?
            ORDER BY name
            "#,
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ContactError::DatabaseError {
            message: e.to_string(),
        })?;

        let contacts = rows
            .into_iter()
            .map(|row| Contact {
                id: Uuid::parse_str(&row.get::<String, _>("id")).unwrap(),
                name: row.get("name"),
                email: row.get("email"),
                phone: row.get("phone"),
                address: row.get("address"),
                created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at")).unwrap().with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at")).unwrap().with_timezone(&Utc),
            })
            .collect();

        Ok(contacts)
    }
}
