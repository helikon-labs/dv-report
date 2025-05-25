use async_trait::async_trait;
use dv_report_config::Config;
use dv_report_repository::Repository;
use dv_report_service::Service;
use dv_report_types::substrate::event::ReferendumEvent;
use dv_report_types::substrate::network::Network;
use lazy_static::lazy_static;
use std::cmp::max;
use std::time::Duration;

mod metrics;

lazy_static! {
    static ref CONFIG: Config = Config::default();
}

pub struct Indexer {
    repository: Repository,
}

impl Indexer {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            repository: Repository::new(&CONFIG).await?,
        })
    }

    pub async fn process_block(&self, network_id: u32, block_number: u64) -> anyhow::Result<()> {
        log::info!("Process block {block_number}.");
        let block = self.repository.get_block_by_number(block_number).await?;
        let block_vote_calls = self
            .repository
            .get_vote_calls_in_block(network_id, block_number)
            .await?;
        let block_referendum_events = self
            .repository
            .get_referendum_events_in_block(block_number)
            .await?;
        let mut new_referenda = Vec::new();
        for block_referendum_event in block_referendum_events.iter() {
            if let ReferendumEvent::Submitted {
                referendum_index,
                track_id: _,
            } = block_referendum_event
            {
                log::info!("New referendum {referendum_index}.");
                let new_referendum = self
                    .repository
                    .get_ongoing_referendum(network_id, *referendum_index, block.hash.as_str())
                    .await?;
                new_referenda.push(new_referendum);
            }
        }
        self.repository
            .save_block_with_details(
                network_id,
                &block,
                &new_referenda,
                &block_referendum_events,
                &block_vote_calls,
            )
            .await?;
        Ok(())
    }
}

#[async_trait(? Send)]
impl Service for Indexer {
    fn get_metrics_server_addr() -> (&'static str, u16) {
        (CONFIG.metrics.host.as_str(), CONFIG.metrics.indexer_port)
    }

    async fn run(&'static self) -> anyhow::Result<()> {
        let network = Network::from_id(CONFIG.substrate.network_id);
        let cohort = self
            .repository
            .get_cohort(network.id, CONFIG.indexer.cohort_number)
            .await?;
        log::info!(
            "{} indexer started for DV Cohort #{}.",
            network.display,
            cohort.number,
        );
        let delegates = self
            .repository
            .get_cohort_delegates(network.id, cohort.number)
            .await?;
        self.repository
            .init_cohort(&network, &cohort, delegates.as_slice())
            .await?;
        let delay_seconds = CONFIG.common.recovery_retry_seconds;
        if let Some(end_block_number) = CONFIG.indexer.end_block_number {
            let max_block_number = self.repository.get_max_block_number(network.id).await?;
            let start_block_number = max((max_block_number + 1) as u64, cohort.start_block.number);
            for block_number in start_block_number..=end_block_number {
                self.process_block(network.id, block_number).await?;
                metrics::indexed_finalized_block_number().set(block_number as i64);
                log::info!("Indexed block {block_number}.");
            }
            return Ok(());
        }
        loop {
            let finalized_block = self.repository.get_finalized_block().await?;
            let max_block_number = self.repository.get_max_block_number(network.id).await?;
            let start_block_number = max((max_block_number + 1) as u64, cohort.start_block.number);
            for block_number in start_block_number..=finalized_block.number {
                self.process_block(network.id, block_number).await?;
                metrics::indexed_finalized_block_number().set(block_number as i64);
                log::info!("Indexed block {block_number}.");
            }
            log::info!(
                "Reached finalized head {}. Will check again in {delay_seconds} seconds.",
                finalized_block.number,
            );
            tokio::time::sleep(Duration::from_secs(delay_seconds)).await;
        }
    }
}
