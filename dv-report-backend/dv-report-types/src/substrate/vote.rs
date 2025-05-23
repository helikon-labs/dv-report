use crate::metadata::polkadot::api::runtime_types::pallet_conviction_voting::vote::AccountVote;
use crate::substrate::account_id::AccountId;
use frame_support::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct BlockVoteCalls {
    pub vote_calls: Vec<VoteCall>,
    pub remove_vote_calls: Vec<RemoveVoteCall>,
}

impl BlockVoteCalls {
    pub fn append(&mut self, block_vote_calls: &mut BlockVoteCalls) {
        self.vote_calls.append(&mut block_vote_calls.vote_calls);
        self.remove_vote_calls
            .append(&mut block_vote_calls.remove_vote_calls);
    }
}

#[derive(Debug)]
pub struct VoteCall {
    pub block_hash: String,
    pub block_number: u64,
    pub extrinsic_index: u32,
    pub extrinsic_hash: String,
    pub is_batch: bool,
    pub is_multisig: bool,
    pub is_proxy: bool,
    pub is_successful: bool,
    pub signer: AccountId,
    pub voter: AccountId,
    pub referendum_index: u32,
    pub vote: AccountVote<u128>,
}

#[derive(Debug)]
pub struct RemoveVoteCall {
    pub block_hash: String,
    pub block_number: u64,
    pub extrinsic_index: u32,
    pub extrinsic_hash: String,
    pub is_batch: bool,
    pub is_multisig: bool,
    pub is_proxy: bool,
    pub is_successful: bool,
    pub signer: AccountId,
    pub voter: AccountId,
    pub referendum_index: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tally {
    pub ayes: u128,
    pub nays: u128,
    pub support: u128,
}
