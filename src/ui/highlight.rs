use ratatui::{
    style::{Color, Style},
    text::Span,
};

const OPS: [char; 6] = ['+', '-', '*', '/', '%', '^'];

pub fn highlight_expr<'static_>(expr: String) -> Vec<Span<'static_>> {
    let mut res = Vec::new();
    for c in expr.chars() {
        let col: Option<Color> = if c.is_ascii_digit() {
            Some(Color::LightBlue)
        } else if OPS.contains(&c) {
            Some(Color::LightGreen)
        } else if c == '(' || c == ')' {
            Some(Color::LightYellow)
        } else {
            None
        };
        if let Some(a) = col {
            res.push(Span::styled(c.to_string(), Style::default().fg(a)))
        } else {
            res.push(Span::raw(c.to_string()));
        }
    }
    res
}
