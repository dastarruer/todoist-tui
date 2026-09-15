use std::{env::var, fmt::Display, str::FromStr};

use reqwest::{Client, Url};

pub struct APIClient {
    pub key: TodoistAPIKey,
    client: Client,
}

impl APIClient {
    pub fn new(key: TodoistAPIKey) -> Self {
        let client = Client::new();
        Self { client, key }
    }

    fn base_url() -> Url {
        Url::from_str("https://api.todoist.com").expect("base API URL should be valid")
    }

    pub async fn tasks(&self) -> anyhow::Result<String> {
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
pub struct TodoistAPIKey(String);

impl Display for TodoistAPIKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn retrieve_api_key() -> anyhow::Result<TodoistAPIKey> {
    Ok(TodoistAPIKey(var("API_KEY")?))
}
