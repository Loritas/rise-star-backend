use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;

#[derive(Serialize)]
struct ApiResponse {
    code: i32,
    message: String
}

pub async fn login_admin_handler() -> impl IntoResponse {
    let response = ApiResponse {
        code: 200,
        message: "Login success".to_string(),
    };

    (StatusCode::OK, Json(response))
}
