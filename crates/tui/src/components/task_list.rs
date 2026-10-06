use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Row, Table},
};
use todoist_sdk::types::task::Task;

use crate::{action::Action, components::Component};

#[derive(Debug, Clone, Default)]
pub struct TaskList {
    tasks: Vec<Task>,
    highlighted_task_index: usize,
}

impl TaskList {
    pub const fn new(tasks: Vec<Task>) -> Self {
        Self {
            tasks,
            highlighted_task_index: 0,
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn highlighted_task(&self) -> Option<&Task> {
        // `.get()` here is more convenient
        self.tasks.get(self.highlighted_task_index)
    }

    const fn scroll_up(&mut self) {
        self.highlighted_task_index = self.highlighted_task_index.saturating_sub(1);
    }

    fn scroll_down(&mut self) {
        self.highlighted_task_index = self
            .highlighted_task_index
            .saturating_add(1)
            .min(self.tasks.len().saturating_sub(1));
    }
}

impl Component for TaskList {
    fn draw(&self, frame: &mut Frame, area: Rect) {
        let mut task_rows = Vec::<Row>::new();
        let highlighted_task = self.highlighted_task();
        for task in &self.tasks {
            let bg_color = if let Some(highlighted_task) = highlighted_task
                && task == highlighted_task
            {
                Color::White
            } else {
                Color::Reset
            };
            let task_style = Style::default().fg(Color::Yellow).bg(bg_color);

            let row = Row::new(vec![format!("[ ] {}", task.content)]).style(task_style);
            task_rows.push(row);
        }
        let tasks_table = Table::new(task_rows, [Constraint::Fill(1)])
            .block(Block::new().borders(Borders::LEFT | Borders::BOTTOM));
        frame.render_widget(tasks_table, area);
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
            _ => None,
        }
    }
}
