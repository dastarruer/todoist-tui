use std::{env::var, fmt::Display, str::FromStr};

use anyhow::Ok;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

// TODO: Add missing due, deadline, and duration fields
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Task {
    /// The ID of the task.
    pub id: Id,
    /// The owner of the task.
    pub user_id: Uid,
    /// The ID of the parent project.
    pub project_id: Id,
    /// The text of the task. This value may contain
    /// markdown-formatted text and hyperlinks.
    pub content: String,
    /// A description for the task. This value may contain
    /// markdown-formatted text and hyperlinks.
    pub description: String,
    /// The priority of the task (a number between `1` and
    /// `4`, `4` for very urgent and `1` for natural).
    ///
    /// **Note:** Keep in mind that very urgent is the
    /// priority `1` on clients. So, `p1` will return `4` in
    /// the API.
    pub priority: u8,
    /// The ID of the parent task. Set to `None` for root
    /// tasks.
    pub parent_id: Option<Id>,
    /// The order of the task. Defines the position of the
    /// task among all the tasks with the same parent.
    pub child_order: i8,
    /// Task's fractional-indexing order key: tasks sort by
    /// comparing keys lexicographically among siblings
    /// sharing the same project, section and parent task.
    /// May be `None` for tasks not yet migrated.
    pub order_key: Option<String>,
    /// The ID of the parent section. Set to `None` for
    /// tasks not belonging to a section.
    pub section_id: Option<Id>,
    /// The order of the task inside the `Today` or
    /// `Next 7 days` view (a number, where the smallest
    /// value would place the task at the top).
    pub day_order: i8,
    /// Whether the task's sub-tasks are collapsed.
    pub is_collapsed: bool,
    /// The task's labels (a list of names that may
    /// represent either personal or shared labels).
    pub labels: Vec<String>,
    /// The UID of the user who created the task. This makes
    /// sense for shared projects only. For tasks created
    /// before `31 Oct 2019` the value is set to `None`.
    /// Cannot be set explicitly or changed via API.
    pub added_by_uid: Option<Uid>,
    /// The UID of the user who assigned the task. This
    /// makes sense for shared projects only. Accepts any
    /// UID from the list of project collaborators. If this
    /// value is unset or invalid, it will automatically be
    /// set up to your UID.
    pub assigned_by_uid: Option<Uid>,
    /// The UID of user who is responsible for accomplishing
    /// the current task. This makes sense for shared
    /// projects only. Accepts any UID from the list of
    /// project collaborators or null or an empty string to
    /// unset.
    pub responsible_uid: Option<Uid>,
    /// Whether the task is marked as completed.
    pub checked: bool,
    /// Whether the task is marked as deleted.
    pub is_deleted: bool,
    /// The date when the task was completed (or `None` if
    /// not completed).
    pub completed_at: Option<String>,
    /// The datetime when the task was created.
    pub added_at: String,
    /// The datetime when the task was updated.
    pub updated_at: String,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id(String);

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uid(String);

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
struct TasksResponse {
    results: Vec<Task>,
    next_cursor: Option<String>,
}

pub struct APIClient {
    pub key: TodoistAPIKey,
    client: Client,
}

impl APIClient {
    #[must_use]
    pub fn new(key: TodoistAPIKey) -> Self {
        let client = Client::new();
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
    pub async fn tasks(&self) -> anyhow::Result<Vec<Task>> {
        #[expect(clippy::missing_panics_doc, reason = "infallible")]
        let url = Self::base_url()
            .join("api/v1/tasks")
            .expect("joined URL should be valid");
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
        Ok(resp.results)
    }
}

#[derive(Debug)]
pub struct TodoistAPIKey(String);

impl Display for TodoistAPIKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Retrieves the Todoist API key from the user's system.
///
/// # Errors
///
/// Returns an error if:
///
/// - The `API_KEY` environment variable is not set.
pub fn retrieve_api_key() -> anyhow::Result<TodoistAPIKey> {
    Ok(TodoistAPIKey(var("API_KEY")?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_task() {
        let json = include_str!("./fixtures/tasks/task.json");
        let task =
            serde_json::from_str::<Task>(json).expect("task should be successfully deserialized");
        let expected = Task {
            id: Id(String::from("6X7rM8997g3RQmvh")),
            user_id: Uid(String::from("2671355")),
            project_id: Id(String::from("6Jf8VQXxpwv56VQ7")),
            content: String::from("Buy Milk"),
            description: String::new(),
            priority: 1,
            parent_id: None,
            child_order: 1,
            order_key: Some(String::from("a1V")),
            section_id: Some(Id(String::from("3Ty8VQXxpwv28PK3"))),
            day_order: -1,
            is_collapsed: false,
            labels: vec![String::from("Food"), String::from("Shopping")],
            added_by_uid: Some(Uid(String::from("2671355"))),
            assigned_by_uid: Some(Uid(String::from("2671355"))),
            responsible_uid: None,
            checked: false,
            is_deleted: false,
            added_at: String::from("2025-01-21T21:28:43.841504Z"),
            updated_at: String::from("2025-01-21T21:28:43Z"),
            completed_at: None,
        };
        pretty_assertions::assert_eq!(task, expected);
    }

    #[test]
    fn deserialize_tasks_response() {
        let json = include_str!("./fixtures/tasks/tasks.json");
        let response = serde_json::from_str::<TasksResponse>(json)
            .expect("tasks response should be successfully deserialized");
        let expected = Task {
            id: Id(String::from("6XGgmFVcrG5RRjVr")),
            user_id: Uid(String::from("1234567")),
            project_id: Id(String::from("6XGgm6PHrGgMpCFX")),
            content: String::from("Buy milk"),
            description: String::from("Pick up organic milk"),
            priority: 1,
            parent_id: Some(Id(String::from("6XGgmFVcrG5RRjVr"))),
            child_order: 1,
            order_key: Some(String::from("a1V")),
            section_id: Some(Id(String::from("6fFPHV272WWh3gpW"))),
            day_order: 1,
            is_collapsed: false,
            labels: vec![String::from("priority")],
            added_by_uid: Some(Uid(String::from("1234567"))),
            assigned_by_uid: Some(Uid(String::from("1234567"))),
            responsible_uid: Some(Uid(String::from("1234567"))),
            checked: false,
            is_deleted: false,
            added_at: String::from("2025-01-15T10:30:00Z"),
            updated_at: String::from("2025-01-17T10:30:00Z"),
            completed_at: Some(String::from("2025-01-16T10:30:00Z")),
        };

        assert_eq!(response.results.len(), 1);
        pretty_assertions::assert_eq!(response.results[0], expected);
        assert_eq!(
            response.next_cursor,
            Some(String::from("14540000435w8hj8pXXwPQJJch.X9DBH8ya2Xenok55"))
        );
    }
}
