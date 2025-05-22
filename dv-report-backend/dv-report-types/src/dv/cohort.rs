use crate::substrate::chain::Chain;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cohort {
    pub number: u32,
    pub network: Chain,
    pub announcement_date: NaiveDateTime,
    pub announcement_url: Option<String>,
    pub delegation_date: NaiveDateTime,
    pub start_block_number: u64,
}
