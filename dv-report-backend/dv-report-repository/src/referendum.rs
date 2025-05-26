use crate::Repository;
use dv_report_substrate_client::ReferendumInfo;
use dv_report_types::governance::referendum::{Referendum, ReferendumStatus};
use dv_report_types::substrate::track::Track;

impl Repository {
    pub async fn get_referendum_count(&self, block_hash: &str) -> anyhow::Result<u32> {
        self.substrate_client.get_referendum_count(block_hash).await
    }

    pub async fn get_ongoing_referendum(
        &self,
        network_id: u32,
        referendum_index: u32,
        block_hash: &str,
    ) -> anyhow::Result<Referendum> {
        let Some(referendum_info) = self
            .substrate_client
            .get_referendum_info(referendum_index, block_hash)
            .await?
        else {
            return Err(anyhow::Error::msg(format!(
                "Referendum {referendum_index} not found in Substrate storage."
            )));
        };
        let referendum = match referendum_info {
            ReferendumInfo::Ongoing(status) => {
                let submission_block_hash = self
                    .substrate_client
                    .get_block_hash(status.submitted as u64)
                    .await?;
                let submission_block = self
                    .substrate_client
                    .get_block(submission_block_hash.as_str())
                    .await?;
                Referendum {
                    id: 0,
                    network_id,
                    index: referendum_index,
                    track: Track::from_id(status.track),
                    submission_block,
                    status: ReferendumStatus::Ongoing,
                }
            }
            _ => {
                return Err(anyhow::Error::msg(format!(
                    "Referendum {referendum_index} is not ongoing."
                )))
            }
        };
        Ok(referendum)
    }
}
