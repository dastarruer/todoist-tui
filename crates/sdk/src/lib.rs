pub mod command;
pub mod error;
pub mod types;

use crate::{
    command::{Command, CommandArgs},
    error::Result,
    types::{project::Project, task::Task},
};

use std::{collections::HashMap, num::NonZeroU16, str::FromStr, time::Duration};

use reqwest::{Client, Url};
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

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

    /// Creates a new client with a sync key.
    ///
    /// External consumers should store the sync key somewhere permanent and
    /// initialize `APIClient` with this method if it is necessary to use
    /// incremental sync across restarts.
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

    /// Sends commands to write resources to the Todoist API. The `APIClient`'s
    /// internal sync key will be automatically updated if a sync token is
    /// returned.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - An error occurs while sending the request.
    /// - An error status code (`400-599`) is returned.
    ///
    /// It is possible for commands to fail on the server. These error values
    /// will be provided in the returned [`SyncWriteOutput`] value. as simply
    /// returning an `Err` if simply one command fails would not
    /// indicate that other commands might have succeeded.
    ///
    /// In other words, even if this method returns `Ok`, make sure to check
    /// [`SyncWriteOutput::errors`] for info on failed commands.
    pub async fn send<T: CommandArgs + Serialize + Sync>(
        &mut self,
        commands: &[Command<T>],
    ) -> Result<SyncWriteOutput> {
        let commands = serde_json::to_string(commands)?;
        let data = [("commands", commands)];
        let resp = serde_json::from_str::<SyncWriteResponse>(
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

        if let Some(sync_token) = resp.sync_token {
            self.sync_key = SyncKey::from(sync_token);
        }

        let errors = resp
            .sync_status
            .values()
            .filter_map(|r| r.as_ref().err())
            .cloned()
            .collect::<Vec<SyncWriteError>>();
        let output = SyncWriteOutput {
            temp_id_mapping: resp.temp_id_mapping,
            errors,
        };

        Ok(output)
    }

    fn sync_url() -> Url {
        Url::from_str("https://api.todoist.com/api/v1/sync").expect("sync API URL should be valid")
    }
}

/// Stores the sync key the Todoist sync API uses to enable incremental sync.
///
/// External consumers should store this somewhere permanent if it is necessary
/// to use incremental sync across restarts.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SyncKey(String);

impl From<String> for SyncKey {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for SyncKey {
    fn from(value: &str) -> Self {
        Self::from(String::from(value))
    }
}

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
    /// Whether the response is a full sync or an incremental sync.
    #[serde(default)]
    pub full_sync: bool,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncWriteOutput {
    /// A dictionary object that maps temporary resource IDs to real resource
    /// IDs.
    ///
    /// For more info on temporary IDs, see [`command::Command::temp_id`].
    pub temp_id_mapping: Option<HashMap<Uuid, String>>,
    /// A collection of all errors returned by commands.
    pub errors: Vec<SyncWriteError>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct SyncWriteResponse {
    #[serde(deserialize_with = "deserialize_sync_status")]
    sync_status: HashMap<String, std::result::Result<String, SyncWriteError>>,
    sync_token: Option<String>,
    temp_id_mapping: Option<HashMap<Uuid, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SyncWriteError {
    /// A machine-readable error identifier (e.g. `INVALID_ARGUMENT_VALUE`).
    pub error_tag: String,
    /// A numeric error code.
    pub error_code: u16,
    /// A human-readable error message.
    pub error: String,
    /// The HTTP status code associated with this error. This code may differ from the REST HTTP status for [`Self::error_tag`].
    pub http_code: NonZeroU16, // Should be a reqwest::StatusCode, but it doesn't implement serde traits
    /// Additional context about the error.
    pub error_extra: SyncWriteErrorExtra,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SyncWriteErrorExtra {
    /// The name of the argument that caused the error.
    pub argument: Option<String>,
    /// A detailed error description, included when it provides more context than the generic [`SyncWriteError::error`] message.
    pub explanation: Option<String>,
    /// Seconds to wait before retrying. May be returned on rate-limited
    /// requests and other API errors, not just `429` responses.
    pub retry_after: Option<u32>,
    /// The workspace ID related to the error.
    pub workspace_id: Option<u32>,
    /// The limit that was exceeded (for limit-related errors).
    pub max_count: Option<u32>,
    /// An event ID for error tracking/support purposes.
    pub event_id: Option<i32>,
    /// The project ID related to the error.
    pub project_id: Option<String>,
    /// The section ID related to the error.
    pub section_id: Option<String>,
    // TODO: add bad_item field
}

fn deserialize_sync_status<'de, D>(
    deserializer: D,
) -> std::result::Result<HashMap<String, std::result::Result<String, SyncWriteError>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Raw {
        Ok(String),
        Err(SyncWriteError),
    }

    let raw: HashMap<String, Raw> = HashMap::deserialize(deserializer)?;
    Ok(raw
        .into_iter()
        .map(|(k, v)| {
            let result = match v {
                Raw::Ok(s) => Ok(s),
                Raw::Err(e) => Err(e),
            };
            (k, result)
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_write_error() {
        let json = include_str!("./fixtures/write_error.json");
        let result = serde_json::from_str::<SyncWriteResponse>(json)
            .expect("json should be successfully deserialized");

        let expected_err = SyncWriteError {
            error_tag: String::from("INVALID_ARGUMENT_VALUE"),
            error_code: 20,
            error: String::from("Invalid argument value"),
            http_code: NonZeroU16::new(400).expect("400 is a non-zero u16"),
            error_extra: SyncWriteErrorExtra {
                argument: Some(String::from("file_url")),
                explanation: Some(String::from("file_url contains disallowed URL")),
                retry_after: None,
                workspace_id: None,
                max_count: None,
                event_id: None,
                project_id: None,
                section_id: None,
            },
        };
        let mut map: HashMap<String, std::result::Result<String, SyncWriteError>> = HashMap::new();
        map.insert(
            String::from("bec5b356-3cc1-462a-9887-fe145e3e1ebf"),
            Err(expected_err),
        );

        let expected = SyncWriteResponse {
            sync_status: map,
            sync_token: None,
            temp_id_mapping: None,
        };

        pretty_assertions::assert_eq!(result, expected);
    }
}
