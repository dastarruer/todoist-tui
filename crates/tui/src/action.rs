#[derive(Debug, Clone, Copy)]
pub enum Action {
    Quit,
    OpenAddTaskModal,
    CloseAddTaskModal,
    AddTask,
}
