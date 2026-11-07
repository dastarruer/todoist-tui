use std::{num::NonZeroU32, str::FromStr};

use bon::Builder;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use chrono_tz::Tz;
use isolang::Language;
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    types::{Id, Priority, Uid},
};

#[derive(
    Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Builder,
)]
pub struct Task {
    /// The ID of the task.
    #[serde(default)]
    #[builder(default)]
    pub id: Id,
    /// The owner of the task.
    #[builder(into)]
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
    /// The priority of the task.
    #[serde(default)]
    #[builder(default)]
    pub priority: Priority,
    /// The ID of the parent task. Set to `None` for root
    /// tasks.
    pub parent_id: Option<Id>,
    /// The order of the task. Defines the position of the
    /// task among all the tasks with the same parent.
    #[serde(default)]
    #[builder(default)]
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
    #[serde(default)]
    #[builder(default)]
    pub day_order: i32,
    /// Whether the task's sub-tasks are collapsed.
    #[serde(default)]
    #[builder(default)]
    pub is_collapsed: bool,
    /// The task's labels (a list of names that may
    /// represent either personal or shared labels).
    #[serde(default)]
    #[builder(default)]
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
    #[serde(default)]
    #[builder(default)]
    pub checked: bool,
    /// Whether the task is marked as deleted.
    #[serde(default)]
    #[builder(default)]
    pub is_deleted: bool,
    /// The date when the task was completed (or `None` if
    /// not completed).
    pub completed_at: Option<DateTime<Utc>>,
    /// The datetime when the task was created.
    #[builder(default = Utc::now())]
    pub added_at: DateTime<Utc>,
    /// The datetime when the task was last updated.
    #[builder(default = Utc::now())]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    date: Option<String>,
    /// Human-readable representation of due date. String always represents the
    /// due object in user's timezone. Look at the Todoist docs to
    /// [see which formats are supported](https://www.todoist.com/help/todoist/features/schedule-a-date-and-time-for-your-todoist-tasks-q7VobO).
    pub string: String,
    /// Timezone of the due instance. Not public since timezone calculations
    /// can be handled through getters instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    timezone: Option<String>,
    /// Language which has to be used to parse the content of the string
    /// attribute. Used by clients and on the server side to properly process
    /// due dates when date object is not set, and when dealing with recurring
    /// tasks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<Language>,
    /// Whether the due object represents a recurring due date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_recurring: Option<bool>,
}

impl DueDate {
    /// Convert the due date to a chrono date type.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Timezone is invalid.
    pub fn to_datetime(&self) -> Result<DueDateType> {
        DueDateType::new(
            self.date.as_deref().ok_or_else(|| {
                Error::Generic(String::from("due date does not have date string"))
            })?,
            self.timezone.as_deref(),
        )
    }

    /// Intended to be used to create dates from natural language strings,
    /// provided by the user when creating a task.
    ///
    /// See [Write Dates in Natural Language](https://www.todoist.com/help/todoist/features/schedule-a-date-and-time-for-your-todoist-tasks-q7VobO#h_01HD1A9MAEMR98FZT5FG6ZZB73)
    /// for more.
    #[must_use]
    pub const fn from_natural_language_string(string: String) -> Self {
        Self {
            string,
            date: None,
            timezone: None,
            lang: None,
            is_recurring: None,
        }
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

    use crate::types::task::deadline_time_format::DEADLINE_FORMAT;

    use super::*;

    mod due_date {
        use super::*;
        use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
        use chrono_tz::Europe::Madrid;

        #[test]
        fn to_datetime_full_day() {
            let due_date = DueDate {
                date: Some("2016-12-06".to_string()),
                timezone: None,
                is_recurring: Some(false),
                lang: Some(Language::from_str("en").expect("en is a valid ISO 639 language code")),
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
                date: Some("2016-12-06T13:00:00".to_string()),
                timezone: None,
                is_recurring: Some(false),
                lang: Some(Language::from_str("en").expect("en is a valid ISO 639 language code")),
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
                date: Some("2016-12-06T13:00:00.000000Z".to_string()),
                timezone: Some("Europe/Madrid".to_string()),
                is_recurring: Some(false),
                lang: Some(Language::from_str("en").expect("en is a valid ISO 639 language code")),
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
        fn datetime_missing_date() {
            let due_date = DueDate::from_natural_language_string(String::from("tomorrow"));

            // Should return an error since no date string exists
            assert!(due_date.to_datetime().is_err());

            let json = serde_json::to_string(&due_date)
                .expect("due date should be successfully serialized");
            assert_eq!(json, String::from(r#"{"string":"tomorrow"}"#));
        }

        #[test]
        fn to_datetime_invalid_date() {
            let due_date = DueDate {
                date: Some("not-a-date".to_string()),
                timezone: None,
                is_recurring: Some(false),
                lang: Some(Language::from_str("en").expect("en is a valid ISO 639 language code")),
                string: String::from("tomorrow"),
            };

            assert!(due_date.to_datetime().is_err());
        }

        #[test]
        fn to_datetime_invalid_timezone() {
            let due_date = DueDate {
                date: Some("2016-12-06T13:00:00.000000Z".to_string()),
                timezone: Some("Not/A_Timezone".to_string()),
                is_recurring: Some(false),
                lang: Some(Language::from_str("en").expect("en is a valid ISO 639 language code")),
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
            id: Id(String::from("6XGgmFVcrG5RRjVr")),
            user_id: Uid(String::from("1234567")),
            project_id: Id(String::from("6XGgm6PHrGgMpCFX")),
            content: String::from("Buy milk"),
            description: String::from("Pick up organic milk"),
            due: Some(DueDate {
                date: Some(String::from("2025-02-12")),
                is_recurring: Some(false),
                timezone: None,
                lang: Some(Language::from_str("en").expect("en is a valid ISO 639 language code")),
                string: String::from("tomorrow"),
            }),
            deadline: Some(Deadline {
                date: NaiveDate::parse_from_str("2025-02-12", DEADLINE_FORMAT)
                    .expect("date should be valid"),
                lang: Language::from_str("en").expect("en is a valid ISO 639 language code"),
            }),
            priority: Priority::NoPriority,
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
        pretty_assertions::assert_eq!(task, expected);
    }
}
