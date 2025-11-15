use sqlx::PgPool;
use std::env;

pub async fn database_connection(){
    dotenvy::dotenv;
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    Ok(pool)
}
