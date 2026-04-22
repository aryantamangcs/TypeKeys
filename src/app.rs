use crossterm::event::KeyCode;
use ratatui::{widgets::Paragraph, Frame};

use crate::{config::Config, state::State};

#[derive(Debug)]
pub struct App<'a> {
    state: &'a State,
    config: &'a Config,
    pub should_quit: bool,
}

impl<'a> App<'a> {
    pub fn new(state: &'a State, config: &'a Config) -> Self {
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

    pub fn handle_events(&self, code: KeyCode) -> bool {
        match code {
            KeyCode::Char('q') => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}
