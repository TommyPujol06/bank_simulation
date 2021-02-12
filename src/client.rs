use crate::services::base::Service;
use crate::walker_alias::AliasTable;
use rand::thread_rng;
use std::{fmt, time::Duration, time::Instant};

pub struct Client {
    pub uid: String,
    pub priority: u8,
    pub start_time: Option<Instant>,
    pub elapsed_time: Option<Duration>,
    pub service: Service,
}

impl Client {
    pub fn random_priority(ptable: &AliasTable) -> u8 {
        let rng = thread_rng();
        ptable.random(rng) as u8
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} [{}] [{:?}] (Elapsed: {:?})",
            self.uid, self.priority, self.service.sid, self.elapsed_time
        )
    }
}
