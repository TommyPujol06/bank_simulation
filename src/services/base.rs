use super::services::Services;
use crate::client::Client;

#[derive(Copy, Clone)]
pub struct Service {
    pub awt: u32, // Average wait time in seconds
    pub cost: u32,
    pub hourly_demand: u32,
    pub profit: i32,
    pub offer: u32,
    pub sid: Services,
    usage: u32,
    //    queue: &[u32],
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
            // queue: &[0];
        }
    }

    pub fn work(&mut self, _client: &Client) -> &Service {
        self.usage += 1;
        let mut first_go = true;

        while self.usage == self.offer {
            if first_go {
                // self.queue.append(client);
                // self.queue.sort_by_key(|c| c.priority);
                first_go = false;
            }
        }

        // before cuda init
        // client.start_time = Some(Instant::now());

        self
    }
}
