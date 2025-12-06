use axum::Json;
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
