use color_eyre::{self, Result};

use crate::config::Config;
use crate::state::State;
use crate::text::get_text;
use crate::ui::run_ui;

mod app;
mod config;
mod state;
mod text;
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;

    let config = Config::new();

    loop {
        let text = get_text(&config).unwrap();
        let state = State::new(text);

        let should_loop = run_ui(&config, &state)?;
        if !should_loop {
            break;
        }
    }

    Ok(())
}
