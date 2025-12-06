use crate::api::get_all_async;
use crate::api::handlers::vault_handler::delete_async;
use crate::utils::index::generate_password;
use crate::{
    api::handlers::vault_handler::{create_async, update_async},
    db::conn::{get_db, init_db}
};
use axum::{
    Router,
    routing::{get, put},
};
use migration::{Migrator, MigratorTrait};
use tokio::net::TcpListener;

mod api;
mod db;
mod models;
mod utils;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize the global database connection
    init_db().await?;
    println!("Database connection successful. Running migrations...");

    // 2. Apply migrations using SeaORM's Migrator with the global connection
    Migrator::up(get_db(), None)
        .await
        .map_err(|e| format!("Migration failed. Error: {}", e))?;

    println!("Migrations applied successfully.");

    // 3. Web Server Setup
    let app = Router::new()
        .route("/generate-password", 
        get(generate_password))
        .route("/api/admin/vaults", 
        get(get_all_async).post(create_async))
        .route("/api/admin/vaults/{id}",
         put(update_async).delete(delete_async)
        );

    // 4. Start the Server
    let listener: TcpListener = TcpListener::bind("0.0.0.0:5000").await?;
    println!("Web server listening on http://localhost:5000");

    axum::serve(listener, app).await?;

    Ok(())
}
