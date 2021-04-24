//! Hello World server in Rust
//! Binds REP socket to tcp://*:5555
//! Expects "Hello" from client, replies with "World"

#![allow(dead_code)]

mod data;
mod msg;

use clap::{App, Arg};
use data::Data;
use msg::{Response, Status};
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
        let req: msg::Request = rmp_serde::decode::from_slice(&msg).unwrap();
        println!("From Client: {:?}", req);

        thread::sleep(Duration::from_millis(1000));

        let response = Response::new(Status::Good(req.id()));
        let mpk = rmp_serde::encode::to_vec(&response).unwrap();
        responder.send(&mpk, 0).unwrap();
    }
}

fn client(port: u32) {
    // Setup ZeroMQ
    let addr = format!("tcp://localhost:{}", port);
    println!("Connecting to {}...\n", addr);

    let context = zmq::Context::new();
    let mut requester = context.socket(zmq::REQ).unwrap();
    println!("New Socket: {:?}", requester.get_identity().unwrap());
    assert!(requester.connect(&addr).is_ok());

    // Initialize push loop
    for request_nbr in 0..10 {
        println!("Sending Message {}...", request_nbr);
        let data = Data::new(&vec![1., 2., 3.]);
        let msg = msg::Request::new(request_nbr, &data);
        let mpk = rmp_serde::encode::to_vec(&msg).unwrap();

        while let Err(msg) = requester.send(&mpk, 0) {
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
                        let mut response = zmq::Message::new();
                        match requester.recv(&mut response, 0) {
                            Ok(_) => {
                                let response: msg::Response =
                                    rmp_serde::decode::from_slice(&response).unwrap();
                                println!("Received '{:?}': {}", response, request_nbr);
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
