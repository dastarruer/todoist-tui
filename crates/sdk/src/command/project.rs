use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::{
    command::CommandArgs,
    types::{
        Color, Id,
        project::{Project, ProjectAccess, ProjectStatus, Role, ViewStyle},
    },
};

/// Add a new project.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Default))]
#[builder(on(String, into), on(Id, into))]
pub struct AddProject {
    /// The name of the project.
    pub name: String,
    /// Description for the project (up to 1024 characters). Only used for
    /// teams.
    pub description: Option<String>,
    /// The status of the project.
    pub status: Option<ProjectStatus>,
    /// The color of the project icon.
    pub color: Option<Color>,
    /// The ID of the parent project. Set to `None` for root projects.
    pub parent_id: Option<Id>,
    /// The ID of the folder, when creating projects in workspaces.
    /// Set to `None` for root projects.
    pub folder_id: Option<String>,
    /// The order of the project. Defines the position of the project among all the projects with the same `parent_id`.
    pub child_order: Option<i32>,
    /// Fractional-indexing order key for the new project. When omitted, the backend generates a key placing the project at the bottom of its parent's list. If a sibling under the same `parent_id` already uses the requested key, the backend stores an adjusted key placing the project immediately after that position; the corrected value is returned via sync.
    pub order_key: Option<String>,
    /// Whether the project is a favorite.
    pub is_favorite: Option<bool>,
    /// Determines the way the project is displayed within Todoist clients.
    pub view_style: Option<ViewStyle>,
    /// ID of the workspace the project should belong to.
    pub workspace_id: Option<Id>,
    /// Indicates if the project is invite-only or if it should be visible for everyone in the workspace. If left as `None`, the default value from the workspace `is_invite_only_default` will be used. Only used for teams.
    pub is_invite_only: Option<bool>,
    /// If `false`, the project is invite-only and people can't join by link. If `true`, the project is visible to anyone with a link, and anyone can join it. Only used for teams.
    pub is_link_sharing_enabled: Option<bool>,
    /// The default role a user can have. Only used for teams.
    pub collaborator_role_default: Option<Role>,
    /// Project access configuration.
    pub access: Option<ProjectAccess>,
    /// Whether Project Insights is enabled for this project.
    ///
    /// Defaults to `true`. Only used for teams.
    #[builder(default = true)]
    pub is_project_insights_enabled: bool,
}

impl CommandArgs for AddProject {
    fn command_type(&self) -> &'static str {
        "project_add"
    }
    fn creates_resource(&self) -> bool {
        true
    }
}

impl From<Project> for AddProject {
    fn from(project: Project) -> Self {
        let Project {
            name,
            description,
            workspace_id,
            is_invite_only,
            status,
            is_link_sharing_enabled,
            collaborator_role_default,
            color,
            parent_id,
            is_favorite,
            view_style,
            folder_id,
            access,
            is_project_insights_enabled,
            id: _,
            child_order: _,
            order_key: _,
            is_collapsed: _,
            shared: _,
            can_assign_tasks: _,
            is_deleted: _,
            is_archived: _,
            is_frozen: _,
            role: _,
            inbox_project: _,
            created_at: _,
            updated_at: _,
            is_pending_default_collaborator_invites: _,
            default_order_key: _,
        } = project;

        // `Project` stores plain values for team-only fields, so their defaults
        // would otherwise be sent for personal projects too.
        let is_team = workspace_id != 0;

        Self {
            name,
            description: Some(description).filter(|d| !d.is_empty()),
            status: is_team.then_some(status),
            color: Some(color),
            parent_id,
            folder_id: folder_id.map(|id| id.0),
            child_order: None, // omitted so the backend places it at the bottom
            order_key: None,
            is_favorite: Some(is_favorite),
            view_style: Some(view_style),
            workspace_id: is_team.then(|| Id(workspace_id.to_string())),
            is_invite_only,
            is_link_sharing_enabled: is_team.then_some(is_link_sharing_enabled),
            collaborator_role_default,
            access: Some(access),
            is_project_insights_enabled,
        }
    }
}

