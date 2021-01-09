extern crate easyrand;

mod client;

use easyrand::randint;
use uuid::Uuid;
use client::Client;

const SIMULATION_SIZE: usize = 10000000;

fn gen_priority(max_priority: u8) -> u8 {
    randint(1, max_priority as i64) as u8    
}

fn main() {
    let mut v: Vec<Client> = vec![];
    for _ in 0..SIMULATION_SIZE {
        v.push(Client {
            uid: Uuid::new_v4().to_string(),
            priority: gen_priority(10),
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
