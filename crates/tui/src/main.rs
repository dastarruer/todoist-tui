use std::path::PathBuf;

use flexi_logger::{FileSpec, Logger};
use todoist_sdk::APIClient;

#[tokio::main]
async fn main() {
    bootstrap_app();
    let key = todoist_sdk::retrieve_api_key().unwrap_or_else(|e| {
        log::error!("Error retrieving API key: {e}");
        std::process::exit(1);
    });
    let client = APIClient::new(key);
    let tasks = client.tasks().await;
    println!("{tasks:?}");
}

fn bootstrap_app() {
    let log_dir = std::env::var("XDG_STATE_HOME")
        .map_or_else(
            |_| {
                let home = std::env::var("HOME").expect("HOME is not set");
                PathBuf::from(home).join(".local/state")
            },
            PathBuf::from,
        )
        .join("todoist-tui");
    let _logger = Logger::try_with_env()
        .expect("Value of RUST_LOG is malformed")
        .log_to_file(
            FileSpec::default()
                .directory(log_dir)
                .basename("gtkshutdown"),
        )
        .duplicate_to_stdout(flexi_logger::Duplicate::Trace)
        .rotate(
            flexi_logger::Criterion::Size(1_000_000),
            flexi_logger::Naming::Numbers,
            flexi_logger::Cleanup::KeepLogFiles(5),
        )
        .start();
}
