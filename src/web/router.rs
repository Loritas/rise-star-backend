use anyhow::Result;
use axum::routing::{ get, post};
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use crate::web::login_admin_handler::login_admin_handler;
use crate::web::login_user_handler::login_user_handler;
use crate::web::questions_handler::get_categories_handler;

pub async fn init_app() -> Result<Router<()>> {
    let app = Router::new()
        .route("/", get(test_handler))
        .route("/admin/login", post(login_admin_handler))
        .route("/api/login", post(login_user_handler))
        .route("/api/categories", get(get_categories_handler))
        .layer(
            CorsLayer::new()
                .allow_methods(Any)
                .allow_origin(Any)// 允许任意来源
                .allow_headers(Any)
        );
    Ok(app)
}

async fn test_handler() -> &'static str {
    "Hello, world!"
}
