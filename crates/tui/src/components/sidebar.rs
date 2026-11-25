use ratatui::widgets::{Block, List, ListItem};

use crate::components::Component;

#[derive(Debug, Clone, Default)]
pub struct Sidebar;

impl Component for Sidebar {
    fn draw(&self, frame: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect) {
        let sidebar = List::new(Vec::<ListItem>::new()).block(Block::bordered());
        frame.render_widget(sidebar, area);
    }

    #[allow(unused_variables)]
    fn handle_key(&mut self, event: crossterm::event::KeyEvent) -> Option<crate::action::Action> {
        None
    }
}
