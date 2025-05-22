use crate::postgres::PostgreSQLStorage;
use dv_report_types::dv::delegation::Delegation;
use dv_report_types::substrate::account_id::AccountId;
use dv_report_types::substrate::chain::Chain;
use sqlx::FromRow;
use std::str::FromStr;

#[derive(Debug, FromRow)]
struct DelegationRow {
    pub id: i32,
    pub cohort_number: i32,
    pub network_id: i32,
    pub delegator_account_id: String,
    pub delegate_id: String,
    pub delegate_account_id: String,
    pub delegation_start_block_number: i64,
    pub delegation_start_block_hash: String,
    pub delegation_start_extrinsic_hash: String,
    pub delegation_start_extrinsic_index: i32,
    pub delegation_end_block_number: Option<i64>,
    pub delegation_end_block_hash: Option<String>,
    pub delegation_end_extrinsic_hash: Option<String>,
    pub delegation_end_extrinsic_index: Option<i32>,
}

fn delegation_row_into_delegation(row: &DelegationRow) -> anyhow::Result<Delegation> {
    Ok(Delegation {
        id: row.id as u32,
        cohort_number: row.cohort_number as u32,
        network: Chain::from_id(row.network_id as u32),
        delegator_account_id: AccountId::from_str(&row.delegator_account_id)?,
        delegate_id: row.delegate_id.clone(),
        delegate_account_id: AccountId::from_str(&row.delegate_account_id)?,
        delegation_start_block_number: row.delegation_start_block_number as u64,
        delegation_start_block_hash: row.delegation_start_block_hash.clone(),
        delegation_start_extrinsic_hash: row.delegation_start_extrinsic_hash.clone(),
        delegation_start_extrinsic_index: row.delegation_start_extrinsic_index as u32,
        delegation_end_block_number: row.delegation_end_block_number.map(|x| x as u64),
        delegation_end_block_hash: row.delegation_end_block_hash.clone(),
        delegation_end_extrinsic_hash: row.delegation_end_extrinsic_hash.clone(),
        delegation_end_extrinsic_index: row.delegation_end_extrinsic_index.map(|x| x as u32),
    })
}

impl PostgreSQLStorage {
    pub async fn get_delegation(
        &self,
        cohort_number: u32,
        network_id: u32,
        delegate_id: &str,
    ) -> anyhow::Result<Delegation> {
        let row: DelegationRow = sqlx::query_as::<_, DelegationRow>(
            r#"
            SELECT id, cohort_number, network_id, delegator_account_id, delegate_id, delegate_account_id, delegation_start_block_number, delegation_start_block_hash, delegation_start_extrinsic_hash, delegation_start_extrinsic_index, delegation_end_block_number, delegation_end_block_hash, delegation_end_extrinsic_hash, delegation_end_extrinsic_index
            FROM delegation
            WHERE cohort_number = $1 AND network_id = $2 AND delegate_id = $3
            "#,
        )
            .bind(cohort_number as i32)
            .bind(network_id as i32)
            .bind(delegate_id)
            .fetch_one(&self.connection_pool)
            .await?;
        delegation_row_into_delegation(&row)
    }
}
