use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{Color, Id};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::struct_excessive_bools)]
pub struct Project {
    /// The ID of the project.
    pub id: Id,
    /// The name of the project.
    pub name: String,
    /// Description for the project. Only used for teams.
    pub description: String,
    /// Real or temp ID of the workspace the project. Only used for teams.
    pub workspace_id: u32,
    /// Indicates if the project is invite-only or if it should be visible for
    /// everyone in the workspace. If missing or null, the default value from
    /// the workspace `is_invite_only_default` will be used. Only used for
    /// teams.
    pub is_invite_only: Option<bool>,
    /// The status of the project. Only used for teams.
    pub status: ProjectStatus,
    /// If `false`, the project is invite-only and people can't join by link.
    /// If true, the project is visible to anyone with a link, and anyone can
    /// join it. Only used for teams.
    pub is_link_sharing_enabled: bool,
    /// The default role a user can have. Only used for teams.
    pub collaborator_role_default: Option<Role>,
    /// The color of the project icon.
    pub color: Color,
    /// The ID of the parent project. Set to `None` for root
    /// projects.
    pub parent_id: Option<Id>,
    /// The order of the project. Defines the position of the project among all the projects with the same `parent_id`
    pub child_order: i32,
    /// Project's fractional-indexing order key: personal projects sort by
    /// comparing keys lexicographically among siblings with the same
    /// `parent_id`. May be `None` for projects not yet migrated, and is always
    /// `None` for workspace projects (ordered via folders instead).
    pub order_key: Option<String>,
    /// Whether the project's sub-projects are collapsed.
    pub is_collapsed: bool,
    /// Whether the project is shared.
    pub shared: bool,
    /// Whether tasks in the project can be assigned to users.
    pub can_assign_tasks: bool,
    /// Whether the project is marked as deleted.
    pub is_deleted: bool,
    /// Whether the project is marked as archived.
    pub is_archived: bool,
    /// Whether the project is a favorite.
    pub is_favorite: bool,
    /// Whether the project is from a canceled subscription.
    pub is_frozen: bool,
    /// The mode in which to render tasks in this project.
    pub view_style: ViewStyle,
    /// The role of the requesting user. Only used for teams.
    pub role: Role,
    /// Whether the project is `Inbox`.
    pub inbox_project: bool,
    /// The ID of the folder which this project is in.
    pub folder_id: Option<Id>,
    /// Date at which project was created.
    pub created_at: DateTime<Utc>,
    /// Date at which project was last updated.
    pub updated_at: DateTime<Utc>,
    /// If `true`, default collaborators are still being added to the project
    /// in the background. Only used for teams.
    pub is_pending_default_collaborator_invites: bool,
    /// Project access configuration.
    pub access: ProjectAccess,
    /// Whether Project Insights is enabled for this project. Defaults to
    /// `true` for new workspace projects. Only used for teams.
    #[serde(default)]
    pub is_project_insights_enabled: bool,
    /// Fractional-indexing key for the workspace's shared default-ordering
    /// scope: workspace projects and folders sort by comparing keys
    /// lexicographically against each other. null for personal projects, and
    /// for workspace projects not yet migrated. Only used for teams.
    default_order_key: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProjectAccess {
    pub visibility: ProjectVisibility,
    /// This field only matters for public projects.
    pub configuration: ProjectConfiguration,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProjectConfiguration {
    /// Whether collaborator details are hidden from public viewers.
    pub hide_collaborator_details: bool,
    /// Whether public viewers can duplicate this project.
    pub disable_duplication: bool,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum ProjectVisibility {
    #[default]
    Restricted,
    Team,
    Public,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProjectStatus {
    #[default]
    Planned,
    InProgress,
    Paused,
    Completed,
    Canceled,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    Creator,
    Admin,
    #[default]
    ReadWrite,
    EditOnly,
    CompleteOnly,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum ViewStyle {
    #[default]
    List,
    Board,
    Calendar,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn deserialize_task() {
        let json = include_str!("../fixtures/projects/project.json");
        let project = serde_json::from_str::<Project>(json)
            .expect("project should be successfully deserialized");
        let expected = Project {
            id: Id(String::from("6Jf8VQXxpwv56VQ7")),
            name: String::from("Shopping List"),
            description: String::from("Stuff to buy"),
            workspace_id: 12345,
            is_invite_only: Some(false),
            status: ProjectStatus::InProgress,
            is_link_sharing_enabled: true,
            collaborator_role_default: Some(Role::ReadWrite),
            color: Color::LimeGreen,
            parent_id: None,
            child_order: 1,
            order_key: Some(String::from("a1V")),
            is_collapsed: false,
            shared: false,
            can_assign_tasks: false,
            is_deleted: false,
            is_archived: false,
            is_favorite: false,
            is_frozen: false,
            view_style: ViewStyle::List,
            role: Role::ReadWrite,
            inbox_project: true,
            folder_id: None,
            created_at: DateTime::from_str("2023-07-13T10:20:59Z")
                .expect("`created_at` timestamp should be valid"),
            updated_at: DateTime::from_str("2024-12-10T13:27:29Z")
                .expect("`updated_at` timestamp should be valid"),
            is_pending_default_collaborator_invites: false,
            access: ProjectAccess {
                visibility: ProjectVisibility::Public,
                configuration: ProjectConfiguration {
                    hide_collaborator_details: true,
                    disable_duplication: true,
                },
            },
            is_project_insights_enabled: false,
            default_order_key: None,
        };
        pretty_assertions::assert_eq!(project, expected);
    }
}
