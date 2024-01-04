#[derive(Debug, Default)]
pub struct App {
    pub counter: usize,
    pub should_quit: bool,
}

impl App {
    /// New instance of [`App`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles tick event
    pub fn tick(&self) {}

    /// Quit app
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Increment counter
    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    /// Decrement counter
    pub fn decrement_counter(&mut self) {
        // Min value is 0
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_app_increment_counter() {
        let mut app = App::default();
        app.increment_counter();
        assert_eq!(app.counter, 1);
    }

    #[test]
    fn test_app_decrement_counter() {
        let mut app = App::default();
        app.decrement_counter();
        assert_eq!(app.counter, 0);
    }

    #[test]
    fn test_app_counter_value() {
        let mut app = App {
            counter: 10,
            ..Default::default()
        };

        app.decrement_counter();
        assert_eq!(app.counter, 9);
    }
}
