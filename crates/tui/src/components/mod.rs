use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
};
use ratatui_textarea::TextArea;

use crate::action::Action;

pub mod add_task_popup;
pub mod task_list;

pub trait Component {
    /// Either internally updates the component or returns an `Action` to be performed in an external piece of code.
    fn handle_key(&mut self, event: KeyEvent) -> Option<Action>;
    fn draw(&self, frame: &mut Frame, area: Rect);
}

fn inactivate(textarea: &mut TextArea, block_title: &str) {
    textarea.set_cursor_line_style(Style::default());
    textarea.set_cursor_style(Style::default());
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::DarkGray))
            .title(format!(" {block_title} (^X to switch) ")),
    );
}

fn activate(textarea: &mut TextArea, block_title: &str) {
    textarea.set_cursor_line_style(Style::default());
    textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(format!("{block_title}:")),
    );
}
