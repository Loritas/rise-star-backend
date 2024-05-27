use crate::config::env::Env;
use anyhow::Result;
use once_cell::sync::OnceCell;
use sea_orm::{Database, DatabaseConnection};

static DB_CONN: OnceCell<DatabaseConnection> = OnceCell::new();

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
    let db: DatabaseConnection = Database::connect(opt).await?;
    DB_CONN
        .set(db)
        .expect("Database connection already initialized");
    Ok(())
}

pub fn get_db_conn() -> &'static DatabaseConnection {
    DB_CONN.get().expect("Database connection not initialized")
}
