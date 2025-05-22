use crate::postgres::PostgreSQLStorage;
use dv_report_types::dv::cohort::Cohort;
use dv_report_types::substrate::chain::Chain;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct CohortRow {
    pub number: i32,
    pub network_id: i32,
    pub announcement_date: NaiveDateTime,
    pub announcement_url: Option<String>,
    pub delegation_date: NaiveDateTime,
    pub start_block_number: i64,
}

fn cohort_row_into_cohort(row: &CohortRow) -> Cohort {
    Cohort {
        number: row.number as u32,
        network: Chain::from_id(row.network_id as u32),
        announcement_date: row.announcement_date,
        announcement_url: row.announcement_url.clone(),
        delegation_date: row.delegation_date,
        start_block_number: row.start_block_number as u64,
    }
}

impl PostgreSQLStorage {
    pub async fn get_cohort(&self, number: u32, network_id: u32) -> anyhow::Result<Cohort> {
        let row: CohortRow = sqlx::query_as::<_, CohortRow>(
            r#"
            SELECT number, network_id, announcement_date, announcement_url, delegation_date, start_block_number
            FROM cohort
            WHERE number = $1 AND network_id = $2
            "#,
        )
            .bind(number as i32)
            .bind(network_id as i32)
            .fetch_one(&self.connection_pool)
            .await?;
        Ok(cohort_row_into_cohort(&row))
    }
}
