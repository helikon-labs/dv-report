use crate::substrate::account_id::AccountId;
use crate::substrate::vote::Tally;
use frame_support::{Deserialize, Serialize};

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
