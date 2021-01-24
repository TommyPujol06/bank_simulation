use crate::client::Client;

pub struct Service {
    pub awt: u32, // Average wait time in seconds
    pub cost: u32,
    pub hourly_demand: u32,
    pub profit: i32,
    pub offer: u32,
}

impl Service {
    pub fn new(awt: u32, cost: u32, hourly_demand: u32, profit: i32, offer: u32) -> Service {
        Service {
            awt,
            cost,
            hourly_demand,
            profit,
            offer,
        }
    }

    pub fn start<T>(&self, client: &Client<T>) -> &Service {
        let _client = client;
        self
    }
}
