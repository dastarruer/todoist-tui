use std::collections::HashMap;

use color_eyre::eyre::OptionExt;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use todoist_sdk::{
    APIClient, ResourceType,
    command::{
        Command, CommandArgs,
        task::{AddTask, CompleteTask},
    },
    types::{Id, task::Task, user::User},
};

use crate::{
    action::Action,
    components::{Component, add_task_popup::AddTaskPopup, task_list::TaskList},
};

#[derive(Debug, PartialEq, Eq)]
pub enum ViewState {
    List,
    AddTask,
}

#[derive(Debug)]
struct KeybindHints {
    hints: HashMap<String, String>,
}

impl KeybindHints {
    fn new() -> Self {
        let hints = HashMap::from([
            (String::from("<j/k>"), String::from("move")),
            (String::from("<a>"), String::from("add task")),
        ]);
        Self { hints }
    }

    /// Empty all bind hints.
    fn empty(&mut self) {
        self.hints = HashMap::new();
    }

    /// Return all keybind hints as a `Vec<Span>` to be used in ratatui
    /// components.
    fn spans(&self) -> Vec<Span<'_>> {
        self.hints
            .iter()
            .flat_map(|(bind, hint)| {
                [
                    Span::from(format!("{bind} ")).blue(),
                    Span::from(format!("{hint} ")),
                ]
            })
            .collect::<Vec<Span>>()
    }
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
    add_task_popup: Option<AddTaskPopup<'a>>,
}

impl App<'_> {
    pub async fn new(mut client: APIClient) -> color_eyre::Result<Self> {
        let hints = KeybindHints::new();

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

        Ok(Self {
            view: ViewState::List,
            should_quit: false,
            hints,
            inbox_id,
            user,
            client,
            task_list: TaskList::new(tasks),
            add_task_popup: None,
        })
    }

    pub async fn tick(&mut self, event: KeyEvent) -> color_eyre::Result<()> {
        match self.handle_key(event) {
            Some(Action::OpenAddTaskModal) => self.start_adding_task(),
            Some(Action::CloseAddTaskModal) => self.cancel_adding_task(),
            Some(Action::CompleteTask) => {
                if let Some(task) = self.task_list.focused_task() {
                    self.complete_task(task.clone()).await?;
                }
            }
            Some(Action::AddTask) => {
                self.add_task(
                    self.add_task_popup
                        .as_ref()
                        .expect("popup should exist")
                        .content(),
                    self.add_task_popup
                        .as_ref()
                        .expect("popup should exist")
                        .desc(),
                )
                .await?;
            }
            Some(Action::Quit) => self.should_quit = true,
            None => {}
        }
        Ok(())
    }

    fn start_adding_task(&mut self) {
        self.hints.empty();
        self.view = ViewState::AddTask;
        self.add_task_popup = Some(AddTaskPopup::new());
    }

    async fn add_task(
        &mut self,
        content: String,
        description: Option<String>,
    ) -> color_eyre::Result<()> {
        let description = description.unwrap_or_default();
        let task = Task::builder()
            .content(content)
            .description(description)
            .project_id(self.inbox_id.clone())
            .user_id(self.user.id.clone())
            .build();
        self.task_list.add_task(task.clone());

        let cmd = Command::<Box<dyn CommandArgs>>::new(Box::new(AddTask::from(task)));
        let resp = self.client.send(&[cmd]).await?;
        log::debug!("add task resp: {resp:#?}");

        self.cancel_adding_task();
        Ok(())
    }

    async fn complete_task(&mut self, task: Task) -> color_eyre::Result<()> {
        self.task_list.remove_task(&task.id);
        let cmd = Command::<Box<dyn CommandArgs>>::new(Box::new(CompleteTask::from(task)));
        let resp = self.client.send(&[cmd]).await?;
        log::debug!("add task resp: {resp:#?}");

        Ok(())
    }

    fn cancel_adding_task(&mut self) {
        self.hints = KeybindHints::new();
        self.view = ViewState::List;
        self.add_task_popup = None;
    }
}

impl Component for App<'_> {
    fn handle_key(&mut self, event: KeyEvent) -> Option<Action> {
        if self.view == ViewState::List {
            match event.code {
                KeyCode::Char('q') | KeyCode::Esc => return Some(Action::Quit),
                KeyCode::Char('a') => return Some(Action::OpenAddTaskModal),
                _ => {}
            }
        }
        match self.view {
            ViewState::List => self.task_list.handle_key(event),
            ViewState::AddTask => self.add_task_popup.as_mut()?.handle_key(event),
        }
    }

    #[allow(clippy::indexing_slicing)]
    fn draw(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(95), Constraint::Max(10)])
            .split(area);
        let footer_area = chunks[1];
        let view_area = chunks[0];

        // Sidebar and tasks
        let view = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(30), Constraint::Percentage(100)])
            .split(view_area);
        let sidebar_area = view[0];
        let tasks_area = view[1];

        self.task_list.draw(frame, tasks_area);

        let sidebar = List::new(Vec::<ListItem>::new())
            .block(Block::default().borders(Borders::RIGHT | Borders::BOTTOM));
        frame.render_widget(sidebar, sidebar_area);

        // Keybind hints
        let hints = Paragraph::new(Line::from(self.hints.spans()));
        frame.render_widget(hints, footer_area);

        if let Some(popup) = self.add_task_popup.as_ref() {
            popup.draw(frame, area);
        }
    }
}
