//! Hello World server in Rust
//! Binds REP socket to tcp://*:5555
//! Expects "Hello" from client, replies with "World"

#![allow(dead_code)]

mod data;
mod msg;
mod server;

use std::thread;

use clap::{App, Arg};
use log::LevelFilter;
use simple_logger::SimpleLogger;

use server::*;

fn main() {
    // configure logger
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();
    let config = config_args();

    let mut threads = vec![];

    match config.server {
        Some(server_port) => {
            let server = thread::spawn(move || server(server_port));
            threads.push(server);
        }
        None => (),
    }
    match config.client {
        Some(client_port) => {
            let client = thread::spawn(move || {
                client(client_port);
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
        .get_matches();

    let server_port: Option<u32> = matches.value_of("server-port").map(|v| v.parse().unwrap());
    let client_port: Option<u32> = matches.value_of("client-port").map(|v| v.parse().unwrap());

    Config {
        server: server_port,
        client: client_port,
    }
}
