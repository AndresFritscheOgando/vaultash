use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::models::{entities::service::Model};


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
