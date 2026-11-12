use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, HorizontalAlignment, Rect},
    style::Stylize,
    text::Span,
    widgets::{Block, Clear},
};

use crate::{
    action::Action,
    components::{
        Component,
        popup::{DATE_REGEX, TaskForm},
    },
};

#[derive(Debug)]
pub struct AddTaskPopup<'a> {
    form: TaskForm<'a>,
}

impl AddTaskPopup<'_> {
    pub fn new() -> Self {
        Self {
            form: TaskForm::new(),
        }
    }

    pub fn content(&self) -> String {
        let content = self.form.text(0);
        DATE_REGEX.replace_all(&content, "").into_owned() // Remove date strings
    }

    pub fn desc(&self) -> Option<String> {
        Some(self.form.text(1)).filter(|d| !d.is_empty())
    }

    pub fn date_string(&self) -> Option<String> {
        let dates = String::from(DATE_REGEX.find(self.form.text(0).as_str())?.as_str());
        Some(dates).filter(|d| !d.is_empty())
    }

    fn is_addable_task(&self) -> bool {
        !self.content().trim().is_empty()
    }
}

impl Component for AddTaskPopup<'_> {
    #[allow(clippy::indexing_slicing)]
    fn draw(&self, frame: &mut Frame, area: Rect) {
        let area = area.centered(Constraint::Max(50), Constraint::Max(10));
        frame.render_widget(Clear, area);

        let popup = if self.is_addable_task() {
            Block::bordered()
                .title_bottom(vec![
                    Span::from("<alt + enter> ").blue(),
                    Span::from("submit"),
                ])
                .title_alignment(HorizontalAlignment::Right)
        } else {
            Block::bordered()
        };

        frame.render_widget(&popup, area);
        self.form.draw(frame, popup.inner(area));
    }

    fn handle_key(&mut self, event: KeyEvent) -> Option<Action> {
        match event.code {
            KeyCode::Esc => Some(Action::CloseAddTaskModal),
            KeyCode::Enter
                if event.modifiers.contains(KeyModifiers::ALT) && self.is_addable_task() =>
            {
                Some(Action::AddTask)
            }
            _ => {
                self.form.handle_key(event);
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn type_string_into_popup(popup: &mut AddTaskPopup, string: &str) {
        string.chars().for_each(|c| {
            popup.handle_key(KeyEvent::from(KeyCode::Char(c)));
        });
    }

    #[test]
    fn addable_tasks() {
        let mut popup = AddTaskPopup::new();
        assert!(!popup.is_addable_task());

        type_string_into_popup(&mut popup, " ");
        assert!(!popup.is_addable_task());

        type_string_into_popup(&mut popup, "c");
        assert!(popup.is_addable_task());

        let mut popup = AddTaskPopup::new();
        type_string_into_popup(&mut popup, r" \\today");
        assert!(!popup.is_addable_task());
    }

    #[test]
    fn adding_valid_tasks() {
        let mut popup = AddTaskPopup::new();
        let action = popup.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::ALT));
        assert_eq!(action, None); // Empty task content, so should not be added

        popup.handle_key(KeyEvent::from(KeyCode::Char('c')));
        let action = popup.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::ALT));
        assert_eq!(action, Some(Action::AddTask)); // Since there is content, task should be added
    }
}
