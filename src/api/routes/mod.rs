pub mod service_routes;
pub mod vault_routes;

use axum::Router;

pub fn create_routes() -> Router {
    Router::new()
        .merge(service_routes::routes())
        .merge(vault_routes::routes())
}
