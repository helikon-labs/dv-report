use dv_report_api_service::APIService;
use dv_report_service::Service;
use lazy_static::lazy_static;

lazy_static! {
    static ref SERVICE: APIService = APIService;
}

#[tokio::main]
async fn main() {
    SERVICE.start().await;
}
