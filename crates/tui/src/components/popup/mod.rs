use std::sync::LazyLock;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::Block,
};
use ratatui_textarea::TextArea;
use regex::Regex;

use crate::{
    action::Action,
    components::{
        Component,
        popup::{add_task_popup::AddTaskPopup, edit_task_popup::EditTaskPopup},
    },
};

pub mod add_task_popup;
pub mod edit_task_popup;

const DATE_PATTERN: &str = r"\\\\.*";
/// Selects every string starting with `\\`.
pub static DATE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(DATE_PATTERN).expect("DATE_PATTERN is a valid regex"));

#[derive(Debug)]
pub enum Popup<'a> {
    AddTask(AddTaskPopup<'a>),
    EditTask(EditTaskPopup<'a>),
}

impl Component for Popup<'_> {
    fn handle_key(&mut self, event: KeyEvent) -> Option<Action> {
        match self {
            Self::AddTask(p) => p.handle_key(event),
            Self::EditTask(p) => p.handle_key(event),
        }
    }

    fn draw(&self, frame: &mut Frame, area: Rect) {
        match self {
            Self::AddTask(p) => p.draw(frame, area),
            Self::EditTask(p) => p.draw(frame, area),
        }
    }
}

#[derive(Default, Debug)]
pub struct TaskForm<'a> {
    textareas: [TextArea<'a>; 2],
    focused_index: usize,
}

impl TaskForm<'_> {
    fn new(content: String, desc: &str) -> Self {
        let mut content = TextArea::new(vec![content]);
        content
            .set_search_pattern(DATE_PATTERN)
            .expect("DATE_PATTERN is a valid regex");
        content.set_search_style(Style::new().on_red());

        let desc = desc.split('\n').map(str::to_owned).collect();
        let mut desc = TextArea::new(desc);

        activate(&mut content, "Task name");
        inactivate(&mut desc, "Description");
        let textareas = [content, desc];
        Self {
            textareas,
            ..Default::default()
        }
    }

    fn cycle_focus(&mut self) {
        self.focused_index = match self.focused_index {
            0 => {
                inactivate(&mut self.textareas[0], "Task name");
                activate(&mut self.textareas[1], "Description");
                1
            }
            1 => {
                inactivate(&mut self.textareas[1], "Description");
                activate(&mut self.textareas[0], "Task name");
                0
            }
            _ => unreachable!(),
        };
    }

    /// # Panics
    ///
    /// Panics if:
    ///
    /// - `index` is out of bounds.
    fn text(&self, index: usize) -> String {
        #[allow(clippy::indexing_slicing)]
        self.textareas[index].lines().join("\n")
    }
}

impl Component for TaskForm<'_> {
    fn handle_key(&mut self, event: KeyEvent) -> Option<Action> {
        match event.code {
            KeyCode::Tab => self.cycle_focus(),
            _ => {
                self.textareas
                    .get_mut(self.focused_index)
                    .expect("textarea should exist")
                    .input(event);
            }
        }
        None
    }

    #[allow(clippy::indexing_slicing)]
    fn draw(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(area);
        let content_area = chunks[0];
        let desc_area = chunks[1];

        let content = self.textareas.first();
        frame.render_widget(content, content_area);

        let desc = self.textareas.get(1);
        frame.render_widget(desc, desc_area);
    }
}

fn inactivate(textarea: &mut TextArea, block_title: &str) {
    textarea.set_cursor_line_style(Style::default());
    textarea.set_cursor_style(Style::default());
    textarea.set_block(
        Block::bordered()
            .style(Style::default().fg(Color::DarkGray))
            .title(format!(" {block_title} (<tab> to toggle focus) ")),
    );
}

fn activate(textarea: &mut TextArea, block_title: &str) {
    textarea.set_cursor_line_style(Style::default());
    textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    textarea.set_block(
        Block::bordered()
            .style(Style::default())
            .title(format!("{block_title}:")),
    );
}
