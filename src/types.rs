use ratatui::{prelude::Rect, widgets::ScrollbarState};
use std::rc::Rc;

pub type LayoutType = Rc<[Rect]>;

#[derive(Default)]
pub struct App {
    // Basic calculator:
    /// Calculator modes (basic calculator and programmer)
    pub modes: Vec<String>,
    /// Current calculator mode (index in vector)
    pub current_mode: usize,
    /// Calculator history
    pub calculator_history: Vec<f64>,

    /// Calculator history scrollbar state
    pub history_scrollbar_state: ScrollbarState,
    /// Calculator history vertical scroll
    pub history_scroll: usize,

    /// Calculator input string
    pub calculator_input: String,
    /// Is error in input
    pub is_calculator_error: bool,
    /// Calculation result
    pub calculator_result: f64,
    /// Calculation error string
    pub calculator_error: String,

    // Programmer mode:
    /// Number input
    pub programmer_input: String,
    /// Converted number
    pub programmer_number: i64,
    /// Is number valid
    pub programmer_input_valid: bool,
    /// Number input type (HEX, DEC, OCT, BIN)
    pub programmer_input_type: String,
    /// Clear number (without [SYSTEM:]; => Number)
    pub programmer_clear_input: String,
}
