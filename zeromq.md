# Notes about ZeroMQ
## General Lessons
1. The REQ/REP socket protocol requires serial and synchronous semantics.  A REQ message must be immediately followed by a REP message otherwise the client will fail.
2. This makes timeouts a problem, since a timeout means that a REQ was sent but a REP was not received so we want to move to a new state (TIMEOUT) but the socket cannot move to a new state.  As such, we must use a different mechanism to effect the time out (The Poller) and then, if we timeout, we must destroy and recreate the socket.

One thing to note about this Retry code, all settings for the socket are reset to the default settings.  So, those _must also_ be redone.  This makes me want to have a single code line that handles creating sockets that will be used for both creating the socket at the start of the process and for recreating when executing Retry logic.

```rust
match requester.poll(PollEvents::POLLIN, 5000) {
    Ok(i) => {
        if i > 0 { // `i` is the number of messages waiting on the receiver queue, if 0 then the `poll` timed out
            let mut response = zmq::Message::new();
            match requester.recv(&mut response, 0) {
                Ok(_) => {
                    // Do stuff
                }
                Err(msg) => {
                    panic!("Receive Error: {}", msg);
                }
            }
        } else {
            println!("Timeout and create new connection...");
            drop(requester);
            requester = context.socket(zmq::REQ).unwrap();
            assert!(requester.connect(&addr).is_ok());
            println!("New Socket: {:?}", requester.get_identity().unwrap());
        }
    }
    Err(msg) => println!("Polling Error: {}", msg),
}
```
3. For asynchronous communication we would need a different socket type: 

4. ZeroMQ has several patterns of messaging:
- Request/Response - Clients send requests to servers and get responses.
- Pub/Sub - A publisher publishes data and subscribers get that data.
- Pipeline - A set of tasks connected through a DAG to do work
- Exclusive Pair - For connecting two threads in a process
Pub/Sub is not what we want because it is intended for decoupled logic where the publisher does not care who its subscribers are and the subscribers are not tightly coupled in state with the publisher.  For example, in our situation, it is critical that a node B gets data from node A and if it does not, then A must resend the data.  Pub/Sub is not meant for that mechanism.

Pipeline is meant for heterogenous communication not for shared state. Where each stage in the pipeline is a different system doing different work and there is a clear order.  Our system is distributed state.

For our purposes, Request/Response is what we want.

Want a fan in pattern on the server side and a 1-1 pattern on the client side.  That is, a client is talking to a specific server and only that server since each server owns a different chunk of data.

5. The docs state that working with very big messages is difficult: https://zguide.zeromq.org/docs/chapter2/#Working-with-Messages ?  But my Rust code was working with 1MiB messages without any issues? "So one of the main jobs of a good language binding is to wrap this API up in classes that are easier to use.", keep my eye on the Rust binding.

6. To read from multiple sockets at once use `poll` (this would explain why it takes a slice of PollEvents).
7. Applications use multipart messages for wrapping with metadata?  https://zguide.zeromq.org/docs/chapter2/#Multipart-Messages
8. Take care of the error handling: https://zguide.zeromq.org/docs/chapter2/#Handling-Errors-and-ETERM
```none
The error code is provided in errno or zmq_errno().
A descriptive error text for logging is provided by zmq_strerror().
```

```none
In C/C++, asserts can be removed entirely in optimized code, so don’t make the mistake of wrapping the whole ZeroMQ call in an assert(). It looks neat; then the optimizer removes all the asserts and the calls you want to make, and your application breaks in impressive ways.
```
9. Make sure to work through how to handle interrupt signals: https://zguide.zeromq.org/docs/chapter2/#Handling-Interrupt-Signals
10. Something that we'll need eventually: https://zguide.zeromq.org/docs/chapter2/#Zero-Copy
11. PubSub envelopes: https://zguide.zeromq.org/docs/chapter2/#Pub-Sub-Message-Envelopes Does this address the problem I noted above with PubSub?
12. https://zguide.zeromq.org/docs/chapter2/#High-Water-Marks
```none
When your socket reaches its HWM, it will either block or drop data depending on the socket type. *PUB and ROUTER sockets will drop data* if they reach their HWM, while other socket types will block. Over the inproc transport, the sender and receiver share the same buffers, so the real HWM is the sum of the HWM set by both sides.
```
13. https://zguide.zeromq.org/docs/chapter2/#Missing-Message-Problem-Solver.  This again points to PubSub being the wrong solution, if messages are missing the solution is to start all Subs before the Pub.  Combined with Pub dropping messges with HWM is hit means PubSub is the wrong tool to use for our work.
14. When leaving the function `client` if messages are in the queue, the thread does not complete.  I believe the Drop for ZeroMQ must block until the queue is empty.  As soon as I start the server up, it will exit the function and die.  Interestingly enough, when I do this, I do _not_ get all the messages from the queue, only the most recent 8.  

This raises some questions:
- How are messages ordered in the client side queue?
- Why are only some of them delivered to the server before the client Drops the context?
- Is it the context or the socket or what that is waiting for the server?

I believe the issue comes in `Drop` for `Context`, after logging `context dropped` it then attempts to call this function:
```rust
impl RawContext {
    fn term(&self) -> Result<()> {
        zmq_try!(unsafe { zmq_sys::zmq_ctx_term(self.ctx) });
        Ok(())
    }
}
```
and I think the `zmq_ctx_term` function is what causes the hang.  Which is substantiated from the [docs](http://api.zeromq.org/3-3:zmq-ctx-term):
```none
Context termination is performed in the following steps:

    Any blocking operations currently in progress on sockets open within context shall return immediately with an error code of ETERM. With the exception of zmq_close(), any further operations on sockets open within context shall fail with an error code of ETERM.
    After interrupting all blocking calls, zmq_ctx_term() shall block until the following conditions are satisfied: <itemizedlist> <listitem> All sockets open within context have been closed with zmq_close().
    For each socket within context, all messages sent by the application with zmq_send() have either been physically transferred to a network peer, or the socket's linger period set with the ZMQ_LINGER socket option has expired.
```
Looks like I need to consider setting the linger period.  The default linger is -1.
```none
The default value of -1 specifies an infinite linger period. Pending messages shall not be discarded after a call to zmq_close(); attempting to terminate the socket's context with zmq_term() shall block until all pending messages have been sent to a peer.
```
Of note *"attempting to terminate the socket's context with zmq_term() shall block until either all pending messages have been sent to a peer, or the linger period expires, after which any pending messages shall be discarded."*. This is why `client` hangs.

The function `zmq_close()` is called when a socket is dropped: e.g. during the retry state.  So, we need to think about how linger would effect retries.  Could this explain why I only get a subset of messages when the server comes back online?

DON'T FORGET THAT THIS ALSO ALL APPLIES TO THE SERVER SIDE!

15. When the server receives and Acks messages which the client had decided to drop, one thing I have noticed is that the client does not process the incoming Acks. It looks to know how to properly associate REQ to REP.  Although, I will still have logic to check to make sure that the Acked Message ID is the expected Message ID.