//! Hello World server in Rust
//! Binds REP socket to tcp://*:5555
//! Expects "Hello" from client, replies with "World"

#![allow(dead_code)]

mod compute;
mod data;
mod msg;
mod server;

use std::thread;

use clap::{App, Arg};
use crossbeam::channel::unbounded;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

use server::*;

fn main() {
    // configure logger
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
    let config = config_args();

    let mut threads = vec![];

    let (i_s, i_r) = unbounded(); // i_s goes to the server and i_r goes to the worker
    let (o_s, o_r) = unbounded(); // o_s goes to the worker and o_r goes to the client

    let (i_r2, o_s2) = (i_r.clone(), o_s.clone());
    let worker = thread::spawn(move || {
        compute::computer(i_r2, o_s2);
    });
    threads.push(worker);

    if config.prime {
        info!("Priming the pump");
        let primer = data::Data::new(&vec![1., 2., 3.]);
        i_s.send(primer).unwrap();
    }

    match config.server {
        Some(server_port) => {
            let server = thread::spawn(move || server(server_port, i_s));
            threads.push(server);
        }
        None => (),
    }
    match config.client {
        Some(client_port) => {
            let client = thread::spawn(move || {
                client(client_port, o_r);
                println!("Client Done");
            });
            threads.push(client);
        }
        None => (),
    }

    for t in threads {
        t.join().unwrap();
    }
}

struct Config {
    server: Option<u32>,
    client: Option<u32>,
    prime: bool,
}

fn config_args() -> Config {
    let matches = App::new("Simple 0MQ Program")
        .arg(
            Arg::with_name("server-port")
                .short("s")
                .long("server-port")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("client-port")
                .short("c")
                .long("client-port")
                .takes_value(true),
        )
        .arg(Arg::with_name("prime").short("p").long("prime"))
        .get_matches();

    let server_port: Option<u32> = matches.value_of("server-port").map(|v| v.parse().unwrap());
    let client_port: Option<u32> = matches.value_of("client-port").map(|v| v.parse().unwrap());
    let prime = matches.is_present("prime");

    Config {
        server: server_port,
        client: client_port,
        prime,
    }
}
