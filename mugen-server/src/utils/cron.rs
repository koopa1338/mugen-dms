use std::path::Path;

use tokio::task::JoinHandle;

use tokio_schedule::{every, Job};
use tracing_attributes::instrument;

#[instrument]
pub fn scanner_cron() -> JoinHandle<()> {
    let scanner = every(
        dotenv::var("scanner_interval")
            .map_or_else(|error| 30, |value| value.parse::<u32>().expect("no valid number")),
    )
    .seconds()
    .perform(|| async { tracing::debug!("checking scanned folder (unimplemented)") });
    tokio::spawn(scanner)
}
