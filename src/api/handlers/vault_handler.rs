use crate::db::init_db;
use crate::models::entities::vault::{self, Entity as Vault};
use crate::models::dtos::vault::VaultInputDto;

use crate::db::conn::get_db;
use axum::extract::Path;
use chrono::Utc;

use axum::{
    response::{IntoResponse, Json},
    http::StatusCode,
};

use sea_orm::{ActiveModelTrait, EntityTrait, Set};


use uuid::Uuid;
use serde_json::json;

use crate::utils::index::generate_password;



// -------------------------------------------------
// GET ALL
// -------------------------------------------------
pub async fn get_all_async() -> impl IntoResponse {
    match Vault::find().all(get_db()).await {
        Ok(vaults) => Json(json!({ "data": vaults })).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "No vaults found",
        )
            .into_response(),
    }
}


// -------------------------------------------------
// GET Get by Id
// -------------------------------------------------


pub async fn get_by_id_async(
    Path(id): Path<Uuid>
) -> impl IntoResponse {
    
    let db = get_db();

    let vault_model = match Vault::find_by_id(id).one(db).await {
        
        Ok(Some(v)) => v,

        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "success": false, 
                    "error": "Vault not found" 
                })),
            ).into_response();
        }

        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,

                Json(json!({
                    "success": false, 
                    "error": format!("Database error: {}", e) 
                })),
            ).into_response();
        }
    };
    (
        StatusCode::OK, 
        Json(json!({ 
            "success": true, 
            "data": vault_model 
        }))
    ).into_response()
}


// -------------------------------------------------
// CREATE
// -------------------------------------------------
pub async fn create_async(
    Json(input): Json<VaultInputDto>,
) -> impl IntoResponse {
    

    // Generate password
    let password_json = generate_password().await;
    let password = password_json.0["password"]
        .as_str()
        .unwrap_or("")
        .to_string();

    // Build ActiveModel
    let new_vault = vault::ActiveModel {
        id: Set(Uuid::new_v4()),
        service_name: Set(input.service_name),
        service_password: Set(password),
        service_user_name: Set(input.service_user_name),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    // Insert
    match new_vault.insert(get_db()).await {
        Ok(vault) => (StatusCode::CREATED, Json(json!({ "data": vault }))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create vault: {}", e),
        )
            .into_response(),
    }
}
// -------------------------------------------------
// UPDATE
// -------------------------------------------------

pub async fn update_async(
    // 1. Path must be FIRST (non-body extractor)
    Path(id): Path<Uuid>, 
    // 2. Json must be LAST (body extractor)
    Json(model_data): Json<VaultInputDto>, // <-- Using the DTO
) -> impl IntoResponse {
    let db = get_db();

    // 1. Find existing vault (using 'id' from the path)
    let existing = match Vault::find_by_id(id).one(db).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                "Vault not found",
            ).into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            ).into_response();
        }
    };

    // 2. Convert Model -> ActiveModel
    let mut active: vault::ActiveModel = existing.into();

    // 3. Update fields (using 'model_data' from the body)
    active.service_name = Set(model_data.service_name.clone());
    active.service_user_name = Set(model_data.service_user_name.clone());
    active.updated_at = Set(Utc::now());

    // 4. Save
    let updated = match active.update(db).await {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to update vault: {}", e),
            ).into_response();
        }
    };

    // 5. Return result
    (StatusCode::OK, Json(json!({ "data": updated }))).into_response()
}

// -------------------------------------------------
// DELETE
// -------------------------------------------------

pub async fn delete_async(Path(id): Path<Uuid>) -> impl IntoResponse {
    let db = get_db();

    // 1. Try to find the existing record
    let existing = match Vault::find_by_id(id).one(db).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, "Vault not found").into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response();
        }
    };

    // 2. Convert Model → ActiveModel
    let active_model: vault::ActiveModel = existing.into();

    // 3. Delete record
    match active_model.delete(db).await {
        Ok(_) => (StatusCode::OK, "Vault deleted successfully").into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete vault: {}", e),
        )
            .into_response(),
    }
}
