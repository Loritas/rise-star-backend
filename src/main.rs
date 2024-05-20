use anyhow::Result;
use rise_star::{init_app, init_config};

#[tokio::main]
async fn main() -> Result<()> {
    init_config().await?;
    let app = init_app().await?;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
