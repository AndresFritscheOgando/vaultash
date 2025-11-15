use sqlx::PgPool;

async fn database_connection(){
    let pool = PgPool::connect("postgresql://...").await?;

}
