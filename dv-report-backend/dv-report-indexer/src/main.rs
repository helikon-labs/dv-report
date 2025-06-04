use dv_report_indexer::Indexer;
use dv_report_service::Service;
use once_cell::sync::OnceCell;

static SERVICE: OnceCell<Indexer> = OnceCell::new();

#[tokio::main]
async fn main() {
    SERVICE
        .set(Indexer::default())
        .expect("Failed to set the global service.");
    SERVICE
        .get()
        .expect("Failed to initialize the service.")
        .start()
        .await;
}
