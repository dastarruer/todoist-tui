use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{Block, List, ListItem, ListState},
};
use todoist_sdk::types::{Id, task::Task};

use crate::{action::Action, components::Component};

#[derive(Debug, Clone, Default)]
pub struct TaskList {
    tasks: Vec<Task>,
    state: ListState,
}

impl TaskList {
    pub fn new(tasks: Vec<Task>) -> Self {
        let state = if tasks.is_empty() {
            ListState::default()
        } else {
            ListState::default().with_selected(Some(0))
        };

        Self { tasks, state }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, id: &Id) {
        self.tasks = self.tasks.iter().filter(|t| &t.id != id).cloned().collect();
    }

    pub fn focused_task(&self) -> Option<&Task> {
        // `.get()` here is more convenient
        self.tasks
            .get(self.state.selected().expect("item should be selected"))
    }

    const fn scroll_up(&mut self) {
        if self.tasks.is_empty() {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.tasks.len().saturating_sub(1) // Wrap to the bottom
                } else {
                    i.saturating_sub(1) // Scroll up
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    const fn scroll_down(&mut self) {
        if self.tasks.is_empty() {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.tasks.len().saturating_sub(1) {
                    0 // Wrap back to the top
                } else {
                    i.saturating_add(1)
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

impl Component for TaskList {
    fn draw(&self, frame: &mut Frame, area: Rect) {
        let items = self
            .tasks
            .iter()
            .map(|t| ListItem::new(format!("[ ] {}", t.content.clone())));
        let list = List::new(items)
            .block(Block::bordered().gray())
            .style(Color::White)
            .highlight_style(Style::new().black().bg(Color::Yellow))
            .scroll_padding(1);

        frame.render_stateful_widget(list, area, &mut self.state.clone());
    }

    fn handle_key(&mut self, event: KeyEvent) -> Option<Action> {
        match event.code {
            KeyCode::Char('k') => {
                self.scroll_up();
                None
            }
            KeyCode::Char('j') => {
                self.scroll_down();
                None
            }
            KeyCode::Enter => Some(Action::CompleteTask),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a dummy Task for testing purposes
    fn create_dummy_task(content: &str) -> Task {
        Task {
            content: content.to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn test_empty_list_scrolling() {
        let mut list = TaskList::new(vec![]);

        list.scroll_down();
        pretty_assertions::assert_eq!(list.state.selected(), None);

        list.scroll_up();
        pretty_assertions::assert_eq!(list.state.selected(), None);
    }

    #[test]
    fn test_single_item_wrapping() {
        let tasks = vec![create_dummy_task("Only task")];
        let mut list = TaskList::new(tasks);

        pretty_assertions::assert_eq!(list.state.selected(), Some(0));

        // Scrolling down on a single item should wrap to itself (0 -> 0)
        list.scroll_down();
        pretty_assertions::assert_eq!(list.state.selected(), Some(0));

        // Scrolling up on a single item should wrap to itself (0 -> 0)
        list.scroll_up();
        pretty_assertions::assert_eq!(list.state.selected(), Some(0));
    }

    #[test]
    fn test_scroll_down() {
        let tasks = vec![
            create_dummy_task("Task 1"),
            create_dummy_task("Task 2"),
            create_dummy_task("Task 3"),
        ];
        let mut list = TaskList::new(tasks); // Starts at index 0

        pretty_assertions::assert_eq!(list.state.selected(), Some(0));

        // Normal scroll down: 0 -> 1
        list.scroll_down();
        pretty_assertions::assert_eq!(list.state.selected(), Some(1));

        // Normal scroll down: 1 -> 2
        list.scroll_down();
        pretty_assertions::assert_eq!(list.state.selected(), Some(2));

        // Edge case: Wrap from last item (2) back to top (0)
        list.scroll_down();
        pretty_assertions::assert_eq!(list.state.selected(), Some(0));
    }

    #[test]
    fn test_scroll_up() {
        let tasks = vec![
            create_dummy_task("Task 1"),
            create_dummy_task("Task 2"),
            create_dummy_task("Task 3"),
        ];
        let mut list = TaskList::new(tasks);

        // Edge case: Wrap from top item (0) to the bottom item (2)
        pretty_assertions::assert_eq!(list.state.selected(), Some(0));
        list.scroll_up();
        pretty_assertions::assert_eq!(list.state.selected(), Some(2));

        // Normal scroll up: 2 -> 1
        list.scroll_up();
        pretty_assertions::assert_eq!(list.state.selected(), Some(1));

        // Normal scroll up: 1 -> 0
        list.scroll_up();
        pretty_assertions::assert_eq!(list.state.selected(), Some(0));
    }

    #[test]
    fn test_uninitialized_state_fallback() {
        let tasks = vec![create_dummy_task("Task 1"), create_dummy_task("Task 2")];
        let mut list = TaskList::new(tasks);
        // Force state selection to None
        list.state.select(None);

        // Scrolling down from None should default safely to index 0
        list.scroll_down();
        pretty_assertions::assert_eq!(list.state.selected(), Some(0));
    }
}
