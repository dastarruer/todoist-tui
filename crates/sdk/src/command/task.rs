use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::{
    command::CommandArgs,
    types::{
        Id, Uid,
        task::{Deadline, DueDate, TaskDuration},
    },
};

/// Add a task.
#[derive(Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct AddTask {
    /// The text of the task. This value may contain markdown-formatted text
    /// and hyperlinks.
    pub content: String,
    /// A description for the task. This value may contain markdown-formatted
    /// text and hyperlinks.
    pub description: Option<String>,
    /// The ID of the project to add the task to (a number or a temp id). By
    /// default the task is added to the user’s `Inbox` project.
    pub project_id: Option<Id>,
    /// The due date of the task.
    pub due: Option<DueDate>,
    /// The deadline of the task.
    pub deadline: Option<Deadline>,
    /// The priority of the task (a number between `1` and
    /// `4`, `4` for very urgent and `1` for natural).
    ///
    /// **Note:** Keep in mind that very urgent is the
    /// priority `1` on clients. So, `p1` will return `4` in
    /// the API.
    pub priority: Option<u8>,
    /// The ID of the parent task. Set to `None` for root tasks.
    pub parent_id: Option<String>,
    /// The order of task. Defines the position of the task among all the tasks
    /// with the same parent.
    pub child_order: Option<i32>,
    /// Fractional-indexing order key for the new task. When omitted, the
    /// backend generates a key placing the task at the bottom of the list of
    /// siblings sharing the same project, section and parent task. If a
    /// sibling already uses the requested key, the backend stores an adjusted
    /// key placing the task immediately after that position; the corrected
    /// value is returned via sync.
    pub order_key: Option<String>,
    /// The ID of the section. Set to `None` for tasks not belonging to a
    /// section.
    pub section_id: Option<Id>,
    /// The order of the task inside the Today or Next 7 days view. Smaller
    /// values place the task nearer the top.
    pub day_order: Option<i32>,
    /// Whether the task's sub-tasks are collapsed.
    pub is_collapsed: Option<bool>,
    /// The task's labels (a list of names that may represent either personal
    /// or shared labels).
    pub labels: Option<Vec<String>>,
    /// The ID of user who assigns the current task. This makes sense for
    /// shared projects only. Accepts `0` or any user ID from the list of
    /// project collaborators. If this value is unset or invalid, it will be
    /// automatically setup to your UID.
    pub assigned_by_uid: Option<Uid>,
    /// The ID of user who is responsible for accomplishing the current task.
    /// This makes sense for shared projects only. Accepts any user ID from the
    /// list of project collaborators or `None` or an empty string to unset.
    pub responsible_uid: Option<Uid>,
    /// When this option is enabled, the default reminder will be added to the
    /// new item if it has a due date with time set. See also the
    /// [`auto_reminder`](https://developer.todoist.com/api/v1/#tag/Sync/User)
    /// user option for more info about the default reminder.
    pub auto_reminder: Option<bool>,
    /// Whether the labels should be parsed from the task content.
    pub auto_parse_labels: Option<bool>,
    /// The task's duration.
    pub duration: Option<TaskDuration>,
}

impl CommandArgs for AddTask {
    const TYPE: &str = "item_add";
    const CREATES_RESOURCE: bool = true;
}

