use sea_orm::{Database, DatabaseConnection};
use std::env;
use once_cell::sync::OnceCell;
use dotenvy; // Already in scope

static DB: OnceCell<DatabaseConnection> = OnceCell::new();

pub async fn init_db() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load the .env file FIRST. This makes the variables available to env::var().
    dotenvy::dotenv().ok();

    // 2. Retrieve the DATABASE_URL environment variable.
    let conn_string = env::var("DATABASE_URL")
        .map_err(|e| format!("DATABASE_URL is not set: {}", e))?; // Return a clearer error

    // 3. Connect to the database using the retrieved string.
    let db = Database::connect(conn_string.clone()).await?; 

    // 4. Store the connection in the global OnceCell.
    DB.set(db).map_err(|_| "Database already initialized")?;

    println!("Database connection successful.");
    
    // Note: Printing the connection string itself is generally discouraged 
    // because it logs the password, so I removed the print statement of the full string.
    Ok(())
}

pub fn get_db() -> &'static DatabaseConnection {
    DB.get().expect("Database not initialized")
}