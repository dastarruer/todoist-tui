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
    /// An exmaple of how temporary IDs can be used and referenced:
    /// ```rs
    /// let add_cmd = Command::new_with
    /// ```
    // TODO: finish this
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
