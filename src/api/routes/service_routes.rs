use axum::{Router, routing::{get}};
use crate::api::handlers::service_handler::{
    create_service_async,
    get_all_services_async
};

pub fn routes() -> Router {
    Router::new()
        .route("/api/admin/vaults", 
        get(get_all_services_async).post(create_service_async))
    
}
