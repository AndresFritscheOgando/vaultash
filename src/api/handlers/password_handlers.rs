use crate::db::connection::database_connection;

pub async fn get_all_async(){
    
    let pool = database_connection().await;
    let result = sqlx::query!("SELECT * FROM password")
        .fetch_all(pool)
        .await;
}