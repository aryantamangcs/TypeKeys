use color_eyre::eyre::Result;
use color_eyre::SectionExt;
use crossterm::event::{self, DisableMouseCapture, Event, KeyCode};
use crossterm::{event::EnableMouseCapture, execute};
use ratatui;
use std::io::stdout;
use std::time::Duration;

use crate::app::App;
use crate::config::Config;
use crate::state::State;

pub fn run_ui(config: &Config, state: &State) -> Result<bool> {
    let mut terminal = ratatui::init();
    execute!(stdout(), EnableMouseCapture)?;

    let app = App::new(&state, &config);

    loop {
        terminal.draw(|frame| {
            app.draw(frame);
        })?;

        if app.should_quit {
            break;
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if app.handle_events(key.code) {
                    break;
                }
            }
        }
    }

    ratatui::restore();
    execute!(stdout(), DisableMouseCapture)?;
    Ok(state.should_loop)
}
