use crate::postgres::PostgreSQLStorage;
use dv_report_types::governance::referendum::{Referendum, ReferendumStatus};
use sqlx::{Postgres, Transaction};

impl PostgreSQLStorage {
    pub async fn get_referendum_count(&self, network_id: u32) -> anyhow::Result<u64> {
        let row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(DISTINCT (network_id, index))
            FROM referendum
            WHERE network_id = $1
            "#,
        )
        .bind(network_id as i32)
        .fetch_one(&self.connection_pool)
        .await?;
        Ok(row.0 as u64)
    }

    pub async fn update_referendum_status(
        &self,
        network_id: u32,
        referendum_index: u32,
        status: ReferendumStatus,
    ) -> anyhow::Result<bool> {
        let maybe_result: Option<(i32, i32)> = sqlx::query_as(
            r#"
            UPDATE referendum SET status_id = $1
            WHERE network_id = $2 AND index = $3
            RETURNING (network_id, index)
            "#,
        )
        .bind(status.id() as i32)
        .bind(network_id as i32)
        .bind(referendum_index as i32)
        .fetch_optional(&self.connection_pool)
        .await?;
        Ok(maybe_result.is_some())
    }

    pub async fn save_referendum(
        &self,
        referendum: &Referendum,
        tx: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO referendum (network_id, index, track_id, submission_block_hash, status_id)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT(network_id, index) DO UPDATE
            SET
                status_id = EXCLUDED.status_id,
                updated_at = now()
            RETURNING (network_id, index)
            "#,
        )
        .bind(referendum.network_id as i32)
        .bind(referendum.index as i32)
        .bind(referendum.track.id() as i32)
        .bind(referendum.submission_block.hash.as_str())
        .bind(referendum.status.id() as i32)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    pub async fn referendum_exists(
        &self,
        network_id: u32,
        index: u32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<bool> {
        let record_count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(DISTINCT(network_id, index)) FROM referendum
            WHERE network_id = $1 AND index = $2
            "#,
        )
        .bind(network_id as i32)
        .bind(index as i32)
        .fetch_one(&mut **tx)
        .await?;
        Ok(record_count.0 > 0)
    }
}
