use color_eyre::eyre::{OptionExt, bail};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, List, ListItem},
};
use todoist_sdk::{
    APIClient, ResourceType,
    command::{
        Command, CommandArgs,
        task::{AddTask, CompleteTask, UpdateTask},
    },
    types::{
        Id,
        task::{DueDate, Task},
        user::User,
    },
};

use crate::{
    action::Action,
    components::{
        Component,
        keybind_hints::{ActionHint, Key, KeybindHints},
        popup::{Popup, add_task_popup::AddTaskPopup, edit_task_popup::EditTaskPopup},
        task_list::TaskList,
    },
};

#[derive(Debug, PartialEq, Eq)]
pub enum ViewState {
    List,
    TaskPopup,
}

#[derive(Debug)]
pub struct App<'a> {
    pub view: ViewState,
    pub should_quit: bool,
    hints: KeybindHints,
    inbox_id: Id,
    user: User,
    client: APIClient,
    task_list: TaskList,
    popup: Option<Popup<'a>>,
}

impl App<'_> {
    pub async fn new(mut client: APIClient) -> color_eyre::Result<Self> {
        let resp = client
            .sync(vec![
                ResourceType::Items,
                ResourceType::Projects,
                ResourceType::User,
            ])
            .await?;
        log::debug!("initial sync response: {resp:#?}");

        let user = resp
            .user
            .ok_or_eyre("user not available on Todoist backend")?;
        let tasks = resp.items.unwrap_or_default();
        let projects = resp.projects.unwrap_or_default();
        let inbox_id = projects
            .iter()
            .find(|p| p.inbox_project)
            .expect("Inbox project should exist")
            .id
            .clone();

        let mut hints = KeybindHints::new();
        let task_list = TaskList::new(tasks);
        let edit_hint = (Key(String::from("e")), ActionHint(String::from("edit")));
        if task_list.focused_task().is_some() {
            hints.push(vec![edit_hint]);
        }

        Ok(Self {
            view: ViewState::List,
            should_quit: false,
            hints,
            inbox_id,
            user,
            client,
            task_list,
            popup: None,
        })
    }

    pub async fn tick(&mut self, event: KeyEvent) -> color_eyre::Result<()> {
        match self.handle_key(event) {
            Some(Action::OpenAddTaskPopup) => self.start_adding_task(),
            Some(Action::ClosePopup) => self.close_popup(),
            Some(Action::OpenEditTaskPopup) => self.start_editing_task(
                self.task_list
                    .focused_task()
                    .expect("task should be highlighted to be able to edit")
                    .to_owned(),
            ),
            Some(Action::CompleteTask) => {
                if let Some(task) = self.task_list.focused_task() {
                    self.complete_task(task.clone()).await?;
                }
            }
            Some(Action::AddTask) => {
                let Some(Popup::AddTask(popup)) = self.popup.as_ref() else {
                    bail!("add task popup does not exist despite receiving `AddTask` action")
                };
                self.add_task(popup.content(), popup.desc(), popup.date_string())
                    .await?;
            }
            Some(Action::CompleteEditTask) => {
                let Some(Popup::EditTask(popup)) = self.popup.as_ref() else {
                    bail!(
                        "add task popup does not exist despite receiving `CompleteEditTask` action"
                    )
                };
                self.edit_task(
                    popup.id().clone(),
                    popup.content(),
                    popup.desc(),
                    popup.date_string(),
                )
                .await?;
            }
            Some(Action::Quit) => self.should_quit = true,
            None => {}
        }
        Ok(())
    }

    fn start_adding_task(&mut self) {
        self.hints.hide();
        self.view = ViewState::TaskPopup;
        self.popup = Some(Popup::AddTask(AddTaskPopup::new()));
    }

    async fn add_task(
        &mut self,
        content: String,
        description: Option<String>,
        date_str: Option<String>,
    ) -> color_eyre::Result<()> {
        let due = date_str.map(DueDate::from_natural_language_string);
        let description = description.unwrap_or_default();
        let task = Task::builder()
            .content(content)
            .description(description)
            .project_id(self.inbox_id.clone())
            .user_id(self.user.id.clone())
            .maybe_due(due)
            .build();
        self.task_list.add_task(task.clone());

        let cmd = Command::<Box<dyn CommandArgs>>::new(Box::new(AddTask::from(task)));
        let resp = self.client.send(&[cmd]).await?;
        log::debug!("add task resp: {resp:#?}");

        self.close_popup();
        Ok(())
    }

    async fn edit_task(
        &mut self,
        id: Id,
        content: String,
        description: Option<String>,
        date_str: Option<String>,
    ) -> color_eyre::Result<()> {
        let due = date_str.map(DueDate::from_natural_language_string);
        let description = description.unwrap_or_default();
        let task = Task::builder()
            .id(id)
            .content(content)
            .description(description)
            .project_id(self.inbox_id.clone())
            .user_id(self.user.id.clone())
            .maybe_due(due)
            .build();

        let cmd = Command::<Box<dyn CommandArgs>>::new(Box::new(UpdateTask::from(task.clone())));
        let resp = self.client.send(&[cmd]).await?;
        log::debug!("add task resp: {resp:#?}");

        self.close_popup();
        self.task_list.update_task(task);
        Ok(())
    }

    async fn complete_task(&mut self, task: Task) -> color_eyre::Result<()> {
        self.task_list.remove_task(&task.id);
        let cmd = Command::<Box<dyn CommandArgs>>::new(Box::new(CompleteTask::from(task)));
        let resp = self.client.send(&[cmd]).await?;
        log::debug!("add task resp: {resp:#?}");

        Ok(())
    }

    fn close_popup(&mut self) {
        self.hints.show();
        self.view = ViewState::List;
        self.popup = None;
    }

    fn start_editing_task(&mut self, task: Task) {
        self.hints.hide();
        self.view = ViewState::TaskPopup;
        self.popup = Some(Popup::EditTask(EditTaskPopup::new(task)));
    }
}

impl Component for App<'_> {
    fn handle_key(&mut self, event: KeyEvent) -> Option<Action> {
        if self.view == ViewState::List {
            match event.code {
                KeyCode::Char('q') | KeyCode::Esc => return Some(Action::Quit),
                KeyCode::Char('a') => return Some(Action::OpenAddTaskPopup),
                _ => {}
            }
        }
        match self.view {
            ViewState::List => self.task_list.handle_key(event),
            ViewState::TaskPopup => self.popup.as_mut()?.handle_key(event),
        }
    }

    #[allow(clippy::indexing_slicing)]
    fn draw(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(95), Constraint::Max(10)])
            .split(area);
        let bind_hints_area = chunks[1];
        let view_area = chunks[0];

        // Sidebar and tasks
        let view = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(30), Constraint::Percentage(100)])
            .split(view_area);
        let sidebar_area = view[0];
        let tasks_area = view[1];

        self.task_list.draw(frame, tasks_area);

        let sidebar = List::new(Vec::<ListItem>::new()).block(Block::bordered());
        frame.render_widget(sidebar, sidebar_area);

        self.hints.draw(frame, bind_hints_area);

        if let Some(popup) = self.popup.as_ref() {
            popup.draw(frame, area);
        }
    }
}
