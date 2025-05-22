use crate::postgres::PostgreSQLStorage;
use dv_report_types::dv::delegate::Delegate;
use dv_report_types::dv::delegation::Delegation;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct DelegateRow {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
    pub twitter: Option<String>,
}

fn delegate_row_into_delegate(row: &DelegateRow, delegation: Delegation) -> Delegate {
    Delegate {
        id: row.id.clone(),
        name: row.name.clone(),
        url: row.url.clone(),
        twitter: row.twitter.clone(),
        delegation,
    }
}

impl PostgreSQLStorage {
    pub async fn get_delegate_by_id(
        &self,
        id: &str,
        cohort_number: u32,
        network_id: u32,
    ) -> anyhow::Result<Option<Delegate>> {
        let maybe_row: Option<DelegateRow> = sqlx::query_as::<_, DelegateRow>(
            r#"
            SELECT id, name, url, twitter
            FROM delegate
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.connection_pool)
        .await?;
        if let Some(row) = maybe_row {
            let delegation = self
                .get_delegation(cohort_number, network_id, row.id.as_str())
                .await?;
            Ok(Some(delegate_row_into_delegate(&row, delegation)))
        } else {
            Ok(None)
        }
    }

    pub async fn get_all_delegates(
        &self,
        cohort_number: u32,
        network_id: u32,
    ) -> anyhow::Result<Vec<Delegate>> {
        let rows: Vec<DelegateRow> = sqlx::query_as::<_, DelegateRow>(
            "
            SELECT id, name, url, twitter
            FROM delegate
            ORDER BY name ASC
            ",
        )
        .fetch_all(&self.connection_pool)
        .await?;
        let mut result = Vec::new();
        for row in rows.iter() {
            let delegation = self
                .get_delegation(cohort_number, network_id, row.id.as_str())
                .await?;
            result.push(Delegate {
                id: row.id.clone(),
                name: row.name.clone(),
                url: row.url.clone(),
                twitter: row.twitter.clone(),
                delegation,
            })
        }
        Ok(result)
    }
}
