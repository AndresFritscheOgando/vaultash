use crate::api::handlers::vault_handler::{
    create_async, delete_async, get_all_async, get_by_id_async, update_async,
};
use crate::generate_password;
use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello World" })) // inline async handler
        .route("/generate-password", get(generate_password))
        .route("/api/admin/vaults", get(get_all_async).post(create_async))
        .route(
            "/api/admin/vaults/{id}",
            get(get_by_id_async).put(update_async).delete(delete_async),
        )
}
