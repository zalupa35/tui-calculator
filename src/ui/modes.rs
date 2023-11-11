use ratatui::{
    prelude::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::{calculator, programmer};
use crate::types::App;

pub fn draw(app: &mut App, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(50)])
        .split(frame.size());

    // Create simple list
    let mut modes_lines = vec![];
    for (i, v) in app.modes.clone().iter().enumerate() {
        if i == app.current_mode {
            modes_lines.push(Line::from(Span::styled(
                v.to_string(),
                Style::default().bg(Color::White).fg(Color::Black),
            )));
        } else {
            modes_lines.push(Line::raw(v.to_string()));
        }
    }

    // Render list
    frame.render_widget(
        Paragraph::new(Text::from(modes_lines)).block(
            Block::default()
                .title(Line::from(vec![
                    Span::raw("Modes "),
                    Span::styled(
                        "(Press arrows ▲ ▼ to scroll)",
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                ]))
                .borders(Borders::ALL),
        ),
        layout[0],
    );

    match app.modes[app.current_mode].as_str() {
        "Calculator" => calculator::draw(app, frame, layout),
        "Programmer" => programmer::draw(app, frame, layout),
        _ => {}
    };
}
