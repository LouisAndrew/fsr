use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tokio::sync::mpsc::error::{SendError, TryRecvError};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::time::sleep;

use crate::app::App;

pub enum Action {
    Increment,
    Decrement,
    LazyIncrement,
    LazyDecrement,
    Quit,
}

pub struct ActionEvent(Action, Option<usize>);
static COUNTER: AtomicUsize = AtomicUsize::new(1);
fn action_id() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
pub struct ActionBus {
    rx: UnboundedReceiver<ActionEvent>,
    tx: UnboundedSender<ActionEvent>,
}

impl ActionBus {
    pub fn new() -> Self {
        let (tx, rx) = unbounded_channel();
        Self { rx, tx }
    }

    fn send(&self, action: Action) -> Result<(), SendError<ActionEvent>> {
        self.tx.send(ActionEvent(action, None))
    }

    pub fn handle_keypress(&self, key: KeyEvent) -> Result<(), SendError<ActionEvent>> {
        match key.code {
            KeyCode::Char('q') => self.send(Action::Quit),
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key.modifiers == KeyModifiers::CONTROL {
                    self.send(Action::Quit)
                } else {
                    Ok(())
                }
            }
            KeyCode::Char('j') => self.send(Action::Increment),
            KeyCode::Char('J') => self.send(Action::LazyIncrement),
            KeyCode::Char('k') => self.send(Action::Decrement),
            KeyCode::Char('K') => self.send(Action::LazyDecrement),
            _ => Ok(()),
        }
    }

    pub async fn next(&mut self) -> Result<ActionEvent, TryRecvError> {
        self.rx.try_recv()
    }
}

impl Default for ActionBus {
    fn default() -> Self {
        Self::new()
    }
}

pub fn update(app: &mut App, action_event: ActionEvent) {
    let ActionEvent(action, idx) = action_event;

    match action {
        Action::Increment => app.increment_counter(),
        Action::Decrement => app.decrement_counter(),
        Action::LazyIncrement => {
            let tx = app.action_bus.tx.clone();
            let idx = action_id();
            app.action_queue.push(format!("{}: lazy increment", idx));

            tokio::spawn(async move {
                sleep(Duration::from_secs(5)).await; // simulate network request
                tx.send(ActionEvent(Action::Increment, Some(idx))).unwrap();
            });
        }
        Action::LazyDecrement => {
            let tx = app.action_bus.tx.clone();
            let idx = action_id();
            app.action_queue.push(format!("{}: lazy decrement", idx));

            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(5)).await; // simulate network request
                tx.send(ActionEvent(Action::Decrement, Some(idx))).unwrap();
            });
        }
        Action::Quit => app.should_quit = true,
    };

    if let Some(idx) = idx {
        app.action_queue
            // Not sure why, but the ampersand before `format` is strictly required
            .retain(|x| x.starts_with(&format!("{}: ", idx)));
        app.app_log.push(format!("Action {} completed", idx));
    }
}
