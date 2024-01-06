pub mod action;
pub mod app;
pub mod event;
pub mod tui;
pub mod ui;

use action::update;
use app::App;
use color_eyre::Result;
use event::{Event, EventHandler};
use ratatui::backend::CrosstermBackend;
use tui::{Term, Tui};

// https://ratatui.rs/tutorials/counter-async-app/conclusion/
#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(std::io::stderr());
    let term = Term::new(backend)?;
    let handler = EventHandler::new(250);
    let mut tui = Tui::new(term, handler);

    tui.setup(&app)?;
    while !app.should_quit {
        let e = tui.handler.next().await?;
        if let Event::Key(key) = e {
            app.action_bus.handle_keypress(key)?;
        }

        while let Ok(action) = app.action_bus.next().await {
            update(&mut app, action);
            // Render can also be made async, [docs](https://ratatui.rs/tutorials/counter-async-app/full-async-events/)
            tui.draw(&app)?;
        }
    }

    tui.teardown()?;

    Ok(())
}
