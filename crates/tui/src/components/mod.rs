use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect, widgets::ListState};

use crate::action::Action;

pub mod keybind_hints;
pub mod popup;
pub mod sidebar;
pub mod task_list;

pub trait Component {
    /// Either internally updates the component or returns an `Action` to be performed in an external piece of code.
    fn handle_key(&mut self, event: KeyEvent) -> Option<Action>;
    fn draw(&self, frame: &mut Frame, area: Rect);
}

pub trait ScrollableList: Component {
    fn item_count(&self) -> usize;
    fn list_state(&mut self) -> &mut ListState;

    fn scroll_up(&mut self) {
        let len = self.item_count();
        if len == 0 {
            return;
        }

        let state = self.list_state();
        let i = match state.selected() {
            Some(0) => len.saturating_sub(1), // wrap to bottom
            Some(i) => i.saturating_sub(1),
            None => 0,
        };
        state.select(Some(i));
    }

    fn scroll_down(&mut self) {
        let len = self.item_count();
        if len == 0 {
            return;
        }

        let state = self.list_state();
        let i = match state.selected() {
            Some(i) if i.saturating_add(1) >= len => 0, // wrap to top
            Some(i) => i.saturating_add(1),
            None => 0,
        };
        state.select(Some(i));
    }
}

pub trait Focusable: Component {
    fn focus(&mut self);
    fn unfocus(&mut self);
}

#[cfg(test)]
mod tests {
    use todoist_sdk::types::task::Task;

    use super::*;

    // Helper to create a dummy Task for testing purposes
    fn create_dummy_task(content: &str) -> Task {
        Task {
            content: content.to_string(),
            ..Default::default()
        }
    }

    struct TestList {
        items: Vec<Task>,
        state: ListState,
    }

    impl TestList {
        fn new(items: Vec<Task>) -> Self {
            Self {
                items,
                state: ListState::default(),
            }
        }
    }

    #[allow(unused_variables)]
    impl Component for TestList {
        fn draw(&self, frame: &mut Frame, area: Rect) {}
        fn handle_key(&mut self, event: KeyEvent) -> Option<Action> {
            None
        }
    }

    impl ScrollableList for TestList {
        fn item_count(&self) -> usize {
            self.items.len()
        }

        fn list_state(&mut self) -> &mut ListState {
            &mut self.state
        }
    }

    #[test]
    fn test_empty_list_scrolling() {
        let mut list = TestList::new(vec![]);

        list.scroll_down();
        pretty_assertions::assert_eq!(list.state.selected(), None);

        list.scroll_up();
        pretty_assertions::assert_eq!(list.state.selected(), None);
    }

    #[test]
    fn test_single_item_wrapping() {
        let tasks = vec![create_dummy_task("Only task")];
        let mut list = TestList::new(tasks);

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
        let mut list = TestList::new(tasks); // Starts at index 0

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
        let mut list = TestList::new(tasks);

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
        let mut list = TestList::new(tasks);
        // Force state selection to None
        list.state.select(None);

        // Scrolling down from None should default safely to index 0
        list.scroll_down();
        pretty_assertions::assert_eq!(list.state.selected(), Some(0));
    }
}
