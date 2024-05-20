use crate::config::env::Env;
use anyhow::Result;
use sea_orm::{Database, DatabaseConnection};

pub async fn init_db(env: &Env) -> Result<()> {
    let opt = format!(
        "{}://{}:{}@{}:{}/{}",
        env.database.protocol,
        env.database.username,
        env.database.password,
        env.database.url,
        env.database.port,
        env.database.database
    );
    let db: DatabaseConnection =
        Database::connect(opt).await?;
    Ok(())
}