/// Update an existing project.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Default))]
#[builder(on(String, into), on(Id, into))]
pub struct UpdateProject {
    /// The ID of the project to be updated.
    pub id: Id,
    /// The name of the project.
    pub name: Option<String>,
    /// The color of the project icon.
    pub color: Option<Color>,
    /// Whether the project's sub-projects are collapsed.
    pub is_collapsed: Option<bool>,
    /// Fractional-indexing order key for the new project. When omitted, the backend generates a key placing the project at the bottom of its parent's list. If a sibling under the same `parent_id` already uses the requested key, the backend stores an adjusted key placing the project immediately after that position; the corrected value is returned via sync.
    pub order_key: Option<String>,
    /// Fractional-indexing key for the workspace's shared default-ordering scope (workspace projects and folders together). Only accepted for workspace projects; ignored for personal projects. When omitted, the current key is kept. If another project or folder in the workspace already uses the requested key, the backend stores an adjusted key placing the project immediately after that position; the corrected value is returned via sync. Only used for teams.
    pub default_order_key: Option<String>,
    /// Whether the project is a favorite.
    pub is_favorite: Option<bool>,
    /// Determines the way the project is displayed within Todoist clients.
    pub view_style: Option<ViewStyle>,
    /// Description for the project (up to 1024 characters). Only used for
    /// teams.
    pub description: Option<String>,
    /// The status of the project.
    pub status: Option<ProjectStatus>,
    /// If `false`, the project is invite-only and people can't join by link. If `true`, the project is visible to anyone with a link, and anyone can join it. Only used for teams.
    pub is_link_sharing_enabled: Option<bool>,
    /// Project access configuration.
    pub access: Option<ProjectAccess>,
    /// Whether Project Insights is enabled for this project. Only used for
    /// teams.
    pub is_project_insights_enabled: bool,
    /// The default role a user can have. Only used for teams.
    pub collaborator_role_default: Option<Role>,
}

impl CommandArgs for UpdateProject {
    fn command_type(&self) -> &'static str {
        "project_update"
    }
    fn creates_resource(&self) -> bool {
        false
    }
}

impl From<Project> for UpdateProject {
    fn from(project: Project) -> Self {
        let Project {
            id,
            name,
            description,
            workspace_id,
            status,
            is_link_sharing_enabled,
            collaborator_role_default,
            color,
            is_collapsed,
            is_favorite,
            view_style,
            access,
            is_project_insights_enabled,
            is_invite_only: _,
            parent_id: _,
            child_order: _,
            order_key: _,
            shared: _,
            can_assign_tasks: _,
            is_deleted: _,
            is_archived: _,
            is_frozen: _,
            role: _,
            inbox_project: _,
            folder_id: _,
            created_at: _,
            updated_at: _,
            is_pending_default_collaborator_invites: _,
            default_order_key: _,
        } = project;

        let is_team = workspace_id != 0;

        Self {
            id,
            name: Some(name),
            color: Some(color),
            is_collapsed: Some(is_collapsed),
            order_key: None, // omitted keeps the server's current key
            default_order_key: None,
            is_favorite: Some(is_favorite),
            view_style: Some(view_style),
            description: is_team.then_some(description),
            status: is_team.then_some(status),
            is_link_sharing_enabled: is_team.then_some(is_link_sharing_enabled),
            access: Some(access),
            is_project_insights_enabled,
            collaborator_role_default,
        }
    }
}

/// Update the parent project of a project.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Default))]
#[builder(on(String, into), on(Id, into))]
pub struct MoveProject {
    /// The ID of the project.
    pub id: String,
    /// The ID of the parent project. If set to `None`, the project will be
    /// moved to the root.
    pub parent_id: Option<Option<String>>,
}

impl CommandArgs for MoveProject {
    fn command_type(&self) -> &'static str {
        "project_move"
    }
    fn creates_resource(&self) -> bool {
        false
    }
}

/// Moves a personal project into the target workspace.
///
/// A few notes about moving projects to a workspace:
/// - Moving a parent project to a workspace will also move all its child
///   projects to that workspace.
/// - If no `folder_id` is supplied, child projects will be moved into a folder
///   with the same name as the parent project being moved.
/// - If a `folder_id` is supplied, the parent and child projects will be moved
///   into that folder.
/// - At the moment, it is not possible to move a project to another workspace
///   (changing its `workspace_id`), or to the user's personal workspace.
/// - Moving a project to a workspace affects all its collaborators.
///   Collaborators who are not members of the target workspace will be added
///   as guests, if guest members are allowed in the target workspace.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder, Clone, PartialEq, Eq)]
#[builder(on(String, into), on(Id, into))]
pub struct MoveProjectIntoWorkspace {
    /// The ID of the project.
    #[serde(rename = "project_id")]
    pub id: Id,
    /// The ID of the workspace the project will be moved into.
    pub workspace_id: Id,
    /// Whether the project is restricted or open to all workspace members.
    pub is_invite_only: Option<bool>,
    /// If provided, the project and any child projects will be moved into this workspace folder.
    pub folder_id: Option<Id>,
    /// Soon to be deprecated and should always be set to true.
    #[builder(skip = true)]
    use_lro: bool,
}

impl CommandArgs for MoveProjectIntoWorkspace {
    fn command_type(&self) -> &'static str {
        "project_move_to_workspace"
    }
    fn creates_resource(&self) -> bool {
        false
    }
}

#[cfg(test)]
impl Default for MoveProjectIntoWorkspace {
    fn default() -> Self {
        Self {
            id: Id::default(),
            workspace_id: Id::default(),
            is_invite_only: None,
            folder_id: None,
            use_lro: true,
        }
    }
}

