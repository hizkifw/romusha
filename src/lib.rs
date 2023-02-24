#[macro_use]
extern crate log;

use anyhow::Context;

mod backoff;
mod config;
mod service;

pub async fn run() -> anyhow::Result<()> {
    info!("romusha init");

    // Read config file
    let cfg = config::load("romusha.toml")
        .await
        .with_context(|| format!("failed to load config"))?;

    // Start services
    futures::future::join_all(cfg.service.iter().map(|svc| svc.start())).await;

    Ok(())
}
