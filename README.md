# Broadcast Server

A simple single-threaded asynchronous TCP broadcast server implemented in Rust using `tokio`. This server allows multiple clients to connect and broadcast messages to all other connected clients. It can be tested using `netcat` (`nc`).

## Features

- Handles multiple clients asynchronously
- Uses a single-threaded asynchronous approach
- Clients receive all messages broadcasted by others
- Messages are delimited by newlines (`\n`)
- Simple logging for connection and message tracking

## Usage

### Running the Server

The server listens for incoming TCP connections and broadcasts messages to all connected clients. You can start the server simply with:

```sh
cargo run
```

or choose your own port with:
```sh
cargo run -- --socket 127.0.0.1:[PORT]
```

or by choosing a logging level for example only log errors with:
```sh
cargo run -- --logging-level error
```

### Connecting Clients with `netcat`

You can connect to the server using `nc` and start sending messages:

```sh
nc localhost 8888
```

Upon connection, you will receive a message indicating your assigned client ID
which corresponds to the assigned port:

```
LOGIN:[CLIENT_ID]
```

Messages from other clients will be broadcasted in the format:

```
MESSAGE:[SENDER_ID] [MESSAGE]
```

When you send a message, you receive an acknowledgment confirming that the server received it, but not necessarily that all connected clients did.

```
ACK:MESSAGE
```

### Example Session

#### Terminal 1 (Server)
```sh
cargo run --release -- --socket 127.0.0.1:8888
```
```
listening on port 8888
connected 127.0.0.1 25565
connected 127.0.0.1 41337
message 25565 Hello, world!
message 41337 Replying to 25565
```

#### Terminal 2 (Client Alice)
```sh
nc localhost 8888
```
```
LOGIN:25565
Hello, world!
ACK:MESSAGE
MESSAGE:41337 Replying to 25565
```

#### Terminal 3 (Client Bob)
```sh
nc localhost 8888
```
```
LOGIN:41337
MESSAGE:25565 Hello, world!
Replying to 25565
ACK:MESSAGE
```

## Implementation Details

- Uses `tokio` for asynchronous networking.
- Utilizes `broadcast` channels for message distribution.
- Logs client connections, disconnections, and messages.

## License

This project is released under the MIT License.
