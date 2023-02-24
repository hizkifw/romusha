use crate::{backoff::Backoff, config};
use std::process::Stdio;
use tokio::process::Command;

impl config::ServiceConfig {
    pub async fn start(&self) -> anyhow::Result<()> {
        if !self.enabled {
            return Ok(());
        }

        info!("Starting {}", self.name);

        // Retry backoff parameters
        let mut backoff = Backoff::new();

        loop {
            // Spawn the process
            // TODO: do something with stdout/stderr
            let child = Command::new(&self.exec)
                .args(&self.args)
                .stdin(Stdio::null())
                .envs(&self.env)
                .current_dir(self.cwd.clone().unwrap_or(".".into()))
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn();

            // Check if spawn OK
            let mut child = match child {
                Err(err) => {
                    error!("error spawning task {}: {:?}", self.name, err);
                    backoff.fail().await;
                    continue;
                }
                Ok(child) => child,
            };

            // Wait for process to exit
            match child.wait().await {
                Err(err) => {
                    error!("error waiting for task {} to finish: {:?}", self.name, err);
                    backoff.fail().await;
                    continue;
                }
                Ok(res) => {
                    if !res.success() {
                        error!(
                            "task {} exited with code: {:?}",
                            self.name,
                            res.code().unwrap_or(-1)
                        );
                        backoff.fail().await;
                        continue;
                    }
                }
            }

            // Reset backoff if all ok
            backoff.reset();
        }
    }
}
