use crate::types::{ReferendumDTO, Tally};
use crate::{ResultResponse, ServiceState};
use actix_web::{get, web, HttpResponse};
use dv_report_types::err::ServiceError;
use dv_report_types::governance::referendum::{ReferendumStatus, ReferendumStatusRow};
use dv_report_types::governance::subsquare::SubsquareReferendumVoteRow;
use dv_report_types::substrate::account_id::AccountId;
use dv_report_types::substrate::block::Block;
use dv_report_types::substrate::track::{Track, TrackRow};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

fn validate_account_id_path_param(
    ss58_address_or_account_id: &str,
) -> Result<AccountId, HttpResponse> {
    let account_id = match AccountId::from_str(ss58_address_or_account_id) {
        Ok(account_id) => account_id,
        Err(_) => match AccountId::from_str(ss58_address_or_account_id) {
            Ok(account_id) => account_id,
            Err(_) => {
                return Err(HttpResponse::BadRequest()
                    .json(ServiceError::from("Invalid address or account id.")))
            }
        },
    };
    Ok(account_id)
}

#[get("/network")]
pub(crate) async fn get_all_networks(state: web::Data<ServiceState>) -> ResultResponse {
    let networks = state.postgres.get_all_networks().await?;
    Ok(HttpResponse::Ok().json(networks))
}

#[get("/cohort")]
pub(crate) async fn get_all_cohorts(state: web::Data<ServiceState>) -> ResultResponse {
    let rows = state.postgres.get_all_cohorts().await?;
    let mut cohorts = Vec::new();
    for row in rows.iter() {
        let start_block = state
            .postgres
            .get_block(row.network_id as u32, row.start_block_hash.as_str())
            .await?;
        cohorts.push(row.clone().into_cohort(start_block));
    }
    Ok(HttpResponse::Ok().json(cohorts))
}

#[get("/delegate/type")]
pub(crate) async fn get_all_delegate_types(state: web::Data<ServiceState>) -> ResultResponse {
    let rows = state.postgres.get_all_delegate_types().await?;
    Ok(HttpResponse::Ok().json(rows))
}

