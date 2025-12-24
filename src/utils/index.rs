use axum::Json;
use tower_http::cors::{CorsLayer, Any};
use passwords::PasswordGenerator;
use serde_json::json;

pub async fn generate_password() -> Json<serde_json::Value> {
    let pg = PasswordGenerator {
        length: 16,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };

    let password = pg.generate_one().unwrap();
    Json(json!({ "password": password }))
}

pub fn cors_layer() -> CorsLayer {
    let cors = CorsLayer::new()
        // Allow requests from any origin (not recommended for production)
        .allow_origin(Any)
        // Uncomment and modify the following to allow specific origins in production:
        .allow_origin([
            "http://localhost:3000".parse().unwrap(),
            // "http://localhost:3001".parse().unwrap(),
            "https://v0-password-manager-ui-sigma.vercel.app".parse().unwrap(),
        ])
        // Allow specific headers like Authorization and Content-Type
        .allow_headers(Any)
        // Allow GET and POST methods
        .allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::DELETE,
            http::Method::PUT,
        ]);
        cors
}
