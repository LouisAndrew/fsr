use std::{
    io::{self, Stderr},
    panic,
};

use color_eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

pub type Term = Terminal<CrosstermBackend<Stderr>>;

use crate::{app::App, event::EventHandler, ui};

pub struct Tui {
    terminal: Term,
    pub handler: EventHandler,
}

impl Tui {
    fn reset() -> Result<()> {
        disable_raw_mode()?;
        execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;

        Ok(())
    }

    pub fn new(terminal: Term, handler: EventHandler) -> Self {
        Self { terminal, handler }
    }

    pub fn setup(&mut self, app: &App) -> Result<()> {
        enable_raw_mode()?;
        execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        let panic_hook = panic::take_hook();

        // Reset term on panic.
        panic::set_hook(Box::new(move |panic| {
            // not using `self.reset()` -> cannot share `receivers` between threads
            Self::reset().expect("failed to reset terminal");
            panic_hook(panic) // @TODO why re-using taken `panic_hook` here?
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;

        self.draw(app)?;
        Ok(())
    }

    pub fn teardown(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;

        Ok(())
    }

    pub fn draw(&mut self, app: &App) -> Result<()> {
        self.terminal.draw(|f| ui::render(app, f))?;

        Ok(())
    }
}
