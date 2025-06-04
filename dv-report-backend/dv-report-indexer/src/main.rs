use dv_report_indexer::Indexer;
use dv_report_service::Service;

#[tokio::main]
async fn main() {
    let service = Indexer::default();
    if let Err(e) = service.start().await {
        eprintln!("Startup failed: {e:?}");
        std::process::exit(1);
    }
}
