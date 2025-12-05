use axum::{Router, routing::{get}};
use migration::{Migrator, MigratorTrait};
use crate::{api::handlers::vault_handler::create_async, db::conn::{get_db, init_db}};
use crate::utils::index::generate_password;
use tokio::net::TcpListener;
use crate::api::get_all_async;


mod utils;
mod api;
mod db;
mod models;

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
    .route("/generate-password", get(generate_password))
    .route("/vaults", get(get_all_async).post(create_async))
    .with_state(get_db().clone());




// 4. Start the Server
let listener: TcpListener = TcpListener::bind("0.0.0.0:5000").await?;
println!("Web server listening on http://localhost:5000");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}