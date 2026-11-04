use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use isolang::Language;
use serde::{Deserialize, Deserializer, Serialize};
use serde_aux::prelude::*;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::struct_excessive_bools)]
pub struct User {
    /// The default time in minutes for automatic reminders set whenever a due date is specified for a task.
    pub auto_reminder: i32,
    /// The link to a `195x195` pixels image of the user's avatar.
    pub avatar_big: String,
    /// The link to a `60x60` pixels image of the user's avatar.
    pub avatar_medium: String,
    /// The link to a `640x640` pixels image of the user's avatar.
    pub avatar_s640: String,
    /// The link to a `35x35` pixels image of the user's avatar.
    pub avatar_small: String,
    /// The ID of the user's business account.
    pub business_account_id: Option<String>,
    /// The daily goal number of completed tasks for karma.
    pub daily_goal: u32,
    /// Which date format the user prefers.
    pub date_format: DateFormat,
    /// The language expected for date recognition instead of the user's `lang` (`None` if `lang` determines this).
    pub dateist_lang: Option<Language>,
    /// Array of user's days off.
    pub days_off: Vec<Day>,
    /// The user's email address.
    pub email: String,
    /// An opaque ID used internally to handle features for the user.
    pub feature_identifier: String,
    /// Used internally for any special features that apply to the user.
    pub features: UserFeatures,
    /// The user's real name formatted as "Firstname Lastname".
    pub full_name: String,
    /// Whether the user has a password set on the account.
    ///
    /// False if authenticated via OAuth/social login without a password.
    pub has_password: bool,
    /// The user's unique ID.
    pub id: String,
    /// The ID of the user's avatar.
    pub image_id: String,
    /// The ID of the user's Inbox project.
    pub inbox_project_id: String,
    /// Whether the user has an active Todoist Pro subscription.
    pub is_premium: bool,
    /// The UTC timestamp when the user joined Todoist.
    pub joined_at: String,
    /// The user's karma score.
    // Sent as a float by the API; for simplicity just truncate it
    #[serde(deserialize_with = "deserialize_karma")]
    pub karma: i32,
    /// The user's karma trend (for example, "up").
    pub karma_trend: String,
    /// The user's language setting.
    pub lang: Language,
    /// Whether multi-factor authentication is enabled.
    #[serde(default)]
    pub mfa_enabled: bool,
    /// The day of the next week tasks will be postponed to (1 to 7, where 1 is Monday and 7 is Sunday).
    pub next_week: i32,
    #[serde(default)]
    pub onboarded_dated_tasks_created: bool,
    #[serde(default)]
    pub onboarded_desktop_accessed: bool,
    #[serde(default)]
    pub onboarded_mobile_accessed: bool,
    #[serde(default)]
    pub onboarded_tasks_completed: bool,
    #[serde(default)]
    pub onboarding_completed: bool,
    #[serde(default)]
    pub onboarding_initiated: bool,
    /// For example, "beginner".
    #[serde(default)]
    pub onboarding_level: String,
    #[serde(default)]
    pub onboarding_persona: Option<String>,
    #[serde(default)]
    pub onboarding_role: Option<String>,
    #[serde(default)]
    pub onboarding_started: bool,
    #[serde(default)]
    pub onboarding_team_mode: bool,
    /// For example, `["personal", "education"]`.
    #[serde(default)]
    pub onboarding_use_cases: Vec<String>,
    /// Outlines why a user is premium.
    ///
    /// Possible values: `not_premium`, `current_personal_plan`, `legacy_personal_plan`, or `teams_business_member`.
    pub premium_status: String,
    /// The date when the user's Todoist Pro subscription ends (`None` if not Pro).
    ///
    /// **Note:** Use for informational purposes only (does not include grace periods). Use `is_premium` to check active status.
    pub premium_until: Option<String>,
    /// Usage of the ramble sessions feature.
    #[serde(default)]
    pub ramble_sessions_usage: Option<UsageQuota>,
    /// The maximum number of people a free user can share a project with.
    #[serde(default)]
    pub share_limit: u32,
    /// Whether to show projects in oldest dates first order (0) or oldest dates last order (1).
    pub sort_order: i32,
    /// The first day of the week (1 to 7, where 1 is Monday and 7 is Sunday).
    pub start_day: i32,
    /// The user's default view on Todoist (e.g., `inbox`, `today`, `next7days`, `project?id=1234`, `label?name=abc`).
    pub start_page: String,
    /// The user's status, shared across Todoist and Comms. Set to `None` if the user has no status or it has expired.
    #[serde(default)]
    pub status: Option<UserStatus>,
    /// The ID of the user's team inbox (`None` if not on a team).
    #[serde(default)]
    pub team_inbox_id: Option<String>,
    /// For example, "system", "light" or "dark".
    #[serde(default)]
    pub theme_appearance: String,
    /// The currently selected Todoist theme ID (a string representation of a number between 0 and 10).
    pub theme_id: String,
    /// Whether to use 24h format (0) or 12h format (1) when displaying time.
    pub time_format: i32,
    /// The user's authentication token used for API calls.
    pub token: String,
    /// The user's timezone details.
    #[serde(deserialize_with = "deserialize_datetime_with_tz")]
    pub tz_info: DateTime<FixedOffset>,
    #[serde(default)]
    pub unique_prefix: u32,
    /// The device designated to receive urgent reminder alarms (`None` if unset).
    pub urgent_reminder_device: Option<UrgentReminderDevice>,
    /// Describes if the user has verified their e-mail address (`unverified`, `verified`, `blocked`, or `legacy`).
    pub verification_status: String,
    /// The websocket URL for realtime updates.
    #[serde(default)]
    pub websocket_url: String,
    /// The day used when a user chooses to schedule a task for the 'Weekend' (1 to 7, where 1 is Monday and 7 is Sunday).
    pub weekend_start_day: i32,
    /// The target number of tasks to complete per week.
    pub weekly_goal: i32,
}

/// Usage quota for AI-powered features (capture sessions, ramble sessions).
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UsageQuota {
    pub current_count: u32,
    pub limit: u32,
    pub remaining: u32,
    /// The UTC timestamp when the quota resets.
    pub reset_date: String,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserStatus {
    /// Contains the message of the status.
    pub message: String,
    /// Contains an optional emoji.
    pub emoji: Option<String>,
    /// UTC date time when the status expires.
    #[serde(default)]
    pub active_until: Option<DateTime<Utc>>,
    #[serde(default)]
    pub effects: Option<StatusEffects>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StatusEffects {
    #[serde(default)]
    pub pause_notifications: Option<PauseNotifications>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PauseNotifications {
    pub scope: StatusScope,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StatusScope {
    /// Only `"all"` appears in the sample, so other modes are unknown.
    pub mode: String,
}

/// Internal feature flags and settings associated with the user.
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::struct_excessive_bools)]
pub struct UserFeatures {
    /// Whether the user has enabled beta features.
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub beta: bool,
    /// Whether inline date parsing support is disabled.
    pub dateist_inline_disabled: bool,
    /// Whether date parsing language override is set.
    pub dateist_lang: Option<bool>,
    /// Whether the gold theme has been awarded to the user.
    #[serde(deserialize_with = "deserialize_bool_from_anything", default)]
    pub gold_theme: bool,
    /// Whether push reminders are enabled.
    pub has_push_reminders: bool,
    /// Whether karma tracking is disabled.
    pub karma_disabled: bool,
    /// Whether karma vacation mode is enabled.
    pub karma_vacation: bool,
}

/// Information regarding the device assigned to receive urgent reminder notifications.
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UrgentReminderDevice {
    /// Platform of the device (`ios` or `android`).
    pub device_platform: String,
    /// Numeric ID of the device.
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub device_id: i64,
    /// The push notification token for the device.
    pub device_token: String,
    /// Optional user-friendly device name.
    pub device_name: String,
}

/// Timezone details for the user.
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TzUserInfo {
    /// Timezone identifier string (e.g., `America/New_York`).
    pub timezone: String,
    /// Hours difference from GMT.
    pub hours: i32,
    /// Minutes difference from GMT.
    pub minutes: i32,
    /// Whether Daylight Saving Time currently applies.
    pub is_dst: bool,
    /// String representation of the GMT offset (e.g., "GMT -05:00").
    pub gmt_string: String,
}

#[derive(
    Deserialize_repr, Serialize_repr, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord,
)]
#[repr(usize)]
pub enum DateFormat {
    #[default]
    DDMMYYY = 0,
    MMDDYYY = 1,
}

#[derive(
    Deserialize_repr, Serialize_repr, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord,
)]
#[repr(usize)]
pub enum Day {
    #[default]
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Sunday = 7,
}

