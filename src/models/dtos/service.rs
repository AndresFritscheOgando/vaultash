use axum::{
    body::Bytes,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

use crate::models::{entities::service::Model};

#[derive(Debug, Error)]
pub enum ServiceRequestError {
    #[error("Missing field: {0}")]
    MissingField(String),
    #[error("Multipart error: {0}")]
    MultipartError(#[from] axum::extract::multipart::MultipartError),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateServiceRequest {
    pub name: String,
    pub vault_id: String,
}

impl CreateServiceRequest {
    pub async fn from_multipart(
        mut multipart: axum::extract::Multipart,
    ) -> Result<(Self, Option<Bytes>, String), ServiceRequestError> {
        let mut fields = HashMap::new();
        let mut image_data = None;
        let mut image_name = String::new();

        while let Some(field) = multipart.next_field().await? {
            let name = field.name().unwrap_or("").to_string();
            
            if name == "image" {
                if let Some(filename) = field.file_name() {
                    image_name = filename.to_string();
                    image_data = Some(field.bytes().await?);
                }
            } else {
                let value = field.text().await?;
                fields.insert(name, value);
            }
        }

        let name = fields.remove("name")
            .ok_or_else(|| ServiceRequestError::MissingField("name".to_string()))?;
            
        let vault_id = fields.remove("vault_id")
            .ok_or_else(|| ServiceRequestError::MissingField("vault_id".to_string()))?;

        Ok((
            Self { name, vault_id },
            image_data,
            image_name,
        ))
    }
}


// Output to client (GET response)
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceDto {
    pub name: String, 
    pub vault_id: String, 
    pub image_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Model> for ServiceDto {
    fn from(model: Model) -> Self {
        Self {
            name: model.name,
            vault_id: model.vault_id,
            image_url: model.image_url,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
