use super::services::Services;
use crate::client::Client;
use std::time::Instant;

#[derive(Copy, Clone)]
pub struct Service {
    pub awt: u32, // Average wait time in seconds
    pub cost: u32,
    pub hourly_demand: u32,
    pub profit: i32,
    pub offer: u32,
    pub sid: Services,
    usage: u32,
}

impl Service {
    pub fn new(
        awt: u32,
        cost: u32,
        hourly_demand: u32,
        profit: i32,
        offer: u32,
        sid: Services,
    ) -> Service {
        Service {
            awt,
            cost,
            hourly_demand,
            profit,
            offer,
            sid,
            usage: 0,
        }
    }

    pub fn work(&mut self, client: &mut Client, queue: &mut Vec<&mut Client>) -> &Service {
        self.usage += 1;
        let mut first_go = true;

        client.start_time = Some(Instant::now());

        while self.usage == self.offer {
            if first_go {
                queue.append(client);
                queue.sort_by_key(|c| c.priority);
                first_go = false;
            }
        }

        let mut client = queue[0];
        client.elapsed_time = Some(client.start_time.unwrap().elapsed());
        queue.remove(0);
        self.usage -= 1;

        self
    }
}
