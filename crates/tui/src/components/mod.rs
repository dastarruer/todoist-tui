use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};

use crate::action::Action;

pub mod keybind_hints;
pub mod popup;
pub mod task_list;

pub trait Component {
    /// Either internally updates the component or returns an `Action` to be performed in an external piece of code.
    fn handle_key(&mut self, event: KeyEvent) -> Option<Action>;
    fn draw(&self, frame: &mut Frame, area: Rect);
}
