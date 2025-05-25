use crate::postgres::PostgreSQLStorage;
use dv_report_types::runtime::api::runtime_types::pallet_conviction_voting::vote::AccountVote;
use dv_report_types::substrate::vote::{RemoveVoteCall, VoteCall};
use sqlx::{Postgres, Transaction};

impl PostgreSQLStorage {
    pub async fn save_vote_call(
        &self,
        vote_call: &VoteCall,
        tx: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<i32> {
        let (vote_type, is_aye, conviction, balance, aye, nay, abstain) = match &vote_call.vote {
            AccountVote::Standard { vote, balance } => (
                "standard",
                Some(vote.0 >= 80),
                Some(vote.0 % 10),
                Some(*balance),
                None,
                None,
                None,
            ),
            AccountVote::Split { aye, nay } => {
                ("split", None, None, None, Some(*aye), Some(*nay), None)
            }
            AccountVote::SplitAbstain { aye, nay, abstain } => (
                "split_abstain",
                None,
                None,
                None,
                Some(*aye),
                Some(*nay),
                Some(*abstain),
            ),
        };
        let result: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO vote (network_id, referendum_index, block_hash, extrinsic_index, extrinsic_hash, is_batch, is_multisig, is_proxy, signer_account_id, voter_account_id, vote_type, is_aye, conviction, balance, aye, nay, abstain)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            RETURNING id
            "#,
        )
            .bind(vote_call.network_id as i32)
            .bind(vote_call.referendum_index as i32)
            .bind(vote_call.block.hash.as_str())
            .bind(vote_call.extrinsic_index as i32)
            .bind(vote_call.extrinsic_hash.as_str())
            .bind(vote_call.is_batch)
            .bind(vote_call.is_multisig)
            .bind(vote_call.is_proxy)
            .bind(vote_call.signer.to_string())
            .bind(vote_call.voter.to_string())
            .bind(vote_type)
            .bind(is_aye)
            .bind(conviction.map(|c| c as i32))
            .bind(balance.map(|b| b.to_string()))
            .bind(aye.map(|a| a.to_string()))
            .bind(nay.map(|a| a.to_string()))
            .bind(abstain.map(|a| a.to_string()))
            .fetch_one(&mut **tx)
            .await?;
        Ok(result.0)
    }

    pub async fn save_remove_vote_call(
        &self,
        remove_vote_call: &RemoveVoteCall,
        tx: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<i32> {
        let result: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO remove_vote (network_id, referendum_index, block_hash, extrinsic_index, extrinsic_hash, is_batch, is_multisig, is_proxy, signer_account_id, voter_account_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id
            "#,
        )
            .bind(remove_vote_call.network_id as i32)
            .bind(remove_vote_call.referendum_index as i32)
            .bind(remove_vote_call.block.hash.as_str())
            .bind(remove_vote_call.extrinsic_index as i32)
            .bind(remove_vote_call.extrinsic_hash.as_str())
            .bind(remove_vote_call.is_batch)
            .bind(remove_vote_call.is_multisig)
            .bind(remove_vote_call.is_proxy)
            .bind(remove_vote_call.signer.to_string())
            .bind(remove_vote_call.voter.to_string())
            .fetch_one(&mut **tx)
            .await?;
        Ok(result.0)
    }
}
