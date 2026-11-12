#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    OpenAddTaskPopup,
    ClosePopup,
    OpenEditTaskPopup,
    CompleteEditTask,
    CompleteTask,
    AddTask,
}
