use axum::{Router, routing::{get}};
use crate::generate_password;
use crate::api::handlers::vault_handler::{
    delete_async,
    get_by_id_async,
    create_async,
    update_async,
    get_all_async,
};


pub fn routes() -> Router {
    Router::new()
        .route("/generate-password", 
        get(generate_password))
        .route("/api/admin/vaults", 
        get(get_all_async).post(create_async))
        .route("/api/admin/vaults/{id}",
        get(get_by_id_async).put(update_async).delete(delete_async)
        )
}
