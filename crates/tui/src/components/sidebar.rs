use ratatui::{
    style::{Color, Style},
    widgets::{Block, List, ListItem, ListState},
};
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
        let project_items = self.projects.iter().map(|p| ListItem::new(p.name.clone()));
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
