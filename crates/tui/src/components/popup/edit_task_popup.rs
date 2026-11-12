use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, Clear},
};
use todoist_sdk::types::{Id, task::Task};

use crate::{
    action::Action,
    components::{
        Component,
        popup::{DATE_REGEX, TaskForm},
    },
};

#[derive(Debug)]
pub struct EditTaskPopup<'a> {
    form: TaskForm<'a>,
    task_id: Id,
}

impl EditTaskPopup<'_> {
    pub fn new(task: Task) -> Self {
        Self {
            form: TaskForm::new(task.content, &task.description),
            task_id: task.id,
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

    pub const fn id(&self) -> &Id {
        &self.task_id
    }

    fn is_addable_task(&self) -> bool {
        !self.content().trim().is_empty()
    }
}

impl Component for EditTaskPopup<'_> {
    #[allow(clippy::indexing_slicing)]
    fn draw(&self, frame: &mut Frame, area: Rect) {
        let area = area.centered(Constraint::Max(50), Constraint::Max(10));
        frame.render_widget(Clear, area);

        let popup = if self.is_addable_task() {
            Block::bordered()
                .title_top(Line::from(" edit task ").left_aligned())
                .title_bottom(
                    Line::from(vec![
                        Span::from("<alt + enter> ").blue(),
                        Span::from("submit"),
                    ])
                    .right_aligned(),
                )
        } else {
            Block::bordered()
        };

        frame.render_widget(&popup, area);
        self.form.draw(frame, popup.inner(area));
    }

    fn handle_key(&mut self, event: KeyEvent) -> Option<Action> {
        match event.code {
            KeyCode::Esc => Some(Action::ClosePopup),
            KeyCode::Enter
                if event.modifiers.contains(KeyModifiers::ALT) && self.is_addable_task() =>
            {
                Some(Action::CompleteEditTask)
            }
            _ => {
                self.form.handle_key(event);
                None
            }
        }
    }
}
