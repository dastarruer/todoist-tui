pub mod error;
pub mod types;

use crate::{
    error::Result,
    types::{project::Project, task::Task},
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

    /// Retrieves sync data from the Todoist API. The `APIClient`'s internal
    /// sync key will be automatically updated upon a successful sync.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - An error occurs while sending the request.
    /// - An error status code (`400-599`) is returned.
    /// - The response text cannot be parsed into a `SyncResponse`.
    pub async fn sync(&mut self, resource_types: Vec<ResourceType>) -> Result<SyncResponse> {
        let resource_types = serde_json::to_string(&resource_types)?;
        let data = [
            ("sync_token", self.sync_key.key()),
            ("resource_types", &resource_types),
        ];

        let resp = serde_json::from_str::<SyncResponse>(
            &self
                .client
                .post(Self::sync_url())
                .bearer_auth(&self.key)
                .form(&data)
                .send()
                .await?
                .error_for_status()?
                .text()
                .await?,
        )?;
        self.sync_key = SyncKey(resp.sync_token.clone());

        Ok(resp)
    }

    /// Convenience method to retrieve all sync data. The `APIClient`'s internal
    /// sync key will be automatically updated upon a successful sync.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - An error occurs while sending the request.
    /// - An error status code (`400-599`) is returned.
    /// - The response text cannot be parsed into a `SyncResponse`.
    pub async fn sync_all(&mut self) -> Result<SyncResponse> {
        self.sync(vec![ResourceType::All]).await
    }

    fn sync_url() -> Url {
        Url::from_str("https://api.todoist.com/api/v1/sync").expect("sync API URL should be valid")
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

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SyncResponse {
    /// Same thing as 'tasks', but it's been renamed in the Todoist API for
    /// some reason.
    pub items: Option<Vec<Task>>,
    pub projects: Option<Vec<Project>>,
    sync_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum ResourceType {
    // only resource types supported as of now.
    // https://developer.todoist.com/api/v1/#tag/Sync/Overview/Read-resources
    All,
    /// Same thing as a 'task', but it's been renamed in the Todoist API for
    /// some reason.
    Items,
    Projects,
}
