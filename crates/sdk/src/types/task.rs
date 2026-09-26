use std::{num::NonZeroU32, str::FromStr};

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use chrono_tz::Tz;
use isolang::Language;
use serde::{Deserialize, Serialize};

use crate::{
    error::Result,
    types::{Id, Uid},
};

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
    pub priority: u8,
    /// The ID of the parent task. Set to `None` for root
    /// tasks.
    pub parent_id: Option<Id>,
    /// The order of the task. Defines the position of the
    /// task among all the tasks with the same parent.
    pub child_order: i32,
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
    pub day_order: i32,
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
    pub completed_at: Option<DateTime<Utc>>,
    /// The datetime when the task was created.
    pub added_at: DateTime<Utc>,
    /// The datetime when the task was updated.
    pub updated_at: DateTime<Utc>,
    /// Represents a task's duration. Is `None` if the task has no duration.
    pub duration: Option<TaskDuration>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskDuration {
    pub amount: NonZeroU32,
    pub unit: TaskDurationUnit,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum TaskDurationUnit {
    Minute,
    Day,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DueDate {
    date: String,
    /// Human-readable representation of due date. String always represents the
    /// due object in user's timezone. Look at the Todoist docs to
    /// [see which formats are supported](https://www.todoist.com/help/todoist/features/schedule-a-date-and-time-for-your-todoist-tasks-q7VobO).
    pub string: String,
    /// Timezone of the due instance. Not public since timezone calculations
    /// can be handled through getters instead.
    #[serde(default)]
    timezone: Option<String>,
    /// Language which has to be used to parse the content of the string
    /// attribute. Used by clients and on the server side to properly process
    /// due dates when date object is not set, and when dealing with recurring
    /// tasks.
    pub lang: Language,
    /// Whether the due object represents a recurring due date.
    pub is_recurring: bool,
}

impl DueDate {
    /// Convert the due date to a chrono date type.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `date` is an invalid timestamp.
    /// - Timezone is invalid.
    pub fn to_datetime(&self) -> Result<DueDateType> {
        DueDateType::new(&self.date, self.timezone.as_deref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DueDateType {
    /// Derived from a `YYYY-MM-DD` date string.
    FullDay(NaiveDate),
    /// Derived from a timestamp date string.
    FloatingDateTime(NaiveDateTime),
    /// Derived from a timestamp date string with a provided timezone.
    DateTimeTimezone(DateTime<Tz>),
}

impl DueDateType {
    fn new(date: &str, timezone: Option<&str>) -> Result<Self> {
        if let Ok(date) = NaiveDate::from_str(date) {
            return Ok(Self::FullDay(date));
        }

        if let Some(timezone) = timezone {
            let timezone = Tz::from_str(timezone)?;
            let utc = date.parse::<DateTime<Utc>>()?;
            let local = utc.with_timezone(&timezone);
            return Ok(Self::DateTimeTimezone(local));
        }
        let naive = date.parse::<NaiveDateTime>()?;
        Ok(Self::FloatingDateTime(naive))
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Deadline {
    #[serde(with = "deadline_time_format")]
    pub date: NaiveDate,
    /// Not really used and so is a private field. don't ask me what this does
    /// tbh...
    lang: Language,
}

mod deadline_time_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub(super) const DEADLINE_FORMAT: &str = "%Y-%m-%d";

    // could fix this lint but then it's extra work so...
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(DEADLINE_FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, DEADLINE_FORMAT).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{SyncResponse, types::task::deadline_time_format::DEADLINE_FORMAT};

    use super::*;

    mod due_date {
        use super::*;
        use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
        use chrono_tz::Europe::Madrid;

        #[test]
        fn to_datetime_full_day() {
            let due_date = DueDate {
                date: "2016-12-06".to_string(),
                timezone: None,
                is_recurring: false,
                lang: Language::from_str("en").expect("en is a valid ISO 639 language code"),
                string: String::from("tomorrow"),
            };

            let result = due_date.to_datetime().unwrap();

            assert_eq!(
                result,
                DueDateType::FullDay(NaiveDate::from_ymd_opt(2016, 12, 6).unwrap())
            );
        }

        #[test]
        fn to_datetime_floating_datetime() {
            let due_date = DueDate {
                date: "2016-12-06T13:00:00".to_string(),
                timezone: None,
                is_recurring: false,
                lang: Language::from_str("en").expect("en is a valid ISO 639 language code"),
                string: String::from("tomorrow"),
            };

            let result = due_date.to_datetime().unwrap();

            assert_eq!(
                result,
                DueDateType::FloatingDateTime(
                    NaiveDateTime::parse_from_str("2016-12-06T13:00:00", "%Y-%m-%dT%H:%M:%S",)
                        .unwrap()
                )
            );
        }

        #[test]
        fn to_datetime_with_timezone() {
            let due_date = DueDate {
                date: "2016-12-06T13:00:00.000000Z".to_string(),
                timezone: Some("Europe/Madrid".to_string()),
                is_recurring: false,
                lang: Language::from_str("en").expect("en is a valid ISO 639 language code"),
                string: String::from("tomorrow"),
            };

            let result = due_date.to_datetime().unwrap();

            let expected = DueDateType::DateTimeTimezone(
                "2016-12-06T13:00:00Z"
                    .parse::<DateTime<Utc>>()
                    .unwrap()
                    .with_timezone(&Madrid),
            );

            assert_eq!(result, expected);
        }

        #[test]
        fn to_datetime_invalid_date() {
            let due_date = DueDate {
                date: "not-a-date".to_string(),
                timezone: None,
                is_recurring: false,
                lang: Language::from_str("en").expect("en is a valid ISO 639 language code"),
                string: String::from("tomorrow"),
            };

            assert!(due_date.to_datetime().is_err());
        }

        #[test]
        fn to_datetime_invalid_timezone() {
            let due_date = DueDate {
                date: "2016-12-06T13:00:00.000000Z".to_string(),
                timezone: Some("Not/A_Timezone".to_string()),
                is_recurring: false,
                lang: Language::from_str("en").expect("en is a valid ISO 639 language code"),
                string: String::from("tomorrow"),
            };

            assert!(due_date.to_datetime().is_err());
        }
    }

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
            deadline: None,
            due: None,
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
            added_at: DateTime::from_str("2025-01-21T21:28:43.841504Z")
                .expect("`added_at` timestamp should be valid"),
            updated_at: DateTime::from_str("2025-01-21T21:28:43Z")
                .expect("`updated_at` timestamp should be valid"),
            completed_at: None,
            duration: Some(TaskDuration {
                amount: NonZeroU32::new(15).expect("fifteen is non-zero"),
                unit: TaskDurationUnit::Minute,
            }),
        };
        pretty_assertions::assert_eq!(task, expected);
    }

    #[test]
    fn deserialize_tasks_response() {
        let json = include_str!("./fixtures/tasks/tasks.json");
        let response = serde_json::from_str::<SyncResponse>(json)
            .expect("tasks response should be successfully deserialized");
        let expected = Task {
            id: Id(String::from("6XGgmFVcrG5RRjVr")),
            user_id: Uid(String::from("1234567")),
            project_id: Id(String::from("6XGgm6PHrGgMpCFX")),
            content: String::from("Buy milk"),
            description: String::from("Pick up organic milk"),
            due: Some(DueDate {
                date: String::from("2025-02-12"),
                is_recurring: false,
                timezone: None,
                lang: Language::from_str("en").expect("en is a valid ISO 639 language code"),
                string: String::from("tomorrow"),
            }),
            deadline: Some(Deadline {
                date: NaiveDate::parse_from_str("2025-02-12", DEADLINE_FORMAT)
                    .expect("date should be valid"),
                lang: Language::from_str("en").expect("en is a valid ISO 639 language code"),
            }),
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
            added_at: DateTime::from_str("2025-01-15T10:30:00Z")
                .expect("`added_at` timestamp should be valid"),
            updated_at: DateTime::from_str("2025-01-17T10:30:00Z")
                .expect("`updated_at` timestamp should be valid"),
            completed_at: Some(
                DateTime::from_str("2025-01-16T10:30:00Z")
                    .expect("`completed_at` timestamp should be valid"),
            ),
            duration: Some(TaskDuration {
                amount: NonZeroU32::new(30).expect("thirty is non-zero"),
                unit: TaskDurationUnit::Minute,
            }),
        };

        let items = response.items.expect("`items` should exist");
        assert_eq!(items.len(), 1);
        pretty_assertions::assert_eq!(items[0], expected);
    }
}
