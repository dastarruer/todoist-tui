pub mod project;
pub mod task;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Command<T> {
    #[serde(rename = "type")]
    kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    temp_id: Option<Uuid>,
    args: T,
    uuid: Uuid,
}

impl<T: CommandArgs> Command<T> {
    #[must_use]
    pub fn new(cmd: T) -> Self {
        let uuid = Uuid::new_v4();
        let temp_id = if T::CREATES_RESOURCE {
            Some(Uuid::new_v4())
        } else {
            None
        };

        Self {
            kind: String::from(T::TYPE),
            temp_id,
            args: cmd,
            uuid,
        }
    }
}

impl<T> Command<T> {
    /// Temporary resource ID. Only specified for commands that create a new
    /// resource.
    ///
    /// An example of how temporary IDs can be used and referenced:
    ///
    /// ```rust
    /// # use todoist_sdk::command::{Command, project::AddProject, task::AddTask};
    /// # use todoist_sdk::types::Id;
    /// let project = Command::new(AddProject::default());
    /// let temp_id = project.temp_id().expect("project should have a `temp_id` field");
    ///
    /// let task = Command::new(AddTask {
    ///     content: String::from("Buy Milk"),
    ///     project_id: Some(temp_id.into()),
    ///     ..Default::default()
    /// });
    /// ```
    ///
    /// Here, a task is added to the new project by referencing its `temp_id`
    /// before it is actually created through the Todoist API. This is a good
    /// way to batch commands that rely on non-existent resources that are
    /// soon-to-be-created.
    pub const fn temp_id(&self) -> Option<Uuid> {
        self.temp_id
    }
}

/// Trait for structs that can be send as Todoist commands.
pub trait CommandArgs {
    /// The command string to be sent to the Todoist API, e.g. `"item_move"`,
    /// `"item_add"`, etc.
    const TYPE: &str;
    /// Commands that create resources will require a `temp_id` when sending to
    /// the Todoist API.
    const CREATES_RESOURCE: bool;
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::command::task::{AddTask, CloseTask, DeleteTask};

    use super::*;

    #[test]
    fn creating_command_attaches_a_temp_id() {
        let cmd = Command::new(AddTask::default());
        assert!(cmd.temp_id.is_some());
    }

    #[test]
    fn non_creating_command_has_no_temp_id() {
        let cmd = Command::new(DeleteTask::default());
        assert!(cmd.temp_id.is_none());
    }

    #[test]
    fn temp_id_is_omitted_from_json_when_absent() {
        let cmd = Command::new(DeleteTask::default());
        let value = serde_json::to_value(&cmd).unwrap();
        assert!(
            value.get("temp_id").is_none(),
            "temp_id should be skipped, not null"
        );
    }

    #[test]
    fn temp_id_is_present_in_json_when_set() {
        let cmd = Command::new(AddTask::default());
        let value = serde_json::to_value(&cmd).unwrap();
        assert!(value.get("temp_id").is_some());
    }

    #[test]
    fn serialized_command_has_correct_type_tag() {
        let cmd = Command::new(CloseTask::default());
        let value = serde_json::to_value(&cmd).unwrap();
        pretty_assertions::assert_eq!(value["type"], json!("item_close"));
    }

    #[test]
    fn serialized_command_nests_args_under_args_key() {
        let cmd = Command::new(DeleteTask::default());
        let value = serde_json::to_value(&cmd).unwrap();
        assert!(value.get("args").is_some());
        assert!(value["args"].get("id").is_some());
    }

    #[test]
    fn every_serialized_command_includes_a_valid_uuid() {
        let cmd = Command::new(CloseTask::default());
        let value = serde_json::to_value(&cmd).unwrap();
        let uuid_str = value["uuid"]
            .as_str()
            .expect("uuid should serialize as a string");
        assert!(Uuid::parse_str(uuid_str).is_ok());
    }
}
