use std::{env::var, fmt::Display, path::PathBuf, str::FromStr};

use flexi_logger::{FileSpec, Logger};
use reqwest::{Client, Url};

#[tokio::main]
async fn main() {
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
        .start()
        .expect("Logger failed to start");

    let key = retrieve_api_key().unwrap_or_else(|e| {
        log::error!("Error retrieving API key: {e}");
        std::process::exit(1);
    });
    let client = APIClient::new(key);
    let tasks = client.tasks().await;
    println!("{tasks:?}");
}

struct APIClient {
    client: Client,
    key: TodoistAPIKey,
}

impl APIClient {
    fn new(key: TodoistAPIKey) -> Self {
        let client = Client::new();
        Self { client, key }
    }

    fn base_url() -> Url {
        Url::from_str("https://api.todoist.com").expect("base API URL should be valid")
    }

    async fn tasks(&self) -> anyhow::Result<String> {
        let url = Self::base_url()
            .join("api/v1/tasks")
            .expect("joined URL should be valid");
        Ok(self
            .client
            .get(url)
            .bearer_auth(&self.key)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?)
    }
}

#[derive(Debug)]
struct TodoistAPIKey(String);

impl Display for TodoistAPIKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn retrieve_api_key() -> anyhow::Result<TodoistAPIKey> {
    Ok(TodoistAPIKey(var("API_KEY")?))
}
