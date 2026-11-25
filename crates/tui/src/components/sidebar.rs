use crossterm::event::KeyCode;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, List, ListItem, ListState},
};
use todoist_sdk::types::Color as TodoistColor;
use todoist_sdk::types::project::Project;

use crate::{
    action::Action,
    components::{Component, Focusable, ScrollableList},
};

#[derive(Debug)]
pub struct Sidebar {
    pub focused: bool,
    projects: Vec<Project>,
    state: ListState,
}

impl Sidebar {
    pub fn new(projects: Vec<Project>) -> Self {
        let state = ListState::default().with_selected(Some(0));
        let focused = false;
        Self {
            focused,
            projects,
            state,
        }
    }
}

impl Component for Sidebar {
    fn draw(&self, frame: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect) {
        let project_items = self.projects.iter().map(|p| {
            let style = Style::default().fg(to_tui_color(&p.color));
            ListItem::new(p.name.clone()).style(style)
        });

        let highlight = if self.focused {
            Style::new().black().bg(Color::Yellow)
        } else {
            Style::default()
        };

        let sidebar = List::new(project_items)
            .block(Block::bordered())
            .highlight_style(highlight)
            .scroll_padding(1);

        frame.render_stateful_widget(sidebar, area, &mut self.state.clone());
    }

    fn handle_key(&mut self, event: crossterm::event::KeyEvent) -> Option<crate::action::Action> {
        match event.code {
            KeyCode::Char('k') => {
                self.scroll_up();
                None
            }
            KeyCode::Char('j') => {
                self.scroll_down();
                None
            }
            KeyCode::Char('l') => Some(Action::FocusTaskList),
            _ => None,
        }
    }
}

impl ScrollableList for Sidebar {
    fn item_count(&self) -> usize {
        self.projects.len()
    }

    fn list_state(&mut self) -> &mut ListState {
        &mut self.state
    }
}

impl Focusable for Sidebar {
    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}

fn to_tui_color(c: &TodoistColor) -> Color {
    match c {
        TodoistColor::BerryRed => Color::Red,
        TodoistColor::Red | TodoistColor::Salmon | TodoistColor::Taupe => Color::LightRed,
        TodoistColor::Orange => Color::LightYellow,
        TodoistColor::Yellow => Color::Yellow,
        TodoistColor::OliveGreen | TodoistColor::Green => Color::Green,
        TodoistColor::LimeGreen => Color::LightGreen,
        TodoistColor::MintGreen => Color::LightCyan,
        TodoistColor::Teal => Color::Cyan,
        TodoistColor::SkyBlue | TodoistColor::LightBlue => Color::LightBlue,
        TodoistColor::Blue => Color::Blue,
        TodoistColor::Grape | TodoistColor::Violet => Color::Magenta,
        TodoistColor::Lavender | TodoistColor::Magenta => Color::LightMagenta,
        TodoistColor::Charcoal => Color::DarkGray,
        TodoistColor::Grey => Color::Gray,
        &_ => unreachable!(),
    }
}
