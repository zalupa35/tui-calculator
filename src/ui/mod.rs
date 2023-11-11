use crate::types::{App, LayoutType};
use ratatui::prelude::*;

mod calculator;
mod highlight;
mod modes;
mod programmer;

/// Draw UI
pub fn draw(app: &mut App, frame: &mut Frame) {
    modes::draw(app, frame);
}
