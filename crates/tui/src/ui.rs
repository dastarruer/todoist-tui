use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Row, Table},
};
use ratatui_textarea::TextArea;

use crate::app::{AddTaskTextAreas, App, ViewState};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(95), Constraint::Max(10)])
        .split(frame.area());
    let footer_area = *chunks
        .get(1)
        .expect("second element of chunks should exist");
    let view_area = *chunks
        .first()
        .expect("first element of chunks should exist");

    // Sidebar and tasks
    let view = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(30), Constraint::Percentage(100)])
        .split(view_area);
    let sidebar_area = *view.first().expect("second element of view should exist");
    let tasks_area = *view.get(1).expect("second element of view should exist");

    let mut task_rows = Vec::<Row>::new();
    let highlighted_task = app.highlighted_task();
    for task in &app.tasks {
        let bg_color = if let Some(highlighted_task) = highlighted_task
            && task == highlighted_task
        {
            Color::White
        } else {
            Color::Reset
        };
        let task_style = Style::default().fg(Color::Yellow).bg(bg_color);

        let row = Row::new(vec![format!("[ ] {}", task.content)]).style(task_style);
        task_rows.push(row);
    }
    let tasks_table = Table::new(task_rows, [Constraint::Fill(1)])
        .block(Block::new().borders(Borders::LEFT | Borders::BOTTOM));
    frame.render_widget(tasks_table, tasks_area);

    let sidebar = List::new(Vec::<ListItem>::new())
        .block(Block::default().borders(Borders::RIGHT | Borders::BOTTOM));
    frame.render_widget(sidebar, sidebar_area);

    // Keybind hints
    let hints = Paragraph::new(Line::from(vec![
        Span::from("<j/k> ").blue(),
        Span::from("move "),
        Span::from("<a> ").blue(),
        Span::from("add task "),
    ]));
    frame.render_widget(hints, footer_area);

    if let ViewState::AddTask(textareas) = &app.view {
        render_add_task_popup(frame, tasks_area, textareas);
    }
}

// https://ratatui.rs/recipes/layout/center-a-widget/#popups
fn render_add_task_popup(frame: &mut Frame, area: Rect, textareas: &AddTaskTextAreas) {
    let area = area.centered(Constraint::Max(50), Constraint::Max(10));
    let popup = Block::bordered();
    frame.render_widget(Clear, area);
    frame.render_widget(&popup, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(popup.inner(area));

    let mut content = textareas.content().to_owned();
    if textareas.is_focused(0) {
        activate(&mut content, "Task name:");
    } else {
        inactivate(&mut content, "Task name:");
    }
    frame.render_widget(
        &content,
        *chunks
            .first()
            .expect("first element of chunks should exist"),
    );

    let mut desc = textareas.desc().to_owned();
    if textareas.is_focused(1) {
        activate(&mut desc, "Description:");
    } else {
        inactivate(&mut desc, "Description");
    }
    frame.render_widget(
        &desc,
        *chunks
            .get(1)
            .expect("second element of chunks should exist"),
    );
}

fn inactivate<'a>(textarea: &mut TextArea<'a>, block_title: &'a str) {
    textarea.set_cursor_line_style(Style::default());
    textarea.set_cursor_style(Style::default());
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::DarkGray))
            .title(format!(" {block_title} (^X to switch) ")),
    );
}

fn activate<'a>(textarea: &mut TextArea<'a>, block_title: &'a str) {
    textarea.set_cursor_line_style(Style::default());
    textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(block_title),
    );
}
