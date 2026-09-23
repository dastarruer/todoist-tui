pub mod error;
pub mod types;

use crate::{
    error::Result,
    types::task::{Task, TasksResponse},
};

use std::{str::FromStr, time::Duration};

use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

pub struct APIClient {
    pub key: String,
    pub sync_key: SyncKey,
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
        let sync_key = SyncKey::default();
        Self {
            key,
            sync_key,
            client,
        }
    }

    /// Creates a new client with a sync key. If you plan on syncing across
    /// restarts, you should store the sync key somewhere permanent and
    /// initialize an `APIClient` using this method.
    #[must_use]
    pub fn new_with_sync_key(key: String, sync_key: String) -> Self {
        #[expect(clippy::missing_panics_doc, reason = "infallible")]
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(10))
            .build()
            .expect("client config should be valid");
        let sync_key = SyncKey(sync_key);
        Self {
            key,
            sync_key,
            client,
        }
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

/// Stores the sync key the Todoist sync API uses to enable incremental sync.
///
/// External consumers should store this somewhere permanent if you plan to use
/// incremental sync across restarts.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SyncKey(String);

impl SyncKey {
    /// Retrieve the sync key.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.0
    }
}

impl Default for SyncKey {
    fn default() -> Self {
        Self(String::from("*"))
    }
}
