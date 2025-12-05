use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::models::entities::vault::Model;

// Input from client (POST/PUT)
#[derive(Debug, Deserialize)]
pub struct VaultInputDto {
    pub service_name: String,
    pub service_user_name: String,
}

// Output to client (GET response)
#[derive(Debug, Serialize)]
pub struct VaultOutputDto {
    pub id: Uuid,
    pub service_name: String, 
    pub service_user_name: String,
    pub service_password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Model> for VaultInputDto {
    fn from(model: Model) -> Self {
        Self {
            service_name: model.service_name,
            service_user_name: model.service_user_name
        }
    }
}

impl From<Model> for VaultOutputDto {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            service_name: model.service_name,
            service_user_name: model.service_user_name,
            service_password: model.service_password,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
