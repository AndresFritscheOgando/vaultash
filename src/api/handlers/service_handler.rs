use crate::models::entities::service;
use crate::models::entities::service::Entity as Service;
use crate::models::dtos::service::{CreateServiceRequest, ServiceDto};
use crate::api::handlers::aws_handler::{upload_image_file, ImageUploadError};
use axum::{
    response::IntoResponse,
    extract::Multipart,
    http::StatusCode,
    response::Json,
};
use sea_orm::entity::*;
use sea_orm::ActiveModelTrait;
use crate::db::conn::get_db;
use serde_json::json;
use uuid::Uuid;
use chrono::Utc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Failed to process request: {0}")]
    ProcessingError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    
    #[error("Image upload error: {0}")]
    ImageUploadError(#[from] ImageUploadError),
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ServiceError::ProcessingError(msg) => (StatusCode::BAD_REQUEST, msg),
            ServiceError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database operation failed".to_string(),
            ),
            ServiceError::ImageUploadError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to upload image: {}", e),
            ),
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}

pub async fn get_all_services_async() -> impl IntoResponse {
    let service = Service::find().all(get_db()).await;
    
    match service {
        Ok(services) => Json(json!({ "data": services })).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "No services found",
        )
            .into_response(),
    }
}

pub async fn create_service_async(
    multipart: Multipart,
) -> Result<impl IntoResponse, ServiceError> {
    let db = get_db();
    let now = Utc::now();
    
    // Parse multipart form data
    let (create_request, image_data, image_name) = CreateServiceRequest::from_multipart(multipart)
        .await
        .map_err(|e| ServiceError::ProcessingError(e.to_string()))?;

    // Handle image upload if present
    let image_url = if let Some(image_data) = image_data {
        // Upload image to S3
        upload_image_file(image_data.to_vec(), &image_name, "services")
            .await
            .map_err(ServiceError::ImageUploadError)?
    } else {
        // Use a default image URL or handle as needed
        "".to_string()
    };

    // Create new service
    let new_service = service::ActiveModel {
        id: Set(Uuid::new_v4()),
        vault_id: Set(create_request.vault_id),
        image_url: Set(image_url),
        name: Set(create_request.name),
        created_at: Set(now),
        updated_at: Set(now),
    };

    // Save to database
    let service = new_service.insert(db).await?;
    
    // Prepare response
    let response = ServiceDto {
        name: service.name,
        vault_id: service.vault_id,
        image_url: service.image_url,
        created_at: service.created_at,
        updated_at: service.updated_at,
    };

    Ok((StatusCode::CREATED, Json(json!({ "data": response }))))
}