//! Hello World server in Rust
//! Binds REP socket to tcp://*:5555
//! Expects "Hello" from client, replies with "World"

use clap::{App, Arg};
use zmq::{self, PollEvents};

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

        thread::sleep(Duration::from_millis(1000));
        let response = format!("Server Got {}", msg.len());
        responder.send(&response, 0).unwrap();
    }
}

fn client(port: u32) {
    println!("Connecting to hello world server...\n");

    let context = zmq::Context::new();
    let mut requester = context.socket(zmq::REQ).unwrap();
    println!("New Socket: {:?}", requester.get_identity().unwrap());

    let addr = format!("tcp://localhost:{}", port);
    assert!(requester.connect(&addr).is_ok());

    let mut msg = zmq::Message::new();

    for request_nbr in 0..10 {
        println!("Sending Hello {}...", request_nbr);
        let data = vec![5; 5];

        while let Err(msg) = requester.send(&data, 0) {
            println!("Send Error: {}", msg);
            println!("Retrying...");
            thread::sleep(Duration::from_millis(1000));
        }

        println!("Waiting for server...");
        loop {
            match requester.poll(PollEvents::POLLIN, 5000) {
                Ok(i) => {
                    //
                    println!("Polling #: {}", i);
                    if i > 0 {
                        match requester.recv(&mut msg, 0) {
                            Ok(_) => {
                                println!("Received '{}': {}", msg.as_str().unwrap(), request_nbr);
                                break;
                            }
                            Err(msg) => {
                                panic!("Receive Error: {}", msg);
                            }
                        }
                    } else {
                        println!("No Event");
                        drop(requester);
                        requester = context.socket(zmq::REQ).unwrap();
                        assert!(requester.connect(&addr).is_ok());
                        println!("New Socket: {:?}", requester.get_identity().unwrap());
                        break;
                    }
                }
                Err(msg) => println!("Polling Error: {}", msg),
            }
        }
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
