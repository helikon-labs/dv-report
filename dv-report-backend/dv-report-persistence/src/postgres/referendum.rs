use crate::postgres::PostgreSQLStorage;
use dv_report_types::governance::referendum::Referendum;
use sqlx::{Postgres, Transaction};

impl PostgreSQLStorage {
    pub async fn get_referendum_count(&self) -> anyhow::Result<u64> {
        let row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(DISTINCT id)
            FROM referendum
            "#,
        )
        .fetch_one(&self.connection_pool)
        .await?;
        Ok(row.0 as u64)
    }

    pub async fn save_referendum(
        &self,
        referendum: &Referendum,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<i32> {
        let result: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO referendum (network_id, index, track, submission_block_number, referendum_status_id)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT(network_id, index) DO NOTHING
            RETURNING id
            "#,
        )
        .bind(referendum.network_id as i32)
        .bind(referendum.index as i32)
        .bind(referendum.track as i32)
        .bind(referendum.submission_block_number as i64)
        .bind(referendum.status.id() as i32)
        .fetch_one(&mut **transaction)
        .await?;
        Ok(result.0)
    }
}
