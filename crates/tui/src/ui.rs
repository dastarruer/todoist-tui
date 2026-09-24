use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Row, Table},
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(95), Constraint::Max(10)])
        .split(frame.area());

    // Sidebar and tasks
    let view = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(30), Constraint::Percentage(100)])
        .split(
            *chunks
                .first()
                .expect("first element of chunks should exist"),
        );

    let mut task_rows = Vec::<Row>::new();
    for task in &app.tasks {
        let mut task_style = Style::default().fg(Color::Yellow);
        if let Some(highlighted_task) = &app.highlighted_task
            && task == highlighted_task
        {
            task_style = task_style.bg(Color::White);
        }

        let row = Row::new(vec![format!("[ ] {}", task.content)]).style(task_style);
        task_rows.push(row);
    }
    let tasks_table = Table::new(task_rows, [Constraint::Fill(1)])
        .block(Block::new().borders(Borders::LEFT | Borders::BOTTOM));
    frame.render_widget(
        tasks_table,
        *view.get(1).expect("second element of view should exist"),
    );

    let sidebar = List::new(Vec::<ListItem>::new())
        .block(Block::default().borders(Borders::RIGHT | Borders::BOTTOM));
    frame.render_widget(
        sidebar,
        *view.first().expect("first element of view should exist"),
    );

    // Keybind hints
    let footer = *chunks
        .get(1)
        .expect("second element of chunks should exist");
    let hints = Paragraph::new(Line::from(vec![
        Span::from("<j/k> ").blue(),
        Span::from("move "),
    ]));
    frame.render_widget(hints, footer);
}
