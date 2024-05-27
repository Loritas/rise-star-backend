use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::error::Category;
use crate::config::get_db_conn;

pub async fn get_categories_handler() -> impl IntoResponse {
    let db = get_db_conn();
    let categories = Category::find().all(db).await.unwrap();
    let mut categories_vec = Vec::new();
    for category in categories {
        categories_vec.push(category.name);
    }
    (StatusCode::OK, Json(categories_vec))
}