use crate::substrate::block::Block;
use crate::substrate::track::Track;
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
    pub submission_block: Block,
    pub status: ReferendumStatus,
}
