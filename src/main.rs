extern crate easyrand;

mod client;
mod services;

use easyrand::randint;
use uuid::Uuid;
use client::Client;

const SIMULATION_SIZE: usize = 10000000;


fn main() {
    let mut v: Vec<Client> = vec![];
    for _ in 0..SIMULATION_SIZE {
        v.push(Client {
            uid: Uuid::new_v4().to_string(),
            priority: randint(1, 10) as u8,
            start_time: None
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
