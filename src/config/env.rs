use serde::Deserialize;
use anyhow::Result;
use toml;
use crate::config::db::init_db;

#[derive(Debug, Deserialize)]
pub struct Env {
    config: Config,
    pub(crate) database: Database,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub author: String
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub protocol: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub port: u16,
    pub database: String,
}

pub async fn init_config() -> Result<()> {
    let env = parse_env()?;
    init_db(&env).await?;
    println!("{:?}", env);
    Ok(())
}

fn parse_env() -> Result<Env> {
    let env_toml_str = std::fs::read_to_string("resources/env.toml")?;
    let env = toml::from_str(&env_toml_str)?;

    Ok(env)
}