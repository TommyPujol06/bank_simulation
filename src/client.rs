use std::{fmt, time::Instant};

pub struct Client {
    pub uid: String,
    pub priority: u8,
    pub start_time: Option<Instant>,
}

impl Client {
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }
}

impl fmt::Display for Client
where
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        let time = self.start_time.unwrap_or(Instant::now());
        write!(f, "{} [{}] (Elapsed: {:?})", self.uid, self.priority, time.elapsed())
    }
}
