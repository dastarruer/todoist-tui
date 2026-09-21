use todoist_sdk::{APIClient, types::task::Task};

pub struct App {
    _client: APIClient,
    pub tasks: Vec<Task>,
}

impl App {
    pub async fn new(client: APIClient) -> color_eyre::Result<Self> {
        let tasks = client.tasks().await?;
        Ok(Self {
            _client: client,
            tasks,
        })
    }
}
