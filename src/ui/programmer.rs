use super::LayoutType;
use crate::types::App;
use ratatui::Frame;
use ratatui::{prelude::*, widgets::*};
use tui_big_text::BigTextBuilder;

pub fn draw(app: &mut App, frame: &mut Frame, layout: LayoutType) {
    let sub_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Percentage(25),
            Constraint::Length(6),
            Constraint::Percentage(20),
        ])
        .split(layout[1]);

    // Render number input
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::raw(app.programmer_input.as_str()),
            Span::styled(" ", Style::default().bg(Color::White)),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Input")),
        sub_layout[0],
    );

    // Big number
    let big_line = Line::from(if !app.programmer_input_valid {
        vec![Span::styled("???", Style::default().fg(Color::Yellow))]
    } else {
        vec![
            Span::styled(
                &app.programmer_clear_input,
                Style::default().fg(Color::LightCyan),
            ),
            Span::styled(" - ", Style::default().fg(Color::LightCyan)),
            Span::styled(
                app.programmer_input_type.as_str(),
                Style::default().fg(Color::Cyan),
            ),
        ]
    });
    let big_text = BigTextBuilder::default()
        .lines(vec![big_line])
        .build()
        .expect("cant build big text");

    frame.render_widget(big_text, sub_layout[1]);

    let num = app.programmer_number;

    // Result
    frame.render_widget(
        Paragraph::new(Text::from(vec![
            Line::from(vec![
                Span::styled("HEX: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(format!("{:X}", num), Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled("DEC: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(num.to_string(), Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled("OCT: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(format!("{:o}", num), Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled("BIN: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(format!("{:b}", num), Style::default().fg(Color::Cyan)),
            ]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Result")),
        sub_layout[2],
    );

    // Help message
    frame.render_widget(
    Paragraph::new(Text::from(vec![
            Line::from(Span::styled(r#"Add "TYPE/type:number" to the beginning to determine the type of number (without this, it is determined automatically); Examples:"#, Style::default().add_modifier(Modifier::BOLD))),
            Line::from(Span::raw("• HEX:ABCDEF")),
            Line::from(Span::raw("• DEC:10")),
            Line::from(Span::raw("• OCT:7")),
            Line::from(Span::raw("• BIN:1010")),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Help")),
        sub_layout[3],
    );
}
