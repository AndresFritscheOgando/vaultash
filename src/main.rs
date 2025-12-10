
use crate::utils::index::{cors_layer, generate_password};
use crate::db::conn::{get_db, init_db};
use migration::{Migrator, MigratorTrait};
use tokio::net::TcpListener;
use axum::Router;

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
        .merge(api::routes::create_routes())
        .layer(cors_layer());
    
    // 4. Start the Server
    let listener: TcpListener = TcpListener::bind("0.0.0.0:5000").await?;
    println!("Web server listening on http://localhost:5000");

    axum::serve(listener, app).await?;

    Ok(())
}
