use async_trait::async_trait;
use dv_report_config::Config;
use dv_report_persistence::postgres::PostgreSQLStorage;
use dv_report_service::Service;
use dv_report_subsquare_client::SubsquareClient;
use dv_report_substrate_client::ReferendumInfo;
use dv_report_substrate_client::SubstrateClient;
use dv_report_types::dv::cohort::Cohort;
use dv_report_types::dv::delegate::Delegate;
use dv_report_types::governance::referendum::{Referendum, ReferendumStatus};
use dv_report_types::governance::track::Track;
use dv_report_types::substrate::chain::Chain;
use lazy_static::lazy_static;

mod metrics;

lazy_static! {
    static ref CONFIG: Config = Config::default();
}

pub struct Indexer {
    postgres: PostgreSQLStorage,
    subsquare_client: SubsquareClient,
    substrate_client: SubstrateClient,
}

impl Indexer {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            postgres: PostgreSQLStorage::new(&CONFIG).await?,
            subsquare_client: SubsquareClient::new(&CONFIG)?,
            substrate_client: SubstrateClient::new(
                CONFIG.substrate.rpc_url.as_str(),
                CONFIG.substrate.connection_timeout_seconds,
                CONFIG.substrate.request_timeout_seconds,
            )
            .await?,
        })
    }

    async fn init_cohort(
        &self,
        network: &Chain,
        cohort: &Cohort,
        delegates: &[Delegate],
    ) -> anyhow::Result<()> {
        if self.postgres.get_referendum_count().await? > 0 {
            log::info!("Cohort had been initialized.");
            return Ok(());
        }
        log::info!("Initialize {} cohort #{}.", network.display, cohort.number);
        let start_block_hash = self
            .substrate_client
            .get_block_hash(cohort.start_block_number)
            .await?;
        log::info!("{} :: {start_block_hash}", cohort.start_block_number);
        let referendum_count = self
            .substrate_client
            .get_referendum_count(start_block_hash.as_str())
            .await?;
        log::info!("{referendum_count} referenda.");
        let mut tx = self.postgres.begin_tx().await?;
        for index in 1490..referendum_count {
            if let Some(referendum_info) = self
                .substrate_client
                .get_referendum_info(index, start_block_hash.as_str())
                .await?
            {
                match referendum_info {
                    ReferendumInfo::Ongoing(status) => {
                        let referendum = Referendum {
                            id: 0,
                            network_id: network.id,
                            index,
                            track: Track::from_id(status.track),
                            submission_block_number: status.submitted as u64,
                            status: ReferendumStatus::Ongoing,
                        };
                        log::info!(
                            "Save ongoing referendum #{index} on track {}.",
                            referendum.track.name()
                        );
                        self.postgres.save_referendum(&referendum, &mut tx).await?;
                        let vote_calls = self
                            .subsquare_client
                            .fetch_vote_calls(network, index)
                            .await?;
                        for delegate in delegates.iter() {
                            if let Some(delegate_vote_call) = vote_calls
                                .iter()
                                .find(|v| v.voter == delegate.delegation.delegate_account_id)
                            {
                                if delegate_vote_call.extrinsic.block_number
                                    < cohort.start_block_number
                                {
                                    log::info!("{} pre-voted on {}.", delegate.name, index);
                                } else {
                                    log::info!("{} post-voted on {}.", delegate.name, index);
                                }
                            }
                        }
                    }
                    _ => log::info!("Skip referendum #{index}."),
                }
            }
        }
        self.postgres.commit_tx(tx).await?;
        Ok(())
    }
}

#[async_trait(? Send)]
impl Service for Indexer {
    fn get_metrics_server_addr() -> (&'static str, u16) {
        (CONFIG.metrics.host.as_str(), CONFIG.metrics.indexer_port)
    }

    async fn run(&'static self) -> anyhow::Result<()> {
        let chain = Chain::from_id(CONFIG.substrate.chain_id);
        log::info!("{} indexer started.", chain.display);
        let cohort = self
            .postgres
            .get_cohort(CONFIG.indexer.cohort_number, chain.id)
            .await?;
        let delegates = self
            .postgres
            .get_all_delegates(CONFIG.indexer.cohort_number, chain.id)
            .await?;
        log::info!("Found {} delegates.", delegates.len());
        self.init_cohort(&chain, &cohort, delegates.as_slice())
            .await?;
        let header = self.substrate_client.get_finalized_block_header().await?;
        for block_number in 26073110..header.get_number()? {
            log::info!("Process block {}.", block_number);
            let hash = self.substrate_client.get_block_hash(block_number).await?;
            let vote_calls = self.substrate_client.get_vote_calls_in_block(&hash).await?;
            log::info!(
                "Got {} vote calls and {} remove vote calls for block {}.",
                vote_calls.vote_calls.len(),
                vote_calls.remove_vote_calls.len(),
                block_number
            );
            for vote_call in vote_calls.vote_calls.iter() {
                let voter = vote_call.voter;
                log::info!(
                    "VOTER: {}",
                    voter.to_ss58_check_with_version(chain.ss58_prefix)
                );
                if let Some(delegate) = delegates
                    .iter()
                    .find(|delegate| delegate.delegation.delegate_account_id == voter)
                {
                    log::info!(
                        "{} voted on {} #{}.",
                        delegate.name,
                        chain.display,
                        vote_call.referendum_index
                    );
                }
            }
        }
        // tokio::spawn(async move {});
        Ok(())
    }
}
