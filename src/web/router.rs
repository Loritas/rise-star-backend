use anyhow::Result;
use axum::routing::get;
use axum::Router;

pub async fn init_app() -> Result<Router<()>> {
    let app = Router::new().route("/", get(test_handler));
    Ok(app)
}

async fn test_handler() -> &'static str {
    "Hello, world!"
}
