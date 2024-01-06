use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::Frame,
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListDirection, Paragraph},
};

use crate::app::App;

pub fn render(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(0)])
        .split(frame.size());

    frame.render_widget(
        Paragraph::new(format!(
            "
            Press `Ctrl-C` or `q` to stop running.\n\
            Press `j` and `k` to increment and decrement the counter respectively.\n\
            Counter: {}
            ",
            app.counter
        ))
        .block(
            Block::default()
                .title("Counter")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(ratatui::style::Color::LightYellow))
        .alignment(Alignment::Center),
        chunks[0],
    );

    let footer = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let list = List::new(
        app.action_queue
            .clone()
            .iter()
            .enumerate()
            .map(|(i, a)| format!("index: {}, action: {}", i, a)),
    )
    .block(Block::default().title("Queue").borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    .highlight_symbol(">>")
    .repeat_highlight_symbol(true)
    .direction(ListDirection::BottomToTop);

    let list_2 = List::new(app.app_log.clone())
        .block(Block::default().title("Queue").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::BottomToTop);

    frame.render_widget(list, footer[0]);
    frame.render_widget(list_2, footer[1]);
}