fn deserialize_datetime_with_tz<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct TzHelper {
        hours: i32,
        minutes: i32,
    }

    let helper = TzHelper::deserialize(deserializer)?;

    // Construct a FixedOffset from the hours and minutes provided in tz_info
    #[allow(clippy::arithmetic_side_effects)]
    let total_seconds = (helper.hours * 3600) + (helper.minutes.abs() * 60 * helper.hours.signum());
    let offset = FixedOffset::east_opt(total_seconds)
        .ok_or_else(|| serde::de::Error::custom("invalid timezone offset"))?;

    // Defaulting to epoch or combining with a base date if needed,
    // or returning a DateTime anchored at a specific point with that offset:
    Ok(FixedOffset::east_opt(0)
        .expect("0 is a valid UTC offset")
        .timestamp_opt(0, 0)
        .unwrap()
        .with_timezone(&offset))
}

#[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
fn deserialize_karma<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(f64::deserialize(deserializer)?.round() as i32)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_user() {
        let json = include_str!("./fixtures/user/user.json");
        let expected = User {
            auto_reminder: 0,
            avatar_big: String::from("https://*.cloudfront.net/*_big.jpg"),
            avatar_medium: String::from("https://*.cloudfront.net/*_medium.jpg"),
            avatar_s640: String::from("https://*.cloudfront.net/*_s640.jpg"),
            avatar_small: String::from("https://*.cloudfront.net/*_small.jpg"),
            business_account_id: Some(String::from("1")),
            daily_goal: 15,
            date_format: DateFormat::DDMMYYY,
            dateist_lang: None,
            days_off: vec![Day::Saturday, Day::Sunday],
            email: String::from("me@example.com"),
            feature_identifier: String::from(
                "2671355_0123456789abcdef70123456789abcdefe0123456789abcdefd0123456789abc",
            ),
            features: UserFeatures {
                beta: true,
                dateist_inline_disabled: false,
                dateist_lang: None,
                gold_theme: false,
                has_push_reminders: true,
                karma_disabled: false,
                karma_vacation: false,
            },
            full_name: String::from("Example User"),
            has_password: true,
            id: String::from("2671355"),
            image_id: String::from("d160009dfd52b991030d55227003450f"),
            inbox_project_id: String::from("6X7fqH39MwjmwV4q"),
            is_premium: true,
            joined_at: String::from("2015-07-31T18:32:06.000000Z"),
            karma: 37504,
            karma_trend: String::from("up"),
            lang: Language::from_639_1("en").expect("en is a valid ISO 639 language code"),
            mfa_enabled: false,
            next_week: 1,
            premium_status: String::from("current_personal_plan"),
            premium_until: None,
            share_limit: 51,
            sort_order: 0,
            start_day: 1,
            start_page: String::from("project?id=2203306141"),
            status: Some(UserStatus {
                message: String::from("Deep work"),
                emoji: Some(String::from("🎧")),
                active_until: Some(
                    "2026-10-09T17:00:00Z"
                        .parse::<DateTime<Utc>>()
                        .expect("`active_until` field should be valid"),
                ),
                effects: Some(StatusEffects {
                    pause_notifications: Some(PauseNotifications {
                        scope: StatusScope {
                            mode: String::from("all"),
                        },
                    }),
                }),
            }),
            theme_id: String::from("11"),
            time_format: 0,
            token: String::from("0123456789abcdef0123456789abcdef01234567"),
            // `device_id` is a string in the JSON; `deserialize_number_from_string` parses it.
            urgent_reminder_device: Some(UrgentReminderDevice {
                device_platform: String::from("ios"),
                device_id: 12_345_678,
                device_token: String::from("a1b2c3d4..."),
                device_name: String::from("iPhone 15 Pro"),
            }),
            tz_info: {
                // -03:00; the deserializer anchors the offset at the epoch.
                let offset =
                    FixedOffset::west_opt(3 * 3600).expect("-3 hours should be a valid offset");
                DateTime::from_timestamp(0, 0)
                    .unwrap()
                    .with_timezone(&offset)
            },
            verification_status: String::from("legacy"),
            weekend_start_day: 6,
            weekly_goal: 30,
            // Fields absent from this JSON fall back to their `#[serde(default)]` values.
            ..Default::default()
        };

        assert_eq!(
            serde_json::from_str::<User>(json)
                .expect("user json should be successfully serialized"),
            expected
        );
    }
}
