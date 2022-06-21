use std::{
    cell::RefCell,
    time::{Duration, Instant},
};

pub struct Timer {
    label: String,
    start: Instant,
    is_silenced: RefCell<bool>,
}

impl Timer {
    pub fn start_new<S>(label: S) -> Timer
    where
        S: ToString,
    {
        Timer {
            label: label.to_string(),
            start: Instant::now(),
            is_silenced: RefCell::new(false),
        }
    }

    pub fn lap(&self, file: &'static str, line: u32) {
        if *self.is_silenced.borrow() {
            return;
        }
        println!(
            "[{} {}${}] took {:?}",
            self.label,
            file,
            line,
            self.elapsed()
        );
    }

    pub fn silence(&self) {
        *self.is_silenced.borrow_mut() = true;
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn report(&self) {
        if *self.is_silenced.borrow() {
            return;
        }
        println!("[{}] took {:?}", self.label, self.elapsed());
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.report();
    }
}