use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Clear},
};
use ratatui_textarea::TextArea;

use crate::{
    action::Action,
    components::{Component, activate, inactivate},
};

#[derive(Default, Debug)]
pub struct AddTaskPopup<'a> {
    textareas: [TextArea<'a>; 2],
    focused_index: usize,
}

impl<'a> AddTaskPopup<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    const fn is_focused(&self, index: usize) -> bool {
        self.focused_index == index
    }

    const fn content(&self) -> &TextArea<'a> {
        &self.textareas[0]
    }

    const fn desc(&self) -> &TextArea<'a> {
        &self.textareas[1]
    }

    const fn cycle_focus(&mut self) {
        self.focused_index = match self.focused_index {
            0 => 1,
            1 => 0,
            _ => unreachable!(),
        };
    }
}

impl Component for AddTaskPopup<'_> {
    #[allow(clippy::indexing_slicing)]
    fn draw(&self, frame: &mut Frame, area: Rect) {
        let area = area.centered(Constraint::Max(50), Constraint::Max(10));
        let popup = Block::bordered();
        frame.render_widget(Clear, area);
        frame.render_widget(&popup, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(popup.inner(area));
        let content_area = chunks[0];
        let desc_area = chunks[1];

        let mut content = self.content().to_owned();
        if self.is_focused(0) {
            activate(&mut content, "Task name");
        } else {
            inactivate(&mut content, "Task name");
        }
        frame.render_widget(&content, content_area);

        let mut desc = self.desc().to_owned();
        if self.is_focused(1) {
            activate(&mut desc, "Description");
        } else {
            inactivate(&mut desc, "Description");
        }
        frame.render_widget(&desc, desc_area);
    }

    fn handle_key(&mut self, event: KeyEvent) -> Option<Action> {
        match event.code {
            KeyCode::Char('x') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.cycle_focus();
                None
            }
            KeyCode::Esc => Some(Action::CloseAddTaskModal),
            _ => {
                #[allow(clippy::indexing_slicing)]
                self.textareas[self.focused_index].input(event);
                None
            }
        }
    }
}
