use crate::data::*;
use crate::msg;
use std::thread;
use std::time::Duration;

pub fn server(port: u32) {
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

        let response = msg::Response::new(msg::Status::Good(req.id()));
        let mpk = rmp_serde::encode::to_vec(&response).unwrap();
        responder.send(&mpk, 0).unwrap();
    }
}

pub fn client(port: u32) {
    // Setup ZeroMQ
    let addr = format!("tcp://localhost:{}", port);
    println!("Connecting to {}...\n", addr);

    let context = zmq::Context::new();
    let mut requester = context.socket(zmq::REQ).unwrap();
    println!("New Socket: {:?}", requester.get_identity().unwrap());
    assert!(requester.connect(&addr).is_ok());

    // Initialize push loop
    for request_nbr in 0..10 {
        println!("Sending Data ID {}...", request_nbr);
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
            match requester.poll(zmq::PollEvents::POLLIN, 5000) {
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
