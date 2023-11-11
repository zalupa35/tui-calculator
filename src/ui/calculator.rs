use super::highlight::highlight_expr;
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
            Constraint::Percentage(45),
            Constraint::Percentage(15),
            Constraint::Length(9),
            Constraint::Percentage(70),
        ])
        .split(layout[1]);

    // Render calculator input
    let mut line_vec = highlight_expr(app.calculator_input.clone());
    line_vec.push(Span::styled(" ", Style::default().bg(Color::White)));
    frame.render_widget(
        Paragraph::new(Line::from(line_vec))
            .block(Block::default().borders(Borders::ALL).title("Input")),
        sub_layout[0],
    );

    // Create and render big text
    let big_line = Line::from(vec![if app.is_calculator_error {
        Span::styled("Error", Style::default().fg(Color::Red))
    } else {
        Span::styled(
            format!("= {}", app.calculator_result),
            Style::default().fg(Color::Green),
        )
    }]);
    let big_text = BigTextBuilder::default()
        .lines(vec![big_line])
        .build()
        .expect("cant build big text");

    frame.render_widget(big_text, sub_layout[1]);

    // Render result
    let result_lines = vec![
        if app.is_calculator_error {
            Line::from(Span::styled("Error:", Style::default().fg(Color::Red)))
        } else {
            Line::from(vec![
                Span::raw("Result: "),
                Span::styled(
                    app.calculator_result.to_string(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ])
        },
        Line::raw(app.calculator_error.as_str()),
    ];
    frame.render_widget(
        Paragraph::new(Text::from(result_lines))
            .block(Block::default().borders(Borders::ALL).title("Result")),
        sub_layout[2],
    );
    frame.render_widget(
        Paragraph::new(Text::from(vec![
            Line::from(Span::styled(
                r#"Operations:"#,
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::raw("• Add: 2 + 2")),
            Line::from(Span::raw("• Subtract: 2 - 2")),
            Line::from(Span::raw("• Multiply: 2 * 2")),
            Line::from(Span::raw("• Divide: 2 / 2")),
            Line::from(Span::raw("• Power: 2 ^ 2 or 2 ** 2")),
            Line::from(Span::raw("• Modulo: 2 % 2")),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Help")),
        sub_layout[3],
    );

    draw_calculator_history(app, frame, sub_layout);
}

/// Draw calculator history
fn draw_calculator_history(app: &mut App, frame: &mut Frame, sub_layout: LayoutType) {
    let history_area = sub_layout[4];
    let vertical_scroll = app.history_scroll;
    let history_items = &app.calculator_history;

    let history_lines: Vec<Line<'_>> = app
        .calculator_history
        .iter()
        .map(|e| {
            Line::from(Span::styled(
                e.to_string(),
                Style::default().fg(Color::LightBlue),
            ))
        })
        .collect();

    let paragraph = Paragraph::new(Text::from(history_lines))
        .scroll((vertical_scroll as u16, 0))
        .block(Block::new().borders(Borders::ALL).title(Line::from(vec![
            Span::raw("History "),
            Span::styled(
                "(Press ▲ K and ▼ L to scroll, DEL to clear)",
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ])));

    let scrollbar = Scrollbar::default().orientation(ScrollbarOrientation::VerticalRight);
    let mut scrollbar_state =
        ScrollbarState::new(history_items.iter().len()).position(vertical_scroll);

    app.history_scrollbar_state = scrollbar_state;

    frame.render_widget(paragraph, history_area);
    frame.render_stateful_widget(
        scrollbar,
        history_area.inner(&Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut scrollbar_state,
    );
}
