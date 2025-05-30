use actix_cors::Cors;
use actix_web::{dev::Service as _, web, App, HttpResponse, HttpServer};
use async_trait::async_trait;
use dv_report_config::Config;
use dv_report_persistence::postgres::PostgreSQLStorage;
use dv_report_service::err::InternalServerError;
use dv_report_service::Service;
use futures_util::future::FutureExt;
use lazy_static::lazy_static;
use std::sync::Arc;
use std::time::Instant;

mod metrics;
mod service;
mod types;

lazy_static! {
    static ref CONFIG: Config = Config::default();
}

pub(crate) type ResultResponse = Result<HttpResponse, InternalServerError>;

async fn on_server_ready() {
    log::info!("HTTP service started.");
}

#[derive(Clone)]
pub(crate) struct ServiceState {
    postgres: Arc<PostgreSQLStorage>,
}

#[derive(Default)]
pub struct APIService;

#[async_trait(?Send)]
impl Service for APIService {
    fn get_metrics_server_addr() -> (&'static str, u16) {
        (
            CONFIG.metrics.host.as_str(),
            CONFIG.metrics.api_service_port,
        )
    }

    async fn run(&'static self) -> anyhow::Result<()> {
        let postgres = Arc::new(PostgreSQLStorage::new(&CONFIG).await?);
        log::info!(
            "Starting HTTP service @ {}:{}.",
            CONFIG.api.service_host,
            CONFIG.api.api_service_port
        );
        let server = HttpServer::new(move || {
            let cors = Cors::default()
                .allowed_origin("http://localhost:8080")
                .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                .allowed_headers(vec![
                    actix_web::http::header::AUTHORIZATION,
                    actix_web::http::header::CONTENT_TYPE,
                ])
                .supports_credentials();

            App::new()
                .app_data(web::Data::new(ServiceState {
                    postgres: postgres.clone(),
                }))
                .wrap(cors)
                .wrap_fn(|request, service| {
                    metrics::request_counter().inc();
                    metrics::connection_count().inc();
                    let start = Instant::now();
                    service.call(request).map(move |result| {
                        match &result {
                            Ok(response) => {
                                let status_code = response.response().status();
                                metrics::response_time_ms()
                                    .observe(start.elapsed().as_millis() as f64);
                                metrics::response_status_code_counter(status_code.as_str()).inc();
                            }
                            Err(error) => {
                                let status_code = error.as_response_error().status_code();
                                metrics::response_time_ms()
                                    .observe(start.elapsed().as_millis() as f64);
                                metrics::response_status_code_counter(status_code.as_str()).inc();
                            }
                        }
                        metrics::connection_count().dec();
                        result
                    })
                })
                .service(service::get_all_networks)
                .service(service::get_all_referendum_statuses)
                .service(service::get_all_referendum_tracks)
                .service(service::get_all_cohorts)
                .service(service::get_all_network_cohort_tracks)
                .service(service::get_all_delegates)
                .service(service::get_network_referenda)
                .service(service::get_network_voter_votes)
        })
        .workers(10)
        .disable_signals()
        .bind(format!(
            "{}:{}",
            CONFIG.api.service_host, CONFIG.api.api_service_port,
        ))?
        .run();
        let (server_result, _) = tokio::join!(server, on_server_ready());
        Ok(server_result?)
    }
}
