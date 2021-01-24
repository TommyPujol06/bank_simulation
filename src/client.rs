use std::{fmt, time::Instant};

pub struct Client<T> {
    pub uid: String,
    pub priority: u8,
    pub start_time: Option<Instant>,
    pub service: T,
}

impl<T> Client<T> {
    pub fn start<C>(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn work<C>(&mut self) {
        self.service.start(&self);
    }
}

impl<T> fmt::Display for Client<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        let time = self.start_time.unwrap_or(Instant::now());
        write!(
            f,
            "{} [{}] (Elapsed: {:?})",
            self.uid,
            self.priority,
            time.elapsed()
        )
    }
}
