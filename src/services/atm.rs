use crate::services::base::Service;

pub struct ATM {
    pub base: Service,
}

impl ATM {
    fn new(awt: f64, cost: f64, hourly_demand: u64, profit: f64, offer: u64) -> ATM {
        ATM {
            base: Service::new(awt, cost, hourly_demand, profit, offer),
        }
    }
}
