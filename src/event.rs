use std::{
    sync::mpsc::{self},
    thread,
    time::{Duration, Instant},
    u64,
};

use color_eyre::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};

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
}

#[derive(Debug)]
pub struct EventHandler {
    /// Event sender
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>,
    /// Event receiver
    receiver: mpsc::Receiver<Event>,
    /// Event handler
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

/// Abstracts away event polling and handling into a dedicated background thread.
impl EventHandler {
    fn create_handler(sender: mpsc::Sender<Event>, tick_rate: Duration) -> thread::JoinHandle<()> {
        // @TODO What does `move` do here? without it, still compiles just fine.
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(tick_rate);

                if event::poll(timeout).expect("unable to poll for event") {
                    match event::read().expect("unable to read event") {
                        CrosstermEvent::Key(e) => {
                            if e.kind == event::KeyEventKind::Press {
                                // @TODO Returns `<Result<(), SendError<Event>>>`, why is it ok here,
                                // without `?` or `unwrap`?
                                sender.send(Event::Key(e))
                            } else {
                                // @TODO Question: why early return doesn't work here?
                                Ok(())
                            }
                        }
                        CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                        CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                        _ => unimplemented!(),
                    }
                    .expect("failed to send terminal event")
                }

                if last_tick.elapsed() >= tick_rate {
                    sender.send(Event::Tick).expect("Failed to send tick event");
                    last_tick = Instant::now();
                }
            }
        })
    }

    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);

        // Multi producer, single consumer
        let (sender, receiver) = mpsc::channel();

        // @TODO Question: If we don't clone the sender, the compiler won't complain. Is it ok?
        // Answer: The [`Sender`] can be cloned to [`send`] to the same channel multiple times, but
        // only one [`Receiver`] is supported.
        let handler = Self::create_handler(sender.clone(), tick_rate);

        Self {
            sender,
            receiver,
            handler,
        }
    }

    /// Receive next event from handler thread.
    /// Calling this will block the (background?) thread until an event is received.
    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}
