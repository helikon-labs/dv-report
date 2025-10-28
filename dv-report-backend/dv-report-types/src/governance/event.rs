use crate::substrate::account_id::AccountId;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct ReferendumDecisionDepositPlacedEvent {
    #[serde(skip_serializing)]
    pub id: u32,
    pub network_id: u32,
    pub block_hash: String,
    pub referendum_index: u32,
    pub amount: u128,
    pub who: AccountId,
}
