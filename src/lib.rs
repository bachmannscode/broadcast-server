use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    TcpListener, TcpStream,
};
use tokio::sync::broadcast::{channel, Receiver, Sender};

#[derive(Clone, Debug)]
struct Message {
    msg: Arc<String>,
    src: u16,
}

async fn outgoing(mut writer: OwnedWriteHalf, mut rx: Receiver<Message>, port: u16) {
    let mut result: tokio::io::Result<()>;
    while let Ok(Message { msg, src }) = rx.recv().await {
        if src == port {
            result = writer.write_all(b"ACK:MESSAGE\n").await;
        } else {
            result = writer
                .write_all(format!("MESSAGE:{src} {msg}\n").as_bytes())
                .await;
        };
        if result.is_err() {
            break;
        }
    }
}

async fn incoming(reader: OwnedReadHalf, tx: Sender<Message>, addr: SocketAddr) {
    let port = addr.port();
    let mut lines = BufReader::new(reader).lines();
    loop {
        match lines.next_line().await {
            Ok(Some(msg)) => {
                log::info!("message {port} {}", &msg);
                let _ = tx.send(Message {
                    msg: msg.into(),
                    src: port,
                });
            }
            Err(e) if e.kind() == std::io::ErrorKind::ConnectionReset => {
                log::info!("disconnected {} {port}", addr.ip());
                break;
            }
            Ok(None) => {
                log::info!("disconnected {} {port}", addr.ip());
                break;
            }
            _ => {
                break;
            }
        }
    }
}

async fn handle_connection(
    socket: TcpStream,
    peer_addr: SocketAddr,
    tx: Sender<Message>,
    rx: Receiver<Message>,
) {
    let peer_port = peer_addr.port();
    log::info!("connected {} {}", peer_addr.ip(), peer_port);
    let login_msg = format!("LOGIN:{}\n", peer_port);
    let (reader, mut writer) = socket.into_split();
    let _ = writer.write_all(login_msg.as_bytes()).await;
    tokio::spawn(outgoing(writer, rx, peer_port));
    tokio::spawn(incoming(reader, tx, peer_addr));
}

/// A braodcast server that relays incoming messages to all connected TCP streams.
pub async fn run(addr: SocketAddr) {
    let mut port = addr.port();
    let listener = match TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(_) => {
            let l = TcpListener::bind("127.0.0.1:0").await.unwrap();
            port = l.local_addr().unwrap().port();
            log::warn!("Not able to use {addr}, fallback to 127.0.0.1:{port}.");
            l
        }
    };
    log::info!("listening on port {port}");
    let (tx, _) = channel::<Message>(16);
    while let Ok((socket, peer_addr)) = listener.accept().await {
        tokio::spawn(handle_connection(
            socket,
            peer_addr,
            tx.clone(),
            tx.subscribe(),
        ));
    }
}
