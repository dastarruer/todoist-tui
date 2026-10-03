mod action;
mod app;
mod components;

use std::{env::var, io::Stdout, path::PathBuf};

use color_eyre::eyre::Context;
use flexi_logger::{FileSpec, Logger};
use ratatui::crossterm::event::{self, Event};
use ratatui::{Terminal, backend::CrosstermBackend};
use todoist_sdk::APIClient;

use crate::app::App;
use crate::components::Component;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let app = bootstrap_app().await?;
    let mut terminal = ratatui::init();
    run_app(&mut terminal, app)?;
    ratatui::restore();

    Ok(())
}

async fn bootstrap_app() -> color_eyre::Result<App<'static>> {
    color_eyre::install()?;
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
                .basename("todoist-tui"),
        )
        .rotate(
            flexi_logger::Criterion::Size(1_000_000),
            flexi_logger::Naming::Numbers,
            flexi_logger::Cleanup::KeepLogFiles(5),
        )
        .start();

    let key = retrieve_api_key().context("unable to retrieve API key")?;
    let client = APIClient::new(key);
    App::new(client).await
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut app: App,
) -> color_eyre::Result<bool> {
    while !app.should_quit {
        terminal.draw(|f| app.draw(f, f.area()))?;
        if let Event::Key(event) = event::read()? {
            app.tick(event);
        }
    }
    Ok(true)
}

fn retrieve_api_key() -> color_eyre::Result<String> {
    Ok(var("API_KEY")?)
}
