# Questions
1. What happens when the server goes offline and the client times out on send? In my test, when I bring the server back online things are still broken.  Do I need to reconnect the client?
 - https://lucumr.pocoo.org/2012/6/26/disconnects-are-good-for-you/

2. What about Pollers, how do those work and can I use them?

3. Do I need matching send and receives with ZeroMQ?
- https://zeromq.org/socket-api/#request-reply-pattern Looks like REQ/REP are synchronous communication?  i.e. if I send a message I must wait to get a response.  `A REQ socket is used by a client to send requests to and receive replies from a service. This socket type allows only an alternating sequence of sends and subsequent receive calls.`
- This would imply serial communication, unless I built a fan out option.
- I need to be able to send out N messages and asynchronously wait for the responses to come back?

4. I need direct communication with a peer.  one-to-one.  A set of data will be targeted for a specific peer and it must go to that peer and that peer alone and I must confirm that it arrived at that peer.  A node will have N clients, one for each of its adjacent nodes.

5. A node will receive incoming messages from several peers.  The message types will be the same but the contents will be different (each peer will be sending data for different cells).  A peer will only need to run one server if that server can have many open connections.

5. I will need to send out many messages to multiple peers. Should this be done serially or concurrently?  With ZeroMQ, how would I have multiple clients each sending different messages at the same time?   On the server side, should message handling be synchronous or asynchronous?  If sending out concurrent messages, then will need Message IDs so that the sender can confirm the receipt of each message.  What about backpressure?