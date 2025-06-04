#![warn(clippy::disallowed_types)]

use async_trait::async_trait;
use dv_report_config::Config;
use dv_report_types::substrate::network::Network;
use std::str::FromStr;

pub mod err;

#[async_trait]
pub trait Service {
    fn get_metrics_server_addr(&'static self) -> (&'static str, u16);

    async fn run(&'static self) -> anyhow::Result<()>;

    async fn start(&'static self) {
        let config = Config::default();
        dv_report_logging::init(&config);
        Network::from_str(&config.substrate.chain)
            .unwrap()
            .sp_core_set_default_ss58_version();
        log::info!("Starting service...");
        tokio::spawn(dv_report_metrics::server::start(
            self.get_metrics_server_addr(),
        ));
        let delay_seconds = config.common.recovery_retry_seconds;
        loop {
            let result = self.run().await;
            if let Err(error) = result {
                log::error!("{error:?}");
                log::error!(
                    "Process exited with error. Will try again in {delay_seconds} seconds."
                );
                tokio::time::sleep(std::time::Duration::from_secs(delay_seconds)).await;
            } else {
                log::info!("Process completed successfully.");
                break;
            }
        }
    }
}
