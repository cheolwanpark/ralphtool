use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub fn render(frame: &mut Frame, _app: &App) {
    let area = frame.area();

    let block = Block::default()
        .title(" ralphtool ")
        .borders(Borders::ALL);

    let text = vec![
        Line::from("Welcome to ralphtool!"),
        Line::from(""),
        Line::from("Press q to quit"),
    ];

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}
