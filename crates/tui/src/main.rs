mod app;
mod ui;

use std::{
    env::var,
    io::{self, Stdout},
    path::PathBuf,
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use flexi_logger::{FileSpec, Logger};
use ratatui::{Terminal, backend::CrosstermBackend};
use todoist_sdk::APIClient;

use crate::{app::App, ui::ui};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    bootstrap_app();
    let key = retrieve_api_key().unwrap_or_else(|e| {
        log::error!("Error retrieving API key: {e}");
        std::process::exit(1);
    });
    let client = APIClient::new(key);
    let app = App::new(client).await?;

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    enable_raw_mode()?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;
    run_app(&mut terminal, &app)?;
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn bootstrap_app() {
    let log_dir = std::env::var("XDG_STATE_HOME")
        .map_or_else(
            |_| {
                let home = std::env::var("HOME").expect("HOME is not set");
                PathBuf::from(home).join(".local/state")
            },
            PathBuf::from,
        )
        .join("todoist-tui");
    let _logger = Logger::try_with_env()
        .expect("Value of RUST_LOG is malformed")
        .log_to_file(
            FileSpec::default()
                .directory(log_dir)
                .basename("gtkshutdown"),
        )
        .duplicate_to_stdout(flexi_logger::Duplicate::Trace)
        .rotate(
            flexi_logger::Criterion::Size(1_000_000),
            flexi_logger::Naming::Numbers,
            flexi_logger::Cleanup::KeepLogFiles(5),
        )
        .start();
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &App,
) -> color_eyre::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        // probably will need to add other keybinds in the future anyways
        #[allow(clippy::collapsible_if)]
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
}

fn retrieve_api_key() -> color_eyre::Result<String> {
    Ok(var("API_KEY")?)
}
