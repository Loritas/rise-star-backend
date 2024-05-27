use anyhow::Result;
use rise_star::{init_app, init_config};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    init_config().await?;
    let app = init_app().await?;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
