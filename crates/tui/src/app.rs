use crossterm::event::KeyEvent;
use ratatui_textarea::TextArea;
use todoist_sdk::{APIClient, ResourceType, types::task::Task};

#[derive(Debug)]
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
        self.view = ViewState::AddTask(AddTaskTextAreas::default());
    }

    pub fn cancel_adding_task(&mut self) {
        self.view = ViewState::List;
    }
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)] // Only constructed once
pub enum ViewState {
    List,
    AddTask(AddTaskTextAreas),
}

#[derive(Debug, Default, Clone)]
pub struct AddTaskTextAreas {
    textareas: [TextArea<'static>; 2],
    focused: usize,
}

impl AddTaskTextAreas {
    pub fn _focused_textarea(&self) -> &TextArea<'static> {
        self.textareas
            .get(self.focused)
            .expect("focused textarea should exist")
    }

    pub fn _focused_textarea_mut(&mut self) -> &mut TextArea<'static> {
        self.textareas
            .get_mut(self.focused)
            .expect("focused textarea should exist")
    }

    pub const fn content(&self) -> &TextArea<'static> {
        &self.textareas[0]
    }

    pub const fn desc(&self) -> &TextArea<'static> {
        &self.textareas[1]
    }

    pub const fn cycle_focus(&mut self) {
        self.focused = match self.focused {
            0 => 1,
            1 => 0,
            _ => unreachable!(),
        };
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        self.textareas
            .get_mut(self.focused)
            .expect("focused textarea should exist")
            .input(key);
    }
}
