use crate::services::base::Service;
use std::{fmt, time::Instant};

pub struct Client {
    pub uid: String,
    pub priority: u8,
    pub start_time: Option<Instant>,
    pub service: Service,
}

impl Client {
    pub fn work(&mut self) {
        self.start_time = Some(Instant::now());
        self.service.start();
    }
}

impl fmt::Display for Client {
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
