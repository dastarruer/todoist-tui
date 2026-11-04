use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use crate::components::Component;
use ratatui::{
    style::Stylize,
    text::{Line, Span},
    widgets::Paragraph,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Key(pub String);

impl Display for Key {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action(pub String);

impl Display for Action {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct KeybindHints {
    hints: HashMap<Key, Action>,
}

impl KeybindHints {
    pub fn new() -> Self {
        let hints = HashMap::from([
            (Key(String::from("<j/k>")), Action(String::from("move"))),
            (Key(String::from("<a>")), Action(String::from("add task"))),
        ]);
        Self { hints }
    }

    /// Push a new set of keybind hints.
    pub fn push(&mut self, hints: HashMap<Key, Action>) {
        self.hints = hints;
    }

    /// Return all keybind hints as a `Vec<Span>` to be used in ratatui
    /// components.
    fn spans(&self) -> Vec<Span<'_>> {
        self.hints
            .iter()
            .flat_map(|(bind, hint)| {
                [
                    Span::from(format!("{bind} ")).blue(),
                    Span::from(format!("{hint} ")),
                ]
            })
            .collect::<Vec<Span>>()
    }
}

impl Component for KeybindHints {
    fn draw(&self, frame: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect) {
        let hints = Paragraph::new(Line::from(self.spans()));
        frame.render_widget(hints, area);
    }

    // no keybinds to be handled!
    #[allow(unused_variables)]
    fn handle_key(&mut self, event: crossterm::event::KeyEvent) -> Option<crate::action::Action> {
        None
    }
}
