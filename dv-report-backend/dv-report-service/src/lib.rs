#![warn(clippy::disallowed_types)]

use async_trait::async_trait;
use dv_report_config::Config;
use dv_report_types::substrate::network::Network;
use std::str::FromStr;

pub mod err;

#[async_trait]
pub trait Service {
    fn get_metrics_server_addr(&self) -> (String, u16);

    async fn run(&self) -> anyhow::Result<()>;

    async fn start(&self) -> anyhow::Result<()> {
        let config = Config::default();
        dv_report_logging::init(&config);
        let network = Network::from_str(&config.substrate.chain)
            .map_err(|e| anyhow::anyhow!("Invalid network config: {e}"))?;

        network.sp_core_set_default_ss58_version();
        log::info!("Starting service for {network}...");

        let (host, port) = self.get_metrics_server_addr();
        tokio::spawn(async move {
            dv_report_metrics::server::start((host, port)).await;
        });

        let retry_delay = config.common.recovery_retry_seconds;
        loop {
            match self.run().await {
                Ok(()) => {
                    log::info!("Service run completed successfully.");
                    return Ok(());
                }
                Err(e) => {
                    log::error!("Service run failed: {e:?}. Retrying in {retry_delay} seconds.");
                    tokio::time::sleep(std::time::Duration::from_secs(retry_delay)).await;
                }
            }
        }
    }
}
