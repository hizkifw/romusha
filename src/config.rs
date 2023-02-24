use std::collections::HashMap;

use anyhow::Context;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub romusha: ConfigHeader,
    pub service: Vec<ServiceConfig>,

    #[serde(skip)]
    config_path: String,
}

#[derive(Deserialize)]
pub struct ConfigHeader {
    pub version: String,
}

#[derive(Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub exec: String,
    pub cwd: Option<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_true() -> bool {
    true
}

/// Load a config file from the specified path
pub async fn load(path: &str) -> anyhow::Result<Config> {
    let config = tokio::fs::read_to_string(path)
        .await
        .with_context(|| format!("failed to read config file {:?}", path))?;
    let mut config: Config = toml::from_str(&config)
        .with_context(|| format!("failed to parse config file {:?}", path))?;
    config.config_path = path.to_string();
    Ok(config)
}
