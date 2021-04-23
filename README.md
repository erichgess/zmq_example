# Demo
Demonstrates request/response communication between ZeroMQ nodes.  The server port sets what
port the server listens to and the client port sets what port the client will connect to. For
the client, use the port of the server you want it to connect to.

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