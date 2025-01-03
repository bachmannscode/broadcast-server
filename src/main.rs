use broadcast_server::init_logger;
use clap::Parser;
use env_logger::Target::Stderr;
use std::net::SocketAddr;

#[derive(Parser)]
struct Args {
    /// The port the server will listen on.
    #[clap(short, long, default_value = "127.0.0.1:8888")]
    socket: SocketAddr,

    /// The Rust logging level when running the application.
    #[clap(short, long, default_value = "info")]
    logging_level: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();
    init_logger(args.logging_level, Stderr);
    broadcast_server::run(args.socket).await;
}
