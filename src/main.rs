mod client;
mod services;
mod walker_alias;

use client::Client;
use services::base::Service;
use services::services::Services;
use std::mem::MaybeUninit;
use uuid::Uuid;
use walker_alias::AliasTable;

const SIMULATION_SIZE: usize = 10000000;
const SERVICES_SIZE: usize = 7;
const CLIENT_PRIORITY_RANGE: usize = 3; // Starts from 0 and goes up to CLIENT_PRIORITY_RANGE - 1

fn main() {
    let services = vec![
        Service::new(5 * 60, 10000000, 6000, 39000000, 15000, Services::POS),
        Service::new(5 * 60, 10000000, 6000, 39000000, 15000, Services::ATM),
        Service::new(5 * 60, 10000000, 6000, 39000000, 15000, Services::WEB),
        Service::new(5 * 60, 10000000, 6000, 39000000, 15000, Services::APP),
        Service::new(5 * 60, 10000000, 6000, 39000000, 15000, Services::BOT),
        Service::new(5 * 60, 10000000, 6000, 39000000, 15000, Services::HHD),
        Service::new(5 * 60, 10000000, 6000, 39000000, 15000, Services::OFF),
    ];

    let client_prioritiy_probabilities = vec![
        80.0, // 0 -> 80%
        15.0, // 1 -> 15%
        05.0, // 2 -> 5%
    ];

    let mut sweights: MaybeUninit<[f64; SERVICES_SIZE]> = MaybeUninit::uninit();
    let mut cweights: MaybeUninit<[f64; CLIENT_PRIORITY_RANGE]> = MaybeUninit::uninit();
    unsafe {
        for (i, service) in services.iter().enumerate() {
            (sweights.as_mut_ptr() as *mut f64)
                .offset(i as isize)
                .write(service.sid.get_probability());
        }

        for (i, prob) in client_prioritiy_probabilities.iter().enumerate() {
            (cweights.as_mut_ptr() as *mut f64)
                .offset(i as isize)
                .write(*prob);
        }
    }

    let stable = unsafe { AliasTable::new(&sweights.assume_init()) };
    let ctable = unsafe { AliasTable::new(&cweights.assume_init()) };

    let mut v: Vec<Client> = vec![];
    for _ in 0..SIMULATION_SIZE {
        v.push(Client {
            uid: Uuid::new_v4().to_string(),
            priority: Client::random_priority(&ctable),
            start_time: None,
            elapsed_time: None,
            service: Services::random(&stable, &services),
        });
    }
    let mut queue: Vec<&mut Client> = vec![];
    for i in 0..SIMULATION_SIZE {
        let mut service = v[i].service;
        service.work(&mut v[i], &mut queue);
    }

    for i in 0..SIMULATION_SIZE {
        if i % 1000000 == 0 {
            println!("{}", v[i])
        }
    }
}
