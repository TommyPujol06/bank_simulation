pub struct Service {
    pub awt: f64, // Average wait time in seconds
    pub cost: f64,
    pub hourly_demand: u64,
    pub profit: f64,
    pub offer: u64,
}

impl Service {
    pub fn new(awt: f64, cost: f64, hourly_demand: u64, profit: f64, offer: u64) -> Service {
        Service {
            awt,
            cost,
            hourly_demand,
            profit,
            offer,
        }
    }
}
