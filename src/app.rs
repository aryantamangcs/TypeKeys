use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{text::ToSpan, widgets::Paragraph, Frame};

use crate::{config::Config, state::State};

#[derive(Debug)]
pub struct App<'a> {
    state: &'a mut State,
    config: &'a Config,
    pub should_quit: bool,
}

impl<'a> App<'a> {
    pub fn new(state: &'a mut State, config: &'a Config) -> Self {
        Self {
            should_quit: false,
            state: state,
            config: config,
        }
    }

    pub fn draw(&self, frame: &mut Frame) {
        let widget = Paragraph::new(self.state.target.as_str());
        frame.render_widget(widget, frame.area());
    }

    pub fn handle_events(&mut self, code: KeyCode, modifiers: KeyModifiers) -> bool {
        if modifiers.contains(KeyModifiers::CONTROL) {
            match code {
                KeyCode::Char('c') => {
                    self.state.should_loop = false;
                    self.should_quit = false;
                    return true;
                }
                KeyCode::Char('w') => {
                    return self.state.handle_backspace();
                }
                _ => {
                    return false;
                }
            }
        }
        match code {
            KeyCode::Char(c) => return self.state.handle_char(c),
            KeyCode::Enter => return self.state.handle_char('\n'),
            KeyCode::Backspace => return self.state.handle_backspace(),
            KeyCode::Esc => {
                self.state.should_loop = false;
                self.should_quit = false;
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}
