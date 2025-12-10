use crate::utils::multipart::upload;
use crate::models::entities::service;
use crate::models::entities::service::Entity as Service;
use crate::models::dtos::service::ServiceDto;
use axum::response::IntoResponse;
use sea_orm::entity::*;
use sea_orm::ActiveModelTrait;
use crate::db::conn::get_db;
use serde_json::json;
use axum::http::StatusCode;
use axum::response::Json;
use uuid::Uuid;
use chrono::Utc;

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
    Json(service_dto): Json<ServiceDto>
) -> impl IntoResponse {
    let db = get_db();
    
    let new_service = service::ActiveModel {
        id: Set(Uuid::new_v4()),
        vault_id: Set(service_dto.vault_id),
        image_url: Set(service_dto.image_url),
        name: Set(service_dto.name),
        created_at: Set(service_dto.created_at),
        updated_at: Set(service_dto.updated_at),
    };

    match new_service.insert(db).await {
        Ok(service) => {
            let response = ServiceDto {
                name: service.name,
                vault_id: service.vault_id,
                image_url: service.image_url,
                created_at: service.created_at,
                updated_at: service.updated_at,
            };
            (StatusCode::CREATED, Json(json!({ "data": response }))).into_response()
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to create service: {}", e) })),
        ).into_response(),
    }
}