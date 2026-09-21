use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
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

    let mut tasks_list_items = Vec::<ListItem>::new();
    for task in &app.tasks {
        tasks_list_items.push(ListItem::new(Line::from(Span::styled(
            format!("[ ] {}", task.content),
            Style::default().fg(Color::Yellow),
        ))));
    }
    let tasks_list =
        List::new(tasks_list_items).block(Block::new().borders(Borders::LEFT | Borders::BOTTOM));
    frame.render_widget(
        tasks_list,
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
        Span::from("<q> ").blue(),
        Span::from("quit"),
    ]));
    frame.render_widget(hints, footer);
}
