extern crate easyrand;

mod client;
mod services;

use client::Client;
use easyrand::randint;
use services::atm::ATM;
use uuid::Uuid;

const SIMULATION_SIZE: usize = 10000000;

//
// ATM: Average wait time 5mins
// Yearly mantainance cost: 1M €
// 6k ATM transactions throughout the whole country per hour.
// With an average of 100€ withdrawals and a 1.8€ fee per withdrawal the gross yearly profit is ~39M€
// The ATM offer (how many ATM's are available) is 15k
//
const SERVICES: Vec<ATM> = vec![ATM::new(5 * 60, 10_000_000, 6000, 39_000_000, 15_000)];

fn main() {
    let mut v: Vec<_> = vec![];
    for _ in 0..SIMULATION_SIZE {
        v.push(Client {
            uid: Uuid::new_v4().to_string(),
            priority: randint(1, 10) as u8,
            start_time: None,
            service: SERVICES[0],
        });
    }

    v.sort_by_key(|c| c.priority);

    for i in 0..SIMULATION_SIZE {
        v[i].start();
    }

    for i in 0..SIMULATION_SIZE {
        if i % 1000000 == 0 {
            println!("{}", v[i])
        }
    }
}
