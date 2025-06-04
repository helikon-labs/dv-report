use dv_report_api_service::APIService;
use dv_report_service::Service;
use once_cell::sync::OnceCell;

static SERVICE: OnceCell<APIService> = OnceCell::new();

#[tokio::main]
async fn main() {
    SERVICE
        .set(APIService::default())
        .expect("Failed to set the global service.");
    SERVICE
        .get()
        .expect("Failed to initialize service.")
        .start()
        .await;
}
