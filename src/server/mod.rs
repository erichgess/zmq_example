use crate::data::*;
use crate::msg;
use std::thread;
use std::time::Duration;

/// Constants
const RETRY_LIMIT: usize = 3;
const RETRY_DELAY_MS: u64 = 1000;
const POLL_TIMEOUT_MS: i64 = 5000;

/// The server will receive data pushed by peers.  It will Parse the event message
/// and act accordingly.  For a Data message, the received data will be stored in
/// memory and then an Ack sent back to the peer.
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

        // Post message to a channel for processing and then send Ack

        thread::sleep(Duration::from_millis(1000));

        let response = msg::Response::new(msg::Status::Good(req.id()));
        let mpk = rmp_serde::encode::to_vec(&response).unwrap();
        responder.send(&mpk, 0).unwrap();
    }
}

/**
This will connect to a peer and handle pushing new state data
 to the peer

 Some quick thoughts on this code:
 1. Find a design that makes sure that all paths will always go through the backoff process for retries.
 2. Find a design that will make it impossible to retry when the success state is achieved.  In the current code
 I have to remember to `break` after successfully receiving an `Ack` or I will just keep sending messages
*/
pub fn client(port: u32) {
    // setup client to the peer at `port` when new data is ready
    // push that dato to the peer
    // Setup ZeroMQ
    let addr = format!("tcp://localhost:{}", port);
    println!("Connecting to {}...\n", addr);

    let context = zmq::Context::new();
    let mut requester = context.socket(zmq::REQ).unwrap();
    println!("New Socket: {:?}", requester.get_identity().unwrap());
    assert!(requester.connect(&addr).is_ok());

    // Listen to for new data to be ready to push
    for request_nbr in 0..10 {
        println!("Sending Data ID {}...", request_nbr);
        let data = Data::new(&vec![1., 2., 3.]);
        let msg = msg::Request::new(request_nbr, &data);
        let mpk = rmp_serde::encode::to_vec(&msg).unwrap();

        // Push data to peer
        let mut attempts = 0;
        loop {
            attempts += 1;

            if attempts > RETRY_LIMIT {
                println!(
                    "Exceeded max retry limit ({}). Dropping message",
                    RETRY_LIMIT
                );
                break;
            } else if attempts > 1 {
                println!("Wait {}ms then retry...", RETRY_DELAY_MS);
                thread::sleep(Duration::from_millis(RETRY_DELAY_MS));
            }

            match requester.send(&mpk, 0) {
                Ok(_) => (),
                Err(msg) => {
                    println!("Send Error: {}", msg);
                    continue;
                }
            }

            // Wait for peer to Ack the message
            println!("Waiting for server...");
            match requester.poll(zmq::PollEvents::POLLIN, POLL_TIMEOUT_MS) {
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
                        println!("Timeout.");
                        println!("Dropping socket");
                        drop(requester);
                        println!("Creating new socket");
                        requester = context.socket(zmq::REQ).unwrap();
                        assert!(requester.connect(&addr).is_ok());
                    }
                }
                Err(msg) => println!("Polling Error: {}", msg),
            }
        }
    }
    println!("Leaving client function");
}
