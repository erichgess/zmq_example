//! Hello World server in Rust
//! Binds REP socket to tcp://*:5555
//! Expects "Hello" from client, replies with "World"

mod data;
mod msg;

use clap::{App, Arg};
use data::Data;

use std::thread;
use std::time::Duration;

fn main() {
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
            let client = thread::spawn(move || client(client_port));
            threads.push(client);
        }
        None => (),
    }

    for t in threads {
        t.join().unwrap();
    }
}

fn server(port: u32) {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();
    let addr = format!("tcp://*:{}", port);
    assert!(responder.bind(&addr).is_ok());

    let mut msg = zmq::Message::new();
    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("Received Message of Length {}", msg.len());
        let req: msg::Request = rmp_serde::decode::from_slice(&msg).unwrap();
        println!("Recieved Contents: {:?}", req);

        thread::sleep(Duration::from_millis(1000));
        let response = format!("Server Got {}", msg.len());
        responder.send(&response, 0).unwrap();
    }
}

fn client(port: u32) {
    println!("Connecting to hello world server...\n");

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();

    let addr = format!("tcp://localhost:{}", port);
    assert!(requester.connect(&addr).is_ok());

    let mut response = zmq::Message::new();

    for request_nbr in 0..10 {
        println!("Sending Message {}...", request_nbr);
        let data = Data::new(&vec![1., 2., 3.]);
        let msg = msg::Request::new(&data);
        let mpk = rmp_serde::encode::to_vec(&msg).unwrap();

        match requester.send(mpk, 0) {
            Ok(_) => (),
            Err(msg) => {
                println!("{}", msg);
                panic!("{}", msg);
            }
        }

        match requester.recv(&mut response, 0) {
            Ok(_) => (),
            Err(msg) => {
                println!("{}", msg);
                panic!("{}", msg);
            }
        }
        println!("Received '{}': {}", response.as_str().unwrap(), request_nbr);
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
