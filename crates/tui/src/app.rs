use todoist_sdk::{APIClient, ResourceType, types::task::Task};

pub struct App {
    _client: APIClient,
    pub tasks: Vec<Task>,
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
        })
    }
}
