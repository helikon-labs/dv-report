use crate::{ResultResponse, ServiceState};
use actix_web::{get, web, HttpResponse};

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
