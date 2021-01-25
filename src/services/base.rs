use super::services::Services;

#[derive(Copy, Clone)]
pub struct Service {
    pub awt: u32, // Average wait time in seconds
    pub cost: u32,
    pub hourly_demand: u32,
    pub profit: i32,
    pub offer: u32,
    pub sid: Services,
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
        }
    }

    pub fn start(&mut self) -> &Service {
        self
    }
}