/// Updates task attributes.
///
/// Please note that updating the parent, moving, completing or uncompleting
/// tasks is not supported by `UpdateTask`. More specific commands have to be
/// used instead.
#[derive(Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct UpdateTask {
    /// The ID of the task.
    pub id: Id,
    /// The text of the task. This value may contain markdown-formatted text
    /// and hyperlinks.
    pub content: Option<String>,
    /// A description for the task. This value may contain markdown-formatted
    /// text and hyperlinks.
    pub description: Option<String>,
    /// The due date of the task.
    pub due: Option<DueDate>,
    /// The deadline of the task.
    pub deadline: Option<Deadline>,
    /// The priority of the task (a number between `1` and
    /// `4`, `4` for very urgent and `1` for natural).
    ///
    /// **Note:** Keep in mind that very urgent is the
    /// priority `1` on clients. So, `p1` will return `4` in
    /// the API.
    pub priority: Option<u8>,
    /// Whether the task's sub-tasks are collapsed.
    pub is_collapsed: Option<bool>,
    /// The task's labels (a list of names that may represent either personal
    /// or shared labels).
    pub labels: Option<Vec<String>>,
    /// The ID of user who assigns the current task. This makes sense for
    /// shared projects only. Accepts `0` or any user ID from the list of
    /// project collaborators. If this value is unset or invalid, it will be
    /// automatically setup to your UID.
    pub assigned_by_uid: Option<Uid>,
    /// The ID of user who is responsible for accomplishing the current task.
    /// This makes sense for shared projects only. Accepts any user ID from the
    /// list of project collaborators or `None` or an empty string to unset.
    pub responsible_uid: Option<Uid>,
    /// The order of the task inside the Today or Next 7 days view. Smaller
    /// values place the task nearer the top.
    pub day_order: Option<i32>,
    /// The task's duration.
    pub duration: Option<TaskDuration>,
    /// Fractional-indexing order key for the new task. When omitted, the
    /// backend generates a key placing the task at the bottom of the list of
    /// siblings sharing the same project, section and parent task. If a
    /// sibling already uses the requested key, the backend stores an adjusted
    /// key placing the task immediately after that position; the corrected
    /// value is returned via sync.
    pub order_key: Option<String>,
}

impl CommandArgs for UpdateTask {
    const TYPE: &str = "item_update";
    const CREATES_RESOURCE: bool = false;
}

/// Move task to a different location.
///
/// Only one of `parent_id`, `section_id` or `project_id` must be set.
#[derive(Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct MoveTask {
    /// The ID of the task.
    pub id: Id,
    /// ID of the destination parent task. The task becomes the last child task
    /// of the parent task.
    pub parent_id: Option<Id>,
    /// ID of the destination section. The task becomes the last root task of
    /// the section.
    pub section_id: Option<Id>,
    /// ID of the destination project. The task becomes the last root task of
    /// the project.
    pub project_id: Option<Id>,
}

impl CommandArgs for MoveTask {
    const TYPE: &str = "item_move";
    const CREATES_RESOURCE: bool = false;
}

/// Delete a task and all its sub-tasks.
#[derive(Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DeleteTask {
    /// ID of the task to delete.
    pub id: Id,
}

impl CommandArgs for DeleteTask {
    const TYPE: &str = "item_delete";
    const CREATES_RESOURCE: bool = false;
}

/// Completes a task and its sub-tasks and moves them to the archive. See also
/// `CloseTask` for a simplified version of the command.
#[derive(Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct CompleteTask {
    /// Task ID to complete.
    pub id: Id,
    /// Date of completion of the task. If not set, the server will set the
    /// value to the current timestamp.
    pub date_completed: Option<DateTime<Utc>>,
    /// If `true`, skips incrementing completion stats. Used when restoring
    /// task state after undoing a completion.
    pub from_undo: Option<bool>,
}

impl CommandArgs for CompleteTask {
    const TYPE: &str = "item_complete";
    const CREATES_RESOURCE: bool = false;
}

/// (Un)completes and restores a completed task.
///
/// Any ancestor items or sections will also be reinstated. Items will have the
/// checked value reset.
///
/// The reinstated items and sections will appear at the end of the list within
/// their parent, after any previously active tasks.
#[derive(Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct UncompleteTask {
    /// Task ID to uncomplete.
    pub id: Id,
}

impl CommandArgs for UncompleteTask {
    const TYPE: &str = "item_uncomplete";
    const CREATES_RESOURCE: bool = false;
}

