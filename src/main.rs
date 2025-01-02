use clap::Parser;
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

pub fn init_logger(logging_level: String) {
    std::env::set_var("RUST_LOG", logging_level);
    env_logger::Builder::new()
        .parse_default_env()
        .format_timestamp(None)
        .format_level(false)
        .format_target(false)
        .init();
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();
    init_logger(args.logging_level);
    broadcast_server::run(args.socket).await;
}
