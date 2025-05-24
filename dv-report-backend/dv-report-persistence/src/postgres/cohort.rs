use crate::postgres::PostgreSQLStorage;
use dv_report_types::dv::cohort::CohortRow;

impl PostgreSQLStorage {
    pub async fn get_cohort(&self, network_id: u32, number: u32) -> anyhow::Result<CohortRow> {
        let row: CohortRow = sqlx::query_as::<_, CohortRow>(
            r#"
            SELECT network_id, number, announcement_date, announcement_url, delegation_date, start_block_hash
            FROM cohort
            WHERE network_id = $1 AND number = $2
            "#,
        )
            .bind(network_id as i32)
            .bind(number as i32)
            .fetch_one(&self.connection_pool)
            .await?;
        Ok(row)
    }
}
