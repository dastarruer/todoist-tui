#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    OpenAddTaskModal,
    CloseAddTaskModal,
    CompleteTask,
    AddTask,
}
