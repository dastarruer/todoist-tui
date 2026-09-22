pub mod error;
pub mod types;

use crate::{
    error::Result,
    types::task::{Task, TasksResponse},
};

use std::{str::FromStr, time::Duration};

use reqwest::{Client, Url};

pub struct APIClient {
    pub key: String,
    client: Client,
}

impl APIClient {
    #[must_use]
    pub fn new(key: String) -> Self {
        #[expect(clippy::missing_panics_doc, reason = "infallible")]
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(10))
            .build()
            .expect("client config should be valid");
        Self { key, client }
    }

    fn base_url() -> Url {
        Url::from_str("https://api.todoist.com").expect("base API URL should be valid")
    }

    /// Gets all active tasks for the user.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - There was an error when sending a request.
    /// - Any status code between `400` and `599` was returned.
    /// - The response was not successfully decoded.
    pub async fn tasks(&self) -> Result<Vec<Task>> {
        let mut tasks = Vec::new();
        let mut cursor = Some(String::new());
        while cursor.is_some() {
            #[expect(clippy::missing_panics_doc, reason = "infallible")]
            let mut url = Self::base_url()
                .join("api/v1/tasks")
                .expect("joined URL should be valid");
            if let Some(c) = &cursor
                && !c.is_empty()
            {
                url.query_pairs_mut().append_pair("cursor", c);
            }

            let resp = serde_json::from_str::<TasksResponse>(
                &self
                    .client
                    .get(url)
                    .bearer_auth(&self.key)
                    .send()
                    .await?
                    .error_for_status()?
                    .text()
                    .await?,
            )?;
            tasks.extend(resp.results);
            cursor = resp.next_cursor;
        }
        Ok(tasks)
    }
}
