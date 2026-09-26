use bon::Builder;
use serde::Serialize;

use crate::{
    command::CommandArgs,
    types::{
        Color, Id,
        project::{ProjectAccess, ProjectStatus, Role, ViewStyle},
    },
};

/// Add a new project.
#[derive(Serialize, Debug, Builder, Clone, PartialEq, Eq)]
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
    const TYPE: &str = "project_add";
    const CREATES_RESOURCE: bool = true;
}

/// Update an existing project.
#[derive(Serialize, Debug, Builder, Clone, PartialEq, Eq)]
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
    const TYPE: &str = "project_update";
    const CREATES_RESOURCE: bool = false;
}

/// Update the parent project of a project.
#[derive(Serialize, Debug, Builder, Clone, PartialEq, Eq)]
#[builder(on(String, into), on(Id, into))]
pub struct MoveProject {
    /// The ID of the project.
    pub id: String,
    /// The ID of the parent project. If set to `None`, the project will be
    /// moved to the root.
    pub parent_id: Option<String>,
}

impl CommandArgs for MoveProject {
    const TYPE: &str = "project_move";
    const CREATES_RESOURCE: bool = false;
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
    const TYPE: &str = "project_move_to_workspace";
    const CREATES_RESOURCE: bool = false;
}

/// Moves a project inside a workspace out back into a user's personal space.
///
/// Only the original creator of the project has permissions to do this, and
/// only if they are still currently an admin of said workspace.
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

impl CommandArgs for MoveProjectOutOfWorkspace {
    const TYPE: &str = "project_move_to_personal";
    const CREATES_RESOURCE: bool = false;
}

/// Delete an existing project and all its descendants.
///
/// Workspace projects can only be deleted by users with `Role::Admin` and it
/// must be archived first.
#[derive(Serialize, Debug, Builder, Clone, PartialEq, Eq)]
#[builder(on(String, into), on(Id, into))]
pub struct DeleteProject {
    /// The ID of the project to delete.
    pub id: Id,
}

impl CommandArgs for DeleteProject {
    const TYPE: &str = "project_delete";
    const CREATES_RESOURCE: bool = false;
}

/// Archive a project and its descendants.
#[derive(Serialize, Debug, Builder, Clone, PartialEq, Eq)]
#[builder(on(String, into), on(Id, into))]
pub struct ArchiveProject {
    /// The ID of the project to archive.
    pub id: Id,
}

impl CommandArgs for ArchiveProject {
    const TYPE: &str = "project_archive";
    const CREATES_RESOURCE: bool = false;
}

/// Unarchive a project and its descendants.
#[derive(Serialize, Debug, Builder, Clone, PartialEq, Eq)]
#[builder(on(String, into), on(Id, into))]
pub struct UnarchiveProject {
    /// The ID of the project to unarchive.
    pub id: Id,
}

impl CommandArgs for UnarchiveProject {
    const TYPE: &str = "project_unarchive";
    const CREATES_RESOURCE: bool = false;
}

/// Unarchive a project and its descendants.
#[derive(Serialize, Debug, Builder, Clone, PartialEq, Eq)]
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
    const TYPE: &str = "project_change_role";
    const CREATES_RESOURCE: bool = false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_types() {
        pretty_assertions::assert_eq!(AddProject::TYPE, "project_add");
        pretty_assertions::assert_eq!(UpdateProject::TYPE, "project_update");
        pretty_assertions::assert_eq!(MoveProject::TYPE, "project_move");
        pretty_assertions::assert_eq!(MoveProjectIntoWorkspace::TYPE, "project_move_to_workspace");
        pretty_assertions::assert_eq!(MoveProjectOutOfWorkspace::TYPE, "project_move_to_personal");
        pretty_assertions::assert_eq!(DeleteProject::TYPE, "project_delete");
        pretty_assertions::assert_eq!(ArchiveProject::TYPE, "project_archive");
        pretty_assertions::assert_eq!(UnarchiveProject::TYPE, "project_unarchive");
        pretty_assertions::assert_eq!(ChangeProjectRole::TYPE, "project_change_role");
    }

    #[test]
    fn creates_resource() {
        const {
            assert!(AddProject::CREATES_RESOURCE);
            assert!(!UpdateProject::CREATES_RESOURCE);
            assert!(!MoveProject::CREATES_RESOURCE);
            assert!(!MoveProjectIntoWorkspace::CREATES_RESOURCE);
            assert!(!MoveProjectOutOfWorkspace::CREATES_RESOURCE);
            assert!(!DeleteProject::CREATES_RESOURCE);
            assert!(!ArchiveProject::CREATES_RESOURCE);
            assert!(!UnarchiveProject::CREATES_RESOURCE);
            assert!(!ChangeProjectRole::CREATES_RESOURCE);
        }
    }

    #[test]
    fn use_lro_is_true() {
        // use_lro should always be true
        assert!(
            MoveProjectIntoWorkspace::builder()
                .id("123")
                .workspace_id("123")
                .build()
                .use_lro
        );
        assert!(
            MoveProjectOutOfWorkspace::builder()
                .id("123")
                .build()
                .use_lro
        );
    }
}