/// Complete a recurring task.
///
/// The reason why this is a special case is because we need to mark a
/// recurring completion (and using `UpdateTask` won't do this). See also
/// `CloseTask` for a simplified version of the command.
#[derive(Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct CompleteRecurringTask {
    /// Task ID to complete.
    pub id: Id,
    /// The due date of the task.
    pub due: Option<DueDate>,
    /// Set this to `true` for completion, or `false` for uncompletion (e.g.
    /// via undo). If omitted, this argument is set to `true`.
    pub is_forward: Option<bool>,
    /// Set this property to `true` to reset subtasks when a recurring task is
    /// completed. By default, this property is not set (`false`), and subtasks
    /// will retain their existing status when the parent task recurs.
    pub reset_subtasks: Option<bool>,
}

impl CommandArgs for CompleteRecurringTask {
    const TYPE: &str = "item_update_date_complete";
    const CREATES_RESOURCE: bool = false;
}

/// A simplified version of `CompleteTask` / `CompleteRecurringTask`.
///
/// The command does exactly what official clients do when you close a task:
/// regular tasks are completed and moved to the archive, recurring tasks are
/// scheduled to their next occurrence.
#[derive(Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct CloseTask {
    /// The ID of the task to close.
    pub id: Id,
}

impl CommandArgs for CloseTask {
    const TYPE: &str = "item_close";
    const CREATES_RESOURCE: bool = false;
}

/// Update the day orders of multiple tasks at once.
///
/// Unlike `UpdateTask`, this is meant to be used for multiple tasks, rather
/// than just one.
#[derive(Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct UpdateTaskDayOrders {
    /// A mapping where each key is a task ID and each value is a `day_order`.
    pub ids_to_orders: HashMap<Id, i32>,
}

impl CommandArgs for UpdateTaskDayOrders {
    const TYPE: &str = "item_update_day_orders";
    const CREATES_RESOURCE: bool = false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_types() {
        pretty_assertions::assert_eq!(AddTask::TYPE, "item_add");
        pretty_assertions::assert_eq!(UpdateTask::TYPE, "item_update");
        pretty_assertions::assert_eq!(MoveTask::TYPE, "item_move");
        pretty_assertions::assert_eq!(DeleteTask::TYPE, "item_delete");
        pretty_assertions::assert_eq!(CompleteTask::TYPE, "item_complete");
        pretty_assertions::assert_eq!(UncompleteTask::TYPE, "item_uncomplete");
        pretty_assertions::assert_eq!(CompleteRecurringTask::TYPE, "item_update_date_complete");
        pretty_assertions::assert_eq!(CloseTask::TYPE, "item_close");
        pretty_assertions::assert_eq!(UpdateTaskDayOrders::TYPE, "item_update_day_orders");
    }

    #[test]
    fn creates_resource() {
        const {
            assert!(AddTask::CREATES_RESOURCE);
            assert!(!UpdateTask::CREATES_RESOURCE);
            assert!(!MoveTask::CREATES_RESOURCE);
            assert!(!DeleteTask::CREATES_RESOURCE);
            assert!(!CompleteTask::CREATES_RESOURCE);
            assert!(!UncompleteTask::CREATES_RESOURCE);
            assert!(!CompleteRecurringTask::CREATES_RESOURCE);
            assert!(!CloseTask::CREATES_RESOURCE);
            assert!(!UpdateTaskDayOrders::CREATES_RESOURCE);
        }
    }

    #[test]
    fn update_task_day_orders_holds_multiple_mappings() {
        let mut ids_to_orders: HashMap<Id, i32> = HashMap::new();
        ids_to_orders.insert(Id(String::from("111")), 1);
        ids_to_orders.insert(Id(String::from("222")), 2);

        let args = UpdateTaskDayOrders {
            ids_to_orders: ids_to_orders.clone(),
        };
        let args = serde_json::to_value(&args).unwrap();
        let obj = args["ids_to_orders"]
            .as_object()
            .expect("ids_to_orders should serialize as a JSON object");
        let expected = r#"{"ids_to_orders":{"111":1,"222":2}}"#;

        pretty_assertions::assert_eq!(obj.len(), 2);
        pretty_assertions::assert_eq!(
            serde_json::to_string(&args).expect("args should be successfully serialized"),
            expected
        );
    }
}
