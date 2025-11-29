use crate::postgres::PostgreSQLStorage;
use dv_report_types::governance::subsquare::SubsquareReferendumVote;
use sqlx::{Postgres, QueryBuilder};

impl PostgreSQLStorage {
    pub async fn save_subsquare_referendum_votes(
        &self,
        network_id: u32,
        referendum_index: u32,
        votes: &[SubsquareReferendumVote],
    ) -> anyhow::Result<()> {
        for vote_chunk in votes.chunks(1000) {
            let mut query_builder = QueryBuilder::new(
                "INSERT INTO subsquare_referendum_vote(network_id, referendum_index, account_id, delegate_account_id, is_standard, is_split, is_split_abstain, balance, aye, conviction, abstain_balance, abstain_votes, aye_balance, aye_votes, nay_balance, nay_votes, votes, delegated_votes, delegated_capital, query_at) ",
            );
            query_builder.push_values(vote_chunk, |mut query, vote| {
                query
                    .push_bind(network_id as i32)
                    .push_bind(referendum_index as i32)
                    .push_bind(vote.account_id.to_string())
                    .push_bind(
                        vote.delegate_account_id
                            .map(|account_id| account_id.to_string()),
                    )
                    .push_bind(vote.is_standard)
                    .push_bind(vote.is_split)
                    .push_bind(vote.is_split_abstain)
                    .push_bind(vote.balance.as_deref())
                    .push_bind(vote.aye)
                    .push_bind(vote.conviction as i32)
                    .push_bind(vote.abstain_balance.as_deref())
                    .push_bind(vote.abstain_votes.as_deref())
                    .push_bind(vote.aye_balance.as_deref())
                    .push_bind(vote.aye_votes.as_deref())
                    .push_bind(vote.nay_balance.as_deref())
                    .push_bind(vote.nay_votes.as_deref())
                    .push_bind(vote.votes.as_deref())
                    .push_bind(
                        vote.delegations
                            .as_ref()
                            .map(|delegation| delegation.votes.as_str()),
                    )
                    .push_bind(
                        vote.delegations
                            .as_ref()
                            .map(|delegation| delegation.capital.as_str()),
                    )
                    .push_bind(vote.query_at as i64);
            });
            query_builder.push(
                r#"
                ON CONFLICT(network_id, referendum_index, account_id) DO UPDATE
                SET
                    delegate_account_id = EXCLUDED.delegate_account_id,
                    is_standard = EXCLUDED.is_standard,
                    is_split = EXCLUDED.is_split,
                    is_split_abstain = EXCLUDED.is_split_abstain,
                    balance = EXCLUDED.balance,
                    aye = EXCLUDED.aye,
                    conviction = EXCLUDED.conviction,
                    abstain_balance = EXCLUDED.abstain_balance,
                    abstain_votes = EXCLUDED.abstain_votes,
                    aye_balance = EXCLUDED.aye_balance,
                    aye_votes = EXCLUDED.aye_votes,
                    nay_balance = EXCLUDED.nay_balance,
                    nay_votes = EXCLUDED.nay_votes,
                    votes = EXCLUDED.votes,
                    delegated_votes = EXCLUDED.delegated_votes,
                    delegated_capital = EXCLUDED.delegated_capital,
                    query_at = EXCLUDED.query_at
                "#,
            );
            let query: sqlx::query::Query<'_, Postgres, sqlx::postgres::PgArguments> =
                query_builder.build();
            query.execute(&self.connection_pool).await?;
        }
        Ok(())
    }

    pub async fn save_subsquare_referendum_vote(
        &self,
        network_id: u32,
        referendum_index: u32,
        vote: &SubsquareReferendumVote,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO subsquare_referendum_vote(network_id, referendum_index, account_id, delegate_account_id, is_standard, is_split, is_split_abstain, balance, aye, conviction, abstain_balance, abstain_votes, aye_balance, aye_votes, nay_balance, nay_votes, votes, delegated_votes, delegated_capital, query_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)
            ON CONFLICT(network_id, referendum_index, account_id) DO UPDATE
            SET
                delegate_account_id = EXCLUDED.delegate_account_id,
                is_standard = EXCLUDED.is_standard,
                is_split = EXCLUDED.is_split,
                is_split_abstain = EXCLUDED.is_split_abstain,
                balance = EXCLUDED.balance,
                aye = EXCLUDED.aye,
                conviction = EXCLUDED.conviction,
                abstain_balance = EXCLUDED.abstain_balance,
                abstain_votes = EXCLUDED.abstain_votes,
                aye_balance = EXCLUDED.aye_balance,
                aye_votes = EXCLUDED.aye_votes,
                nay_balance = EXCLUDED.nay_balance,
                nay_votes = EXCLUDED.nay_votes,
                votes = EXCLUDED.votes,
                delegated_votes = EXCLUDED.delegated_votes,
                delegated_capital = EXCLUDED.delegated_capital,
                query_at = EXCLUDED.query_at
            "#,
        )
            .bind(network_id as i32)
            .bind(referendum_index as i32)
            .bind(vote.account_id.to_string())
            .bind(vote.delegate_account_id.map(|account_id| account_id.to_string()))
            .bind(vote.is_standard)
            .bind(vote.is_split)
            .bind(vote.is_split_abstain)
            .bind(vote.balance.as_deref())
            .bind(vote.aye)
            .bind(vote.conviction as i32)
            .bind(vote.abstain_balance.as_deref())
            .bind(vote.abstain_votes.as_deref())
            .bind(vote.aye_balance.as_deref())
            .bind(vote.aye_votes.as_deref())
            .bind(vote.nay_balance.as_deref())
            .bind(vote.nay_votes.as_deref())
            .bind(vote.votes.as_deref())
            .bind(vote.delegations.as_ref().map(|delegation| delegation.votes.as_str()))
            .bind(vote.delegations.as_ref().map(|delegation| delegation.capital.as_str()))
            .bind(vote.query_at as i64)
            .execute(&self.connection_pool)
            .await?;
        Ok(())
    }
}
