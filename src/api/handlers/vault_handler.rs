

use crate::models::dtos::vault::VaultInputDto;
use crate::models::entities::vault::Entity as Vault;
use crate::models::entities::vault;
use crate::db::conn::get_db;
use axum::response::{IntoResponse, Json};
use sea_orm_migration::prelude::prelude::Utc;

use axum::http::StatusCode;
use uuid::Uuid;
use crate::utils::index::generate_password;
use std::env;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, EntityTrait, Set, };
use serde_json::{json};




pub async fn get_all_async() -> impl IntoResponse {
    // Query get all

    let vaults = match Vault::find().all(get_db()).await {
        Ok(passwords) => Json(json!({ "data": passwords })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "No Vault has been found").into_response()
    };
    vaults
}



pub async fn create_async(
    Json(vault): Json<VaultInputDto>
) -> impl IntoResponse{

    let conn_string = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_URL not set").into_response()
    };

    let db: DatabaseConnection = match Database::connect(&conn_string).await {
        Ok(conn) => conn,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to connect to database").into_response()
    };


    // Call the shared password generator function
    let password_json = generate_password().await;
    let password = password_json.0["password"].as_str().unwrap_or("").to_string();

    let vault = vault::ActiveModel {
        id: Set(Uuid::new_v4()),
        service_name: Set(vault.service_name),
        service_password: Set(password),
        service_user_name: Set(vault.service_user_name),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now())
    };

    match vault.insert(&db).await {
        Ok(vault) => (StatusCode::CREATED, Json(json!({ "data": vault }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create vault: {}", e)).into_response()
    }
}
