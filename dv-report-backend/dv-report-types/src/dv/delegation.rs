use crate::substrate::account_id::AccountId;
use crate::substrate::chain::Chain;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Delegation {
    pub id: u32,
    pub cohort_number: u32,
    pub network: Chain,
    pub delegator_account_id: AccountId,
    pub delegate_id: String,
    pub delegate_account_id: AccountId,
    pub delegation_start_block_number: u64,
    pub delegation_start_block_hash: String,
    pub delegation_start_extrinsic_hash: String,
    pub delegation_start_extrinsic_index: u32,
    pub delegation_end_block_number: Option<u64>,
    pub delegation_end_block_hash: Option<String>,
    pub delegation_end_extrinsic_hash: Option<String>,
    pub delegation_end_extrinsic_index: Option<u32>,
}
