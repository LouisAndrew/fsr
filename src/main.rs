pub mod app;
pub mod event;
pub mod tui;
pub mod ui;
pub mod update;

use app::App;
use color_eyre::Result;
use event::{Event, EventHandler};
use ratatui::backend::CrosstermBackend;
use tui::{Term, Tui};
use update::update;

fn main() -> Result<()> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(std::io::stderr());
    let term = Term::new(backend)?;
    let handler = EventHandler::new(250);
    let mut tui = Tui::new(term, handler);

    tui.setup()?;
    while !app.should_quit {
        tui.draw(&app)?;

        if let Event::Key(key) = tui.handler.next()? {
            update(&mut app, key);
        }
    }

    tui.teardown()?;

    Ok(())
}
