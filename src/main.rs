//! Hello World server in Rust
//! Binds REP socket to tcp://*:5555
//! Expects "Hello" from client, replies with "World"

use clap::{App, Arg};

use std::thread;
use std::time::Duration;

fn main() {
    let config = config_args();
    let client_port = config.client;
    let server_port = config.server;

    let server = thread::spawn(move || server(server_port));
    let client = thread::spawn(move || client(client_port));

    server.join().unwrap();
    client.join().unwrap();
}

fn server(port: u32) {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();
    let addr = format!("tcp://*:{}", port);
    assert!(responder.bind(&addr).is_ok());

    let mut msg = zmq::Message::new();
    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("Received {}", msg.as_str().unwrap());

        thread::sleep(Duration::from_millis(1000));
        responder.send("World", 0).unwrap();
    }
}

fn client(port: u32) {
    println!("Connecting to hello world server...\n");

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();

    let addr = format!("tcp://localhost:{}", port);
    assert!(requester.connect(&addr).is_ok());

    let mut msg = zmq::Message::new();

    for request_nbr in 0..10 {
        println!("Sending Hello {}...", request_nbr);
        requester.send("Hello", 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        println!("Received World {}: {}", msg.as_str().unwrap(), request_nbr);
    }
}

struct Config {
    server: u32,
    client: u32,
}

fn config_args() -> Config {
    let matches = App::new("Simple 0MQ Program")
        .arg(
            Arg::with_name("server-port")
                .required(true)
                .short("s")
                .long("server-port")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("client-port")
                .required(true)
                .short("c")
                .long("client-port")
                .takes_value(true),
        )
        .get_matches();

    let server_port: u32 = matches.value_of("server-port").unwrap().parse().unwrap();
    let client_port: u32 = matches.value_of("client-port").unwrap().parse().unwrap();

    Config {
        server: server_port,
        client: client_port,
    }
}