#[get("/delegate")]
pub(crate) async fn get_all_delegates(state: web::Data<ServiceState>) -> ResultResponse {
    if let Some(cached_delegates) = state.delegate_cache.get(&0).await {
        return Ok(HttpResponse::Ok().json(cached_delegates));
    }
    let rows = state.postgres.get_all_delegates().await?;
    let mut delegates = Vec::new();
    for delegate_row in rows.iter() {
        let delegation_rows = state
            .postgres
            .get_delegations_for_delegate(delegate_row.id.as_str())
            .await?;
        let mut delegations = Vec::new();
        for delegation_row in delegation_rows.iter() {
            let delegation_start_block = state
                .postgres
                .get_block(
                    delegation_row.network_id as u32,
                    delegation_row.start_block_hash.as_str(),
                )
                .await?;
            let delegation_end_block =
                if let Some(delegation_end_block_hash) = &delegation_row.end_block_hash {
                    Some(
                        state
                            .postgres
                            .get_block(delegation_row.network_id as u32, delegation_end_block_hash)
                            .await?,
                    )
                } else {
                    None
                };
            delegations.push(
                delegation_row
                    .clone()
                    .into_delegation(delegation_start_block, delegation_end_block)?,
            );
        }
        delegates.push(delegate_row.clone().into_delegate(delegations))
    }
    state.delegate_cache.insert(0, delegates.clone()).await;
    Ok(HttpResponse::Ok().json(delegates))
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Vote {
    pub id: i32,
    pub network_id: i32,
    pub referendum_index: i32,
    pub block: Block,
    pub extrinsic_index: i32,
    pub extrinsic_hash: String,
    pub is_batch: bool,
    pub is_multisig: bool,
    pub is_multisig_executed: bool,
    pub is_proxy: bool,
    pub is_successful: bool,
    pub signer_account_id: String,
    pub voter_account_id: String,
    pub vote_type: String,
    pub is_aye: Option<bool>,
    pub conviction: Option<i32>,
    pub balance: Option<String>,
    pub aye: Option<String>,
    pub nay: Option<String>,
    pub abstain: Option<String>,
    pub subsquare_comment_id: Option<String>,
    pub polkassembly_comment_id: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct NetworkCohortVoterAccountIdPathParameter {
    network_id: u32,
    cohort_number: u32,
    voter_account_id: String,
}

#[get("/network/{network_id}/cohort/{cohort_number}/voter/{voter_account_id}/vote")]
pub(crate) async fn get_network_cohort_voter_votes(
    path: web::Path<NetworkCohortVoterAccountIdPathParameter>,
    state: web::Data<ServiceState>,
) -> ResultResponse {
    let account_id = match validate_account_id_path_param(path.voter_account_id.as_str()) {
        Ok(account_id) => account_id,
        Err(response) => return Ok(response),
    };
    if let Some(cached_vote_calls) = state
        .network_voter_vote_cache
        .get(&(path.network_id, account_id))
        .await
    {
        return Ok(HttpResponse::Ok().json(cached_vote_calls));
    }

    let rows = state
        .postgres
        .get_network_cohort_voter_votes(path.network_id, path.cohort_number, &account_id)
        .await?;
    let mut votes = Vec::new();
    for row in rows.iter() {
        let block = state
            .postgres
            .get_block(row.network_id as u32, row.block_hash.as_str())
            .await?;
        votes.push(Vote {
            id: row.id,
            network_id: row.network_id,
            referendum_index: row.referendum_index,
            block,
            extrinsic_index: row.extrinsic_index,
            extrinsic_hash: row.extrinsic_hash.clone(),
            is_batch: row.is_batch,
            is_multisig: row.is_multisig,
            is_multisig_executed: row.is_multisig_executed,
            is_proxy: row.is_proxy,
            is_successful: row.is_successful,
            signer_account_id: row.signer_account_id.clone(),
            voter_account_id: row.voter_account_id.clone(),
            vote_type: row.vote_type.clone(),
            is_aye: row.is_aye,
            conviction: row.conviction,
            balance: row.balance.clone(),
            aye: row.aye.clone(),
            nay: row.nay.clone(),
            abstain: row.abstain.clone(),
            subsquare_comment_id: row.subsquare_comment_id.clone(),
            polkassembly_comment_id: row.polkassembly_comment_id.clone(),
        })
    }
    state
        .network_voter_vote_cache
        .insert((path.network_id, account_id), votes.clone())
        .await;
    Ok(HttpResponse::Ok().json(votes))
}

#[get("/referendum/status")]
pub(crate) async fn get_all_referendum_statuses(state: web::Data<ServiceState>) -> ResultResponse {
    Ok(HttpResponse::Ok().json(state.postgres.get_all_referendum_statuses().await?))
}

#[derive(Deserialize)]
pub(crate) struct NetworkPathParameter {
    network_id: u32,
}

#[get("/network/{network_id}/track")]
pub(crate) async fn get_all_referendum_tracks(
    path: web::Path<NetworkPathParameter>,
    state: web::Data<ServiceState>,
) -> ResultResponse {
    Ok(HttpResponse::Ok().json(
        state
            .postgres
            .get_all_tracks_for_network(path.network_id)
            .await?,
    ))
}

#[derive(Deserialize)]
pub(crate) struct NetworkCohortPathParameter {
    network_id: u32,
    cohort_number: u32,
}

fn get_tally(
    votes: &[SubsquareReferendumVoteRow],
    exclude_account_ids: &[AccountId],
) -> anyhow::Result<Tally> {
    let mut tally = (0u128, 0u128, 0u128);
    for vote in votes.iter() {
        let account_id = AccountId::from_str(&vote.account_id)?;
        if vote.delegate_account_id.is_some() || exclude_account_ids.contains(&account_id) {
            continue;
        }
        if let (Some(is_aye), Some(power)) = (vote.aye, &vote.votes) {
            let power: u128 = power.parse()?;
            if is_aye {
                tally.0 += power;
            } else {
                tally.1 += power;
            }
            if let Some(delegated_power) = &vote.delegated_votes {
                let delegated_power: u128 = delegated_power.parse()?;
                if is_aye {
                    tally.0 += delegated_power;
                } else {
                    tally.1 += delegated_power;
                }
            }
        }
        if let Some(aye_power) = &vote.aye_votes {
            let aye_power: u128 = aye_power.parse()?;
            tally.0 += aye_power;
        }
        if let Some(nay_power) = &vote.nay_votes {
            let nay_power: u128 = nay_power.parse()?;
            tally.1 += nay_power;
        }
        if let Some(abstain_power) = &vote.abstain_votes {
            let abstain_power: u128 = abstain_power.parse()?;
            tally.2 += abstain_power;
        }
    }
    Ok(Tally {
        ayes: tally.0.to_string(),
        nays: tally.1.to_string(),
        abstains: tally.2.to_string(),
    })
}

#[get("/network/{network_id}/cohort/{cohort_number}/referendum")]
pub(crate) async fn get_network_cohort_referenda(
    path: web::Path<NetworkCohortPathParameter>,
    state: web::Data<ServiceState>,
) -> ResultResponse {
    if let Some(cached_referenda) = state
        .network_cohort_referendum_cache
        .get(&(path.network_id, path.cohort_number))
        .await
    {
        return Ok(HttpResponse::Ok().json(cached_referenda));
    }

    let rows = state
        .postgres
        .get_network_cohort_referenda(path.network_id, path.cohort_number)
        .await?;
    let delegate_account_ids = state
        .postgres
        .get_network_cohort_delegate_account_ids(path.network_id, path.cohort_number)
        .await?;
    let mut referenda = Vec::new();
    for row in rows.iter() {
        let votes = state
            .postgres
            .get_subsquare_referendum_votes(path.network_id, row.index as u32)
            .await?;
        let submission_block = state
            .postgres
            .get_block(row.network_id as u32, row.submission_block_hash.as_str())
            .await?;
        let track = Track::from_id(row.track_id as u16);
        let status = ReferendumStatus::from_id(row.status_id as u32);
        referenda.push(ReferendumDTO {
            network_id: row.network_id as u32,
            index: row.index as u32,
            track: TrackRow {
                network_id: row.network_id,
                id: track.id() as i32,
                name: track.name().to_string(),
            },
            submission_block,
            status: ReferendumStatusRow {
                id: status.id() as i32,
                status: status.name(),
            },
            decision_deposit_placed_event: state
                .postgres
                .get_referendum_decision_deposit_placed_event(
                    row.network_id as u32,
                    row.index as u32,
                )
                .await?,
            is_retracted: row.is_retracted,
            tally: get_tally(&votes, &[])?,
            tally_without_dv: get_tally(&votes, &delegate_account_ids)?,
        });
    }
    state
        .network_cohort_referendum_cache
        .insert((path.network_id, path.cohort_number), referenda.clone())
        .await;
    Ok(HttpResponse::Ok().json(referenda))
}

#[get("/network/{network_id}/cohort/{cohort_number}/track")]
pub(crate) async fn get_all_network_cohort_tracks(
    path: web::Path<NetworkCohortPathParameter>,
    state: web::Data<ServiceState>,
) -> ResultResponse {
    Ok(HttpResponse::Ok().json(
        state
            .postgres
            .get_all_tracks_for_network_cohort(path.network_id, path.cohort_number)
            .await?,
    ))
}