/// Moves a project inside a workspace out back into a user's personal space.
///
/// Only the original creator of the project has permissions to do this, and
/// only if they are still currently an admin of said workspace.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder, Clone, PartialEq, Eq)]
#[builder(on(String, into), on(Id, into))]
pub struct MoveProjectOutOfWorkspace {
    /// The ID of the project being moved out.
    #[serde(rename = "project_id")]
    pub id: Id,
    /// Soon to be deprecated and should always be set to true.
    #[builder(skip = true)]
    use_lro: bool,
}

impl From<Project> for MoveProjectOutOfWorkspace {
    fn from(project: Project) -> Self {
        Self {
            id: project.id,
            use_lro: true,
        }
    }
}

impl CommandArgs for MoveProjectOutOfWorkspace {
    fn command_type(&self) -> &'static str {
        "project_move_to_personal"
    }
    fn creates_resource(&self) -> bool {
        false
    }
}

#[cfg(test)]
impl Default for MoveProjectOutOfWorkspace {
    fn default() -> Self {
        Self {
            id: Id::default(),
            use_lro: true,
        }
    }
}

/// Delete an existing project and all its descendants.
///
/// Workspace projects can only be deleted by users with `Role::Admin` and it
/// must be archived first.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Default))]
#[builder(on(String, into), on(Id, into))]
pub struct DeleteProject {
    /// The ID of the project to delete.
    pub id: Id,
}

impl CommandArgs for DeleteProject {
    fn command_type(&self) -> &'static str {
        "project_delete"
    }
    fn creates_resource(&self) -> bool {
        false
    }
}

/// Archive a project and its descendants.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Default))]
#[builder(on(String, into), on(Id, into))]
pub struct ArchiveProject {
    /// The ID of the project to archive.
    pub id: Id,
}

impl CommandArgs for ArchiveProject {
    fn command_type(&self) -> &'static str {
        "project_archive"
    }
    fn creates_resource(&self) -> bool {
        false
    }
}

/// Unarchive a project and its descendants.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Default))]
#[builder(on(String, into), on(Id, into))]
pub struct UnarchiveProject {
    /// The ID of the project to unarchive.
    pub id: Id,
}

impl CommandArgs for UnarchiveProject {
    fn command_type(&self) -> &'static str {
        "project_unarchive"
    }
    fn creates_resource(&self) -> bool {
        false
    }
}

/// Change a project's role.
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Default))]
#[builder(on(String, into), on(Id, into))]
pub struct ChangeProjectRole {
    /// The ID of the project to change the role for.
    pub id: Id,
    /// ID of the user whose role to change.
    pub user_id: u32,
    /// New role for the user.
    ///
    /// Note: Only the project creator can be assigned the `Role::Creator` role.
    pub role: Role,
}

impl CommandArgs for ChangeProjectRole {
    fn command_type(&self) -> &'static str {
        "project_change_role"
    }
    fn creates_resource(&self) -> bool {
        false
    }
}

impl From<Project> for DeleteProject {
    fn from(project: Project) -> Self {
        Self { id: project.id }
    }
}

impl From<Project> for ArchiveProject {
    fn from(project: Project) -> Self {
        Self { id: project.id }
    }
}

impl From<Project> for UnarchiveProject {
    fn from(project: Project) -> Self {
        Self { id: project.id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_types() {
        pretty_assertions::assert_eq!(AddProject::default().command_type(), "project_add");
        pretty_assertions::assert_eq!(UpdateProject::default().command_type(), "project_update");
        pretty_assertions::assert_eq!(MoveProject::default().command_type(), "project_move");
        pretty_assertions::assert_eq!(
            MoveProjectIntoWorkspace::default().command_type(),
            "project_move_to_workspace"
        );
        pretty_assertions::assert_eq!(
            MoveProjectOutOfWorkspace::default().command_type(),
            "project_move_to_personal"
        );
        pretty_assertions::assert_eq!(DeleteProject::default().command_type(), "project_delete");
        pretty_assertions::assert_eq!(ArchiveProject::default().command_type(), "project_archive");
        pretty_assertions::assert_eq!(
            UnarchiveProject::default().command_type(),
            "project_unarchive"
        );
        pretty_assertions::assert_eq!(
            ChangeProjectRole::default().command_type(),
            "project_change_role"
        );
    }

    #[test]
    fn creates_resource() {
        assert!(AddProject::default().creates_resource());
        assert!(!UpdateProject::default().creates_resource());
        assert!(!MoveProject::default().creates_resource());
        assert!(!MoveProjectIntoWorkspace::default().creates_resource());
        assert!(!MoveProjectOutOfWorkspace::default().creates_resource());
        assert!(!DeleteProject::default().creates_resource());
        assert!(!ArchiveProject::default().creates_resource());
        assert!(!UnarchiveProject::default().creates_resource());
        assert!(!ChangeProjectRole::default().creates_resource());
    }

    #[test]
    fn use_lro_is_true() {
        // use_lro should always be true
        assert!(MoveProjectIntoWorkspace::default().use_lro);
        assert!(MoveProjectOutOfWorkspace::default().use_lro);
    }
}
