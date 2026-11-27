use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{Block, List, ListItem, ListState},
};
use todoist_sdk::types::{Id, task::Task};

use crate::{
    action::Action,
    components::{Component, Focusable, ScrollableList},
};

#[derive(Debug, Clone, Default)]
pub struct TaskList {
    pub focused: bool,
    tasks: Vec<Task>,
    state: ListState,
}

impl TaskList {
    pub fn new(tasks: Vec<Task>) -> Self {
        let state = ListState::default().with_selected(Some(0));
        let focused = true;
        Self {
            focused,
            tasks,
            state,
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, id: &Id) {
        self.tasks = self.tasks.iter().filter(|t| &t.id != id).cloned().collect();
    }

    pub fn update_task(&mut self, task: Task) {
        if let Some(t) = self.tasks.iter_mut().find(|t| t.id == task.id) {
            *t = task;
        }
    }

    pub fn focused_task(&self) -> Option<&Task> {
        // `.get()` here is more convenient
        self.tasks.get(self.state.selected()?)
    }
}

impl Component for TaskList {
    fn draw(&self, frame: &mut Frame, area: Rect) {
        let items = self
            .tasks
            .iter()
            .map(|t| ListItem::new(format!("[ ] {}", t.content.clone())));

        let highlight = if self.focused {
            Style::new().black().bg(Color::Yellow)
        } else {
            Style::default()
        };

        let list = List::new(items)
            .block(Block::bordered().gray())
            .style(Color::White)
            .highlight_style(highlight)
            .scroll_padding(1);

        frame.render_stateful_widget(list, area, &mut self.state.clone());
    }

    fn handle_key(&mut self, event: KeyEvent) -> Option<Action> {
        match event.code {
            KeyCode::Char('k') => {
                self.scroll_up();
                None
            }
            KeyCode::Char('j') => {
                self.scroll_down();
                None
            }
            KeyCode::Char('e') => Some(Action::OpenEditTaskPopup),
            KeyCode::Char('h') => Some(Action::FocusSidebar),
            KeyCode::Char('a') => Some(Action::OpenAddTaskPopup),
            KeyCode::Enter => Some(Action::CompleteTask),
            _ => None,
        }
    }
}

impl ScrollableList for TaskList {
    fn item_count(&self) -> usize {
        self.tasks.len()
    }

    fn list_state(&mut self) -> &mut ListState {
        &mut self.state
    }
}

impl Focusable for TaskList {
    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}
