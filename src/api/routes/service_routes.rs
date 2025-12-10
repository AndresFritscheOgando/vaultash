use axum::{
    Router, 
    routing::{get, post},
    extract::DefaultBodyLimit,
    response::IntoResponse,
};
use tower_http::limit::RequestBodyLimitLayer;

use crate::api::handlers::service_handler::{get_all_services_async, create_service_async};

// 10MB max file size
const MAX_UPLOAD_SIZE: usize = 10 * 1024 * 1024;

// Helper function to handle the Result from the service handlers
async fn handle_service_result<T: IntoResponse>(
    result: Result<T, crate::api::handlers::service_handler::ServiceError>,
) -> impl IntoResponse {
    match result {
        Ok(response) => response.into_response(),
        Err(e) => e.into_response(),
    }
}

pub fn routes() -> Router {
    let router = Router::new()
        .route(
            "/api/admin/services", 
            get(get_all_services_async)
        )
        .route(
            "/api/admin/services", 
            post(|multipart| async move { 
                handle_service_result(create_service_async(multipart).await).await 
            })
        )
        .layer(DefaultBodyLimit::max(MAX_UPLOAD_SIZE))
        .layer(RequestBodyLimitLayer::new(MAX_UPLOAD_SIZE));
        
    router
}
