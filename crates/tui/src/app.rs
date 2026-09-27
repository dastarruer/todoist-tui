use todoist_sdk::{APIClient, ResourceType, types::task::Task};

pub struct App {
    _client: APIClient,
    pub tasks: Vec<Task>,
    pub view: ViewState,
    highlighted_task_index: usize,
}

impl App {
    pub async fn new(mut client: APIClient) -> color_eyre::Result<Self> {
        let tasks = client
            .sync(vec![ResourceType::Items])
            .await?
            .items
            .expect("`items` should exist");
        Ok(Self {
            _client: client,
            tasks,
            view: ViewState::List,
            highlighted_task_index: 0,
        })
    }

    pub fn highlighted_task(&self) -> Option<&Task> {
        self.tasks.get(self.highlighted_task_index)
    }

    pub const fn move_up(&mut self) {
        self.highlighted_task_index = self.highlighted_task_index.saturating_sub(1);
    }

    pub fn move_down(&mut self) {
        self.highlighted_task_index = self
            .highlighted_task_index
            .saturating_add(1)
            .min(self.tasks.len().saturating_sub(1));
    }

    pub fn start_adding_task(&mut self) {
        self.view = ViewState::AddTask {
            buffer: String::new(),
        };
    }

    pub fn cancel_adding_task(&mut self) {
        self.view = ViewState::List;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewState {
    List,
    AddTask { buffer: String },
}
