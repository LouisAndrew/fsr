use std::{time::Duration, u64};

use color_eyre::{eyre::eyre, Result};
use crossterm::event::{KeyEvent, MouseEvent};
use futures::{FutureExt, StreamExt};

#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Terminal resize.
    Resize(u16, u16),
    /// Mouse event.
    Mouse(MouseEvent),
    /// Error event.
    Error,
    // Redner event.
    Render,
}

#[derive(Debug)]
pub struct EventHandler {
    /// Event sender
    #[allow(dead_code)]
    sender: tokio::sync::mpsc::UnboundedSender<Event>,
    /// Event receiver
    receiver: tokio::sync::mpsc::UnboundedReceiver<Event>,
    /// Event handler
    #[allow(dead_code)]
    task: tokio::task::JoinHandle<()>,
}

/// Abstracts away event polling and handling into a dedicated background thread.
impl EventHandler {
    fn create_task(
        sender: tokio::sync::mpsc::UnboundedSender<Event>,
        tick_rate: Duration,
    ) -> tokio::task::JoinHandle<()> {
        // @TODO What does `move` do here? without it, still compiles just fine.
        // Async block takes ownership of all variables within
        tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut interval = tokio::time::interval(tick_rate);

            loop {
                let delay = interval.tick();
                // Prevent future blocking thread when additonal polls are called after future is done being handled.
                let crossterm_event = reader.next().fuse();
                // Treesitter and autoformat is not good here..
                tokio::select! {
                    // same as `let maybe_event = crossterm_event.await;`, but borrowed
                    Some(e) = crossterm_event => {
                        match e {
                            Ok(evt) => {
                                if let crossterm::event::Event::Key(key) = evt {
                                    if key.kind == crossterm::event::KeyEventKind::Press {
                                        sender.send(Event::Key(key)).unwrap();
                                    }
                                }
                            },
                            Err(_) => {
                                sender.send(Event::Error).unwrap();
                            }
                        }

                    },
                    _ = delay => {
                        sender.send(Event::Tick).unwrap();
                    }
                }
            }
        })
    }

    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);

        // Multi producer, single consumer
        // Tutorial asks `receiver` to be mutable, but not having it as `mut` is fine, no compile
        // error?!
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

        // @TODO Question: If we don't clone the sender, the compiler won't complain. Is it ok?
        // Answer: The [`Sender`] can be cloned to [`send`] to the same channel multiple times, but
        // only one [`Receiver`] is supported.
        let task = Self::create_task(sender.clone(), tick_rate);

        Self {
            sender,
            receiver,
            task,
        }
    }

    /// Receive next event from handler thread.
    /// Calling this will block the (background?) thread until an event is received.
    pub async fn next(&mut self) -> Result<Event> {
        self.receiver
            .recv()
            .await
            .ok_or(eyre!("Unable to receive event"))
    }
}
