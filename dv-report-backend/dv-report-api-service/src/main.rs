use dv_report_api_service::APIService;
use dv_report_service::Service;

#[tokio::main]
async fn main() {
    let service = APIService::default();
    if let Err(e) = service.start().await {
        log::error!("Startup failed: {e:?}");
        std::process::exit(1);
    }
}
