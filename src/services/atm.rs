use crate::services::base::Service;

pub struct ATM {
    pub base: Service,
}

impl ATM {
    pub fn new(awt: u32, cost: u32, hourly_demand: u32, profit: i32, offer: u32) -> ATM {
        ATM {
            base: Service::new(awt, cost, hourly_demand, profit, offer),
        }
    }
}
