use std::fmt::{Display, Formatter};

use crate::components::Component;
use ratatui::{
    style::Stylize,
    text::{Line, Span},
    widgets::Paragraph,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key(pub String);

impl Display for Key {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Action(pub String);

impl Display for Action {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct KeybindHints {
    hints: Vec<(Key, Action)>,
    hidden: bool,
}

impl KeybindHints {
    pub fn new() -> Self {
        let hints = vec![
            (Key(String::from("j/k")), Action(String::from("move"))),
            (Key(String::from("a")), Action(String::from("add"))),
            (Key(String::from("enter")), Action(String::from("complete"))),
        ];
        Self {
            hints,
            hidden: false,
        }
    }

    /// Show keybind hints.
    pub const fn hide(&mut self) {
        self.hidden = true;
    }

    /// Show keybind hints.
    pub const fn show(&mut self) {
        self.hidden = false;
    }

    /// Return all keybind hints as a `Vec<Span>` to be used in ratatui
    /// components.
    fn spans(&self) -> Vec<Span<'_>> {
        let mut spans = self
            .hints
            .iter()
            .flat_map(|(key, action)| {
                [
                    Span::from(format!("{action}: ")).blue(),
                    Span::from(format!("<{key}>")).blue(),
                    Span::from(String::from(" | ")).dark_gray(),
                ]
            })
            .collect::<Vec<Span>>();
        spans.pop(); // remove last separator
        spans
    }
}

impl Component for KeybindHints {
    fn draw(&self, frame: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect) {
        if self.hidden {
            return;
        }

        let hints = Paragraph::new(Line::from(self.spans()));
        frame.render_widget(hints, area);
    }

    // no keybinds to be handled!
    #[allow(unused_variables)]
    fn handle_key(&mut self, event: crossterm::event::KeyEvent) -> Option<crate::action::Action> {
        None
    }
}
