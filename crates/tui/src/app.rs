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
    command::{Command, CommandArgs, task::AddTask},
    types::{Id, Uid, task::Task},
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
pub struct App<'a> {
    pub view: ViewState,
    pub should_quit: bool,
    inbox_id: Id,
    user_id: Uid,
    client: APIClient,
    task_list: TaskList,
    add_task_popup: Option<AddTaskPopup<'a>>,
}

impl App<'_> {
    pub async fn new(mut client: APIClient) -> color_eyre::Result<Self> {
        let resp = client
            .sync(vec![ResourceType::Items, ResourceType::Projects])
            .await?;
        let tasks = resp.items.unwrap_or_default();
        // TODO: Add command to retrieve user data
        let user_id = tasks.first().ok_or_eyre("no tasks exist; this app currently requires some tasks to exist in order to determine the user ID because im lazy")?.user_id.clone();

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
            inbox_id,
            user_id,
            client,
            task_list: TaskList::new(tasks),
            add_task_popup: None,
        })
    }

    pub async fn tick(&mut self, event: KeyEvent) -> color_eyre::Result<()> {
        match self.handle_key(event) {
            Some(Action::OpenAddTaskModal) => self.start_adding_task(),
            Some(Action::CloseAddTaskModal) => self.cancel_adding_task(),
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
            .content(content.clone())
            .description(description.clone())
            .project_id(self.inbox_id.clone())
            .user_id(self.user_id.clone())
            .build();
        self.task_list.add_task(task.clone());

        let cmd = Command::<Box<dyn CommandArgs>>::new(Box::new(AddTask::from(task)));
        let resp = self.client.send(&[cmd]).await?;
        log::debug!("add task resp: {resp:#?}");

        self.cancel_adding_task();
        Ok(())
    }

    fn cancel_adding_task(&mut self) {
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
        let hints = Paragraph::new(Line::from(vec![
            Span::from("<j/k> ").blue(),
            Span::from("move "),
            Span::from("<a> ").blue(),
            Span::from("add task "),
        ]));
        frame.render_widget(hints, footer_area);

        if let Some(popup) = self.add_task_popup.as_ref() {
            popup.draw(frame, area);
        }
    }
}
