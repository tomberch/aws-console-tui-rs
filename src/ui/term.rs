use std::panic;

use std::{
    io::{self, stdout, Stdout},
    ops::{Deref, DerefMut},
};

use anyhow::{Context, Result};
use crossterm::event::DisableMouseCapture;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;

#[derive(Debug)]
pub struct Term {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Term {
    pub fn start() -> Result<Self> {
        let backend = CrosstermBackend::new(std::io::stdout());
        let terminal = Terminal::new(backend)?;

        enable_raw_mode().context("Tried to enable raw mode")?;
        stdout()
            .execute(EnterAlternateScreen)
            .context("Tried to enter alternate screen")?;

        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::stop().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        Ok(Self { terminal })
    }

    pub fn stop() -> Result<()> {
        disable_raw_mode().context("Tried to disable raw mode")?;
        crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)
            .context("Tried to leave alternate screen")?;

        Ok(())
    }
}

impl Deref for Term {
    type Target = Terminal<CrosstermBackend<Stdout>>;
    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Term {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        let _ = Term::stop();
    }
}
