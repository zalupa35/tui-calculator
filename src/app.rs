use crate::calculator;
use crate::{init_terminal, restore_terminal, types::App, ui};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::{io, time::Duration};

/// Remove last character from String
fn remove_last_char(str: String) -> String {
    let mut chars = str.chars();
    chars.next_back();
    return chars.as_str().to_string();
}

impl App {
    fn new() -> App {
        App {
            modes: vec!["Calculator".to_string(), "Programmer".to_string()],
            ..App::default()
        }
    }

    /// Evaluate calculator input
    pub fn eval_calculator_input(&mut self) {
        let input = &self.calculator_input;
        let result = calculator::calculate(input.to_string());
        match result {
            Ok(n) => {
                self.is_calculator_error = false;
                self.calculator_result = n;
                self.calculator_error = String::new();
                let history = &self.calculator_history;
                if history.is_empty() || history[history.len() - 1] != n {
                    self.calculator_history.push(n);
                }
            }
            Err(e) => {
                self.is_calculator_error = true;
                self.calculator_error = e;
            }
        };
    }

    /// Convert number to other number systems (in programmer mode)
    pub fn eval_programmer_input(&mut self) {
        let input = &self.programmer_input;
        let split: Vec<&str> = input.split(':').collect();

        let mut preffered_type: Option<&str> = None;
        let mut clear_input = input.clone();

        if split.len() != 1 && split[1].trim() != "" {
            preffered_type = Some(split[0]);
            clear_input = split[1].to_string();
        }

        self.programmer_clear_input = clear_input.clone();

        let parsed_bin = i64::from_str_radix(&clear_input, 2);
        let parsed_octal = i64::from_str_radix(&clear_input, 8);
        let parsed_int = clear_input.parse::<i64>();
        let parsed_hex = i64::from_str_radix(&clear_input, 16);

        if let Some(raw_type) = preffered_type {
            let clear_type = raw_type.to_uppercase();
            let mut valid = true;
            let res = match clear_type.as_str() {
                "BIN" => parsed_bin,
                "HEX" => parsed_hex,
                "OCT" => parsed_octal,
                "DEC" => parsed_int,
                _ => {
                    valid = false;
                    Ok(0)
                }
            };

            self.programmer_input_valid = valid && res.is_ok();
            if valid {
                if let Ok(value) = res {
                    self.programmer_input_type = clear_type;
                    self.programmer_number = value;
                } else {
                    self.programmer_number = 0;
                }
            } else {
                self.programmer_number = 0;
            }
        } else {
            self.programmer_input_valid = true;
            if let Ok(value) = parsed_bin {
                self.programmer_input_type = "BIN".to_string();
                self.programmer_number = value;
            } else if let Ok(value) = parsed_octal {
                self.programmer_input_type = "OCT".to_string();
                self.programmer_number = value;
            } else if let Ok(value) = parsed_int {
                self.programmer_input_type = "DEC".to_string();
                self.programmer_number = value;
            } else if let Ok(value) = parsed_hex {
                self.programmer_input_type = "HEX".to_string();
                self.programmer_number = value;
            } else {
                self.programmer_input_valid = false;
                self.programmer_number = 0;
            }
        }
    }

    /// On tick
    pub fn on_tick(&mut self) {
        self.eval_calculator_input();
        self.eval_programmer_input();
    }

    pub fn run() -> io::Result<()> {
        let mut terminal = init_terminal()?;
        let mut app = App::new();

        loop {
            let _ = terminal.draw(|frame| ui::draw(&mut app, frame));
            if event::poll(Duration::from_millis(250))? {
                App::on_tick(&mut app);
                if let Event::Key(key) = event::read()? {
                    let current_mode_val = &app.modes[app.current_mode];
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            // Quit on 'q'
                            KeyCode::Char('q') => break,
                            // Move up in modes vector
                            KeyCode::Up => {
                                if app.current_mode == 0 {
                                    app.current_mode = app.modes.len() - 1;
                                } else {
                                    app.current_mode -= 1;
                                }
                            }
                            // Move down in modes vector
                            KeyCode::Down => {
                                if app.current_mode == app.modes.len() - 1 {
                                    app.current_mode = 0;
                                } else {
                                    app.current_mode += 1;
                                }
                            }
                            // History scrollbar
                            // Scroll up
                            KeyCode::Char('k') => {
                                if current_mode_val == "Calculator" {
                                    app.history_scroll = app.history_scroll.saturating_sub(5);
                                    app.history_scrollbar_state =
                                        app.history_scrollbar_state.position(app.history_scroll);
                                }
                            }
                            // Scroll down
                            KeyCode::Char('l') => {
                                if current_mode_val == "Calculator" {
                                    app.history_scroll = app.history_scroll.saturating_add(5);
                                    app.history_scrollbar_state =
                                        app.history_scrollbar_state.position(app.history_scroll);
                                }
                            }
                            // Clear history
                            KeyCode::Delete => {
                                if current_mode_val == "Calculator" {
                                    app.calculator_history.clear();
                                }
                            }
                            // Add character to all inputs
                            KeyCode::Char(c) => {
                                if current_mode_val == "Calculator" {
                                    app.calculator_input.push(c);
                                } else if current_mode_val == "Programmer" {
                                    app.programmer_input.push(c);
                                }
                            }
                            // Remove last character in all inputs on 'Backspace'
                            KeyCode::Backspace => {
                                if current_mode_val == "Calculator" {
                                    app.calculator_input = remove_last_char(app.calculator_input);
                                } else if current_mode_val == "Programmer" {
                                    app.programmer_input = remove_last_char(app.programmer_input);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        restore_terminal()
    }
}
