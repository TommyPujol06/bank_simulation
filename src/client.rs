use crate::services::base::Service;
use crate::walker_alias::AliasTable;
use rand::thread_rng;
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

    pub fn random_priority(ptable: &AliasTable) -> u8 {
        let rng = thread_rng();
        ptable.random(rng) as u8
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        let time = self.start_time.unwrap_or(Instant::now());
        write!(
            f,
            "{} [{}] [{:?}] (Elapsed: {:?})",
            self.uid,
            self.priority,
            self.service.sid,
            time.elapsed()
        )
    }
}
