mod handlers;
pub use handlers::*;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World" }))
}

