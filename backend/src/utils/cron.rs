use tokio::task::JoinHandle;
use tracing_attributes::instrument;

#[allow(clippy::async_yields_async)]
#[instrument]
pub async fn scanner_cron() -> JoinHandle<()> {
    let interval: u64 = dotenv::var("SCANNER_INTERVAL").map_or(30, |v| v.parse().unwrap_or(30));

    
    tokio::spawn(async move {
        loop {
            tracing::debug!("checking scanned folder (unimplemented)");
            tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
        }
    })
}
