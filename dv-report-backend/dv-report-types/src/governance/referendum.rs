use crate::governance::track::Track;
use crate::substrate::account_id::AccountId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum ReferendumStatus {
    Ongoing,
    Approved,
    Rejected,
    Cancelled,
    TimedOut,
    Killed,
}

impl ReferendumStatus {
    pub fn id(&self) -> u32 {
        match self {
            ReferendumStatus::Ongoing => 1,
            ReferendumStatus::Approved => 2,
            ReferendumStatus::Rejected => 3,
            ReferendumStatus::Cancelled => 4,
            ReferendumStatus::TimedOut => 5,
            ReferendumStatus::Killed => 6,
        }
    }

    pub fn from_id(id: u32) -> Self {
        match id {
            1 => ReferendumStatus::Ongoing,
            2 => ReferendumStatus::Approved,
            3 => ReferendumStatus::Rejected,
            4 => ReferendumStatus::Cancelled,
            5 => ReferendumStatus::TimedOut,
            6 => ReferendumStatus::Killed,
            _ => panic!("Unknown referendum status id: {id}"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Referendum {
    pub id: u32,
    pub network_id: u32,
    pub index: u32,
    pub track: Track,
    pub submission_block_number: u64,
    pub status: ReferendumStatus,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tally {
    pub ayes: u128,
    pub nays: u128,
    pub support: u128,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ReferendumEvent {
    Submitted(u32, u16),
    DecisionDepositPlaced(u32, u128, AccountId),
    DecisionDepositRefunded(u32, u128, AccountId),
    DepositSlashed(u128, AccountId),
    DecisionStarted(u32, u16, Tally),
    ConfirmStarted(u32),
    ConfirmAborted(u32),
    Confirmed(u32, Tally),
    Approved(u32),
    Rejected(u32, Tally),
    Cancelled(u32, Tally),
    TimedOut(u32, Tally),
    Killed(u32, Tally),
    SubmissionDepositRefunded(u32, u128, AccountId),
}
