use ratatui::{
    layout::Alignment,
    prelude::Frame,
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &App, frame: &mut Frame) {
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
        frame.size(),
    )
}
