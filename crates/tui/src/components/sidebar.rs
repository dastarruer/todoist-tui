use ratatui::{
    style::{Color, Style},
    widgets::{Block, List, ListItem, ListState},
};
use todoist_sdk::types::Color as TodoistColor;
use todoist_sdk::types::project::Project;

use crate::components::Component;

#[derive(Debug)]
pub struct Sidebar {
    projects: Vec<Project>,
    state: ListState,
}

impl Sidebar {
    pub fn new(projects: Vec<Project>) -> Self {
        let state = ListState::default();
        Self { projects, state }
    }
}

impl Component for Sidebar {
    fn draw(&self, frame: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect) {
        let project_items = self.projects.iter().map(|p| {
            let style = Style::default().fg(to_tui_color(&p.color));
            ListItem::new(p.name.clone()).style(style)
        });
        let sidebar = List::new(project_items)
            .block(Block::bordered())
            .highlight_style(Style::new().black().bg(Color::Yellow))
            .scroll_padding(1);

        frame.render_stateful_widget(sidebar, area, &mut self.state.clone());
    }

    #[allow(unused_variables)]
    fn handle_key(&mut self, event: crossterm::event::KeyEvent) -> Option<crate::action::Action> {
        None
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
