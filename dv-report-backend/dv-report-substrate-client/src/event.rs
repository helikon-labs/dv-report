use dv_report_types::governance::referendum::{ReferendumEvent, Tally};
use dv_report_types::metadata::polkadot::api::referenda::events::{
    Approved, Cancelled, ConfirmAborted, ConfirmStarted, Confirmed, DecisionDepositPlaced,
    DecisionDepositRefunded, DecisionStarted, DepositSlashed, Killed, Rejected,
    SubmissionDepositRefunded, Submitted, TimedOut,
};
use dv_report_types::substrate::account_id::AccountId;

pub(super) async fn get_referendum_events_in_block(
    block: &crate::vote::Block,
) -> anyhow::Result<Vec<ReferendumEvent>> {
    let mut referendum_events = Vec::new();
    let block_events = block.events().await?;
    for event in block_events.find::<Submitted>() {
        let event = event?;
        referendum_events.push(ReferendumEvent::Submitted(event.index, event.track));
    }
    for event in block_events.find::<DecisionDepositPlaced>() {
        let event = event?;
        referendum_events.push(ReferendumEvent::DecisionDepositPlaced(
            event.index,
            event.amount,
            AccountId::from(event.who.0),
        ));
    }
    for event in block_events.find::<DecisionDepositRefunded>() {
        let event = event?;
        referendum_events.push(ReferendumEvent::DecisionDepositRefunded(
            event.index,
            event.amount,
            AccountId::from(event.who.0),
        ));
    }
    for event in block_events.find::<DepositSlashed>() {
        let event = event?;
        referendum_events.push(ReferendumEvent::DepositSlashed(
            event.amount,
            AccountId::from(event.who.0),
        ));
    }
    for event in block_events.find::<DecisionStarted>() {
        let event = event?;
        referendum_events.push(ReferendumEvent::DecisionStarted(
            event.index,
            event.track,
            Tally {
                ayes: event.tally.ayes,
                nays: event.tally.nays,
                support: event.tally.support,
            },
        ));
    }
    for event in block_events.find::<ConfirmStarted>() {
        let event = event?;
        referendum_events.push(ReferendumEvent::ConfirmStarted(event.index));
    }
    for event in block_events.find::<ConfirmAborted>() {
        let event = event?;
        referendum_events.push(ReferendumEvent::ConfirmAborted(event.index));
    }
    for event in block_events.find::<Confirmed>() {
        let event = event?;
        referendum_events.push(ReferendumEvent::Confirmed(
            event.index,
            Tally {
                ayes: event.tally.ayes,
                nays: event.tally.nays,
                support: event.tally.support,
            },
        ));
    }
    for event in block_events.find::<Approved>() {
        let event = event?;
        referendum_events.push(ReferendumEvent::Approved(event.index));
    }
    for event in block_events.find::<Rejected>() {
        let event = event?;
        referendum_events.push(ReferendumEvent::Rejected(
            event.index,
            Tally {
                ayes: event.tally.ayes,
                nays: event.tally.nays,
                support: event.tally.support,
            },
        ));
    }
    for event in block_events.find::<Cancelled>() {
        let event = event?;
        referendum_events.push(ReferendumEvent::Cancelled(
            event.index,
            Tally {
                ayes: event.tally.ayes,
                nays: event.tally.nays,
                support: event.tally.support,
            },
        ));
    }
    for event in block_events.find::<TimedOut>() {
        let event = event?;
        referendum_events.push(ReferendumEvent::TimedOut(
            event.index,
            Tally {
                ayes: event.tally.ayes,
                nays: event.tally.nays,
                support: event.tally.support,
            },
        ));
    }
    for event in block_events.find::<Killed>() {
        let event = event?;
        referendum_events.push(ReferendumEvent::Killed(
            event.index,
            Tally {
                ayes: event.tally.ayes,
                nays: event.tally.nays,
                support: event.tally.support,
            },
        ));
    }
    for event in block_events.find::<SubmissionDepositRefunded>() {
        let event = event?;
        referendum_events.push(ReferendumEvent::SubmissionDepositRefunded(
            event.index,
            event.amount,
            AccountId::from(event.who.0),
        ));
    }
    Ok(referendum_events)
}
