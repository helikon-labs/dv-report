use dv_report_indexer::Indexer;
use dv_report_service::Service;
use once_cell::sync::OnceCell;

static SERVICE: OnceCell<Indexer> = OnceCell::new();

#[tokio::main]
async fn main() {
    let _ = SERVICE.set(Indexer::new().await.unwrap());
    SERVICE.get().unwrap().start().await;
}
