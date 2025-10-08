use crate::domain::{Contact, ContactRepository, DomainError};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

pub struct SqliteContactRepository {
    pool: SqlitePool,
}

impl SqliteContactRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
        }
    }

    pub async fn init(&self) -> Result<(), sqlx::Error> {
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
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[async_trait]
impl ContactRepository for SqliteContactRepository {
    async fn create(&self, contact: Contact) -> Result<Contact, DomainError> {
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
        .await?;

        Ok(contact)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Contact>, DomainError> {
        let row = sqlx::query("SELECT id, name, email, phone, address, created_at, updated_at FROM contacts WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;

        match row {
            | Some(row) => {
                let contact = Contact {
                    id: Uuid::parse_str(row.get("id")).map_err(|e| DomainError::DatabaseError(e.to_string()))?,
                    name: row.get("name"),
                    email: row.get("email"),
                    phone: row.get("phone"),
                    address: row.get("address"),
                    created_at: DateTime::parse_from_rfc3339(row.get("created_at"))
                        .map_err(|e| DomainError::DatabaseError(e.to_string()))?
                        .with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(row.get("updated_at"))
                        .map_err(|e| DomainError::DatabaseError(e.to_string()))?
                        .with_timezone(&Utc),
                };
                Ok(Some(contact))
            },
            | None => Ok(None),
        }
    }

    async fn get_all(&self) -> Result<Vec<Contact>, DomainError> {
        let rows = sqlx::query("SELECT id, name, email, phone, address, created_at, updated_at FROM contacts ORDER BY name")
            .fetch_all(&self.pool)
            .await?;

        let mut contacts = Vec::new();
        for row in rows {
            let contact = Contact {
                id: Uuid::parse_str(row.get("id")).map_err(|e| DomainError::DatabaseError(e.to_string()))?,
                name: row.get("name"),
                email: row.get("email"),
                phone: row.get("phone"),
                address: row.get("address"),
                created_at: DateTime::parse_from_rfc3339(row.get("created_at"))
                    .map_err(|e| DomainError::DatabaseError(e.to_string()))?
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(row.get("updated_at"))
                    .map_err(|e| DomainError::DatabaseError(e.to_string()))?
                    .with_timezone(&Utc),
            };
            contacts.push(contact);
        }

        Ok(contacts)
    }

    async fn update(&self, contact: Contact) -> Result<Contact, DomainError> {
        sqlx::query(
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
        .await?;

        Ok(contact)
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM contacts WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn search(&self, query: &str) -> Result<Vec<Contact>, DomainError> {
        let search_pattern = format!("%{}%", query);
        let rows = sqlx::query(
            r#"
            SELECT id, name, email, phone, address, created_at, updated_at 
            FROM contacts 
            WHERE name LIKE ? OR email LIKE ? OR phone LIKE ? OR address LIKE ?
            ORDER BY name
            "#,
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await?;

        let mut contacts = Vec::new();
        for row in rows {
            let contact = Contact {
                id: Uuid::parse_str(row.get("id")).map_err(|e| DomainError::DatabaseError(e.to_string()))?,
                name: row.get("name"),
                email: row.get("email"),
                phone: row.get("phone"),
                address: row.get("address"),
                created_at: DateTime::parse_from_rfc3339(row.get("created_at"))
                    .map_err(|e| DomainError::DatabaseError(e.to_string()))?
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(row.get("updated_at"))
                    .map_err(|e| DomainError::DatabaseError(e.to_string()))?
                    .with_timezone(&Utc),
            };
            contacts.push(contact);
        }

        Ok(contacts)
    }
}
