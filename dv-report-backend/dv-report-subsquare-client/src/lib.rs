use dv_report_config::Config;
use dv_report_types::governance::subsquare::{
    SubsquareReferendum, SubsquareReferendumList, SubsquareVoteCall,
};
use dv_report_types::substrate::network::Network;

pub struct SubsquareClient {
    http_client: reqwest::Client,
}

impl SubsquareClient {
    pub fn new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self {
            http_client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(
                    config.http.request_timeout_seconds,
                ))
                .build()?,
        })
    }

    pub async fn fetch_referendum(
        &self,
        network: &Network,
        index: u32,
    ) -> anyhow::Result<Option<SubsquareReferendum>> {
        let url = format!(
            "https://{}-api.subsquare.io/gov2/referendums/{index}?simple=false",
            network.chain,
        );
        let response = self.http_client.get(url).send().await?;
        if response.status().as_u16() == 404 {
            return Ok(None);
        }
        let referendum = response.json::<SubsquareReferendum>().await?;
        Ok(Some(referendum))
    }

    pub async fn fetch_referenda(
        &self,
        chain: &Network,
        page: u16,
        page_size: u16,
    ) -> anyhow::Result<SubsquareReferendumList> {
        let url = format!(
            "https://{}-api.subsquare.io/gov2/referendums?simple=false&page_size={page_size}&page={page}",
            chain.chain,
        );
        Ok(self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<SubsquareReferendumList>()
            .await?)
    }

    pub async fn fetch_vote_calls(
        &self,
        chain: &Network,
        index: u32,
    ) -> anyhow::Result<Vec<SubsquareVoteCall>> {
        let url = format!(
            "https://{}-api.subsquare.io/gov2/referendums/{index}/vote-calls",
            chain.chain,
        );
        Ok(self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<Vec<SubsquareVoteCall>>()
            .await?)
    }
}
