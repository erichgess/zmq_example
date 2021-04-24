# Demo
Demonstrates request/response communication between ZeroMQ nodes.  The server port sets what
port the server listens to and the client port sets what port the client will connect to. For
the client, use the port of the server you want it to connect to.

Expanded to do a demo of ping/pong computation.  Setting up a peer to peer network will result
in each node running a calculation, then pushing the result to a peer which will continue the
calculation, in turn, pushing its result to another peer.

# Requirements
This uses ZeroMQ to handle the networking and messaging layer. Make sure that you have the
core ZeroMQ library installed:

On MacOS:
```
brew install zmq
```

On Fedora:
```
dnf install zeromq-devel
```

On Ubuntu:
```
apt-get install libzmq3-dev
```

# Usage
You can have a node talk with itself by setting the client and server ports as the same.

```
cargo run -- -s 5555 -c 5555
```

You can have two nodes talk with each other via:
```
cargo run -- -s 5555 -c 7878
cargo run -- -s 7878 -c 5555
```

Or you can have a ring:
```
cargo run -- -s 5555 -c 7878
cargo run -- -s 7878 -c 9090
cargo run -- -s 9090 -c 5555
```

None of these setups will actually result in a calculation being done, because there is no
input data to initiate the worker.  So, you need to tell one of the nodes to "prime" the pump
with the `-p/--prime` flag, this will send an initial value to the worker thread and begin
the calculation. (Multiple nodes can be set to prime the pump, the results could be unpredictable
with this :) )