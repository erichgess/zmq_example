# Questions
1. What happens when the server goes offline and the client times out on send? In my test, when I bring the server back online things are still broken.  Do I need to reconnect the client?
 - https://lucumr.pocoo.org/2012/6/26/disconnects-are-good-for-you/
 - https://zguide.zeromq.org/docs/chapter4/#reliable-request-reply

2. What about Pollers, how do those work and can I use them?

3. Do I need matching send and receives with ZeroMQ?
- https://zeromq.org/socket-api/#request-reply-pattern Looks like REQ/REP are synchronous communication?  i.e. if I send a message I must wait to get a response.  `A REQ socket is used by a client to send requests to and receive replies from a service. This socket type allows only an alternating sequence of sends and subsequent receive calls.`
- This would imply serial communication, unless I built a fan out option.
- I need to be able to send out N messages and asynchronously wait for the responses to come back?

4. I need direct communication with a peer.  one-to-one.  A set of data will be targeted for a specific peer and it must go to that peer and that peer alone and I must confirm that it arrived at that peer.  A node will have N clients, one for each of its adjacent nodes.

5. A node will receive incoming messages from several peers.  The message types will be the same but the contents will be different (each peer will be sending data for different cells).  A peer will only need to run one server if that server can have many open connections.

6. I will need to send out many messages to multiple peers. Should this be done serially or concurrently?  With ZeroMQ, how would I have multiple clients each sending different messages at the same time?   On the server side, should message handling be synchronous or asynchronous?  If sending out concurrent messages, then will need Message IDs so that the sender can confirm the receipt of each message.  What about backpressure?

7. A peer may receive the same event multiple times.  For example, if A is attempting to push an event to B and B does not Ack in the Time out, then A will resend the event.  In this case, B could receive the message twice. In such a case, it must appear as if B received the message only once.


## Semantics
SEND Cell To A
CONFIRM A got Cell by receiving an ACK or an ERR. Both mean that A received the message.
IF ACK => Good
IF ERR => Handle
If NO RESPONSE => Handle (Exponential backoff with ceiling + retry)

RECEIVE Cell from A => This means that I got data for a cell from a peer node
ACK Cell From A => What does Ack mean? That A received the message and successfully passed it to the compute layer.
ERR Cell From A => Means that A received the message but it could not be processed, provides error code.

### Optional
REQ From A => Means that A has not received a message from us in the expected amount of time and would like to know why.


## Requirements
1. Be able to send message directly to a specific peer and be able to get a response back.  Request/Response to a specific peer.  This is required for SEND semantics to work
2. Be able to receive messages from multiple peers.  This is required for RECEIVE semantics to work.
3. Be able to send a response back to the specific peer that a message came from.  This is required for ACK & ERR semantics to work.
4. Be able to send multiple messages to a peer, concurrently. SEND/RECV mechanics are asynchronous.  This is required to give flexibility to adapt to needs. This is specifically for SEND/RECV which, even for asynchronous, are required to implement SEND/ACK semantics.
5. Be able to process RECV messages concurrently.  That is, on the receiver side, multiple messages can be received from peers without having to REPLY.  Again, this is for scalable growth and flexibility: to avoid disruptions on peers it may be best to pull N messages off the queue begin processing them concurrently, and RESPOND with ACKs asynchronously.
6. Be able to handle receiving messages out of order and but make it appear to be in order to the user.  For example, if waiting on data for frame 3 and we get data for frames 4, 5, and 6.  Then buffer that data until frame 3 arrives then send frames 3, 4, 5, and 6 to the user (in that order).  Will also need to manage the situation where we never get frame 3 but other frames keep arriving.


## Things I am missing
1. This just covers work communication, it does not cover meta data communication (figuring out who owns what, who needs what data, constructing the topology, etc).


## For ZeroMQ
1. Figure out retry pattern and get that working
2. Figure out how to do async Req/Res
3. Figure out how to do async RECV/ACK
