// src/main.rs
//
// Entry point for the sim-server process.
//
// This binary is a long-lived companion to the Agave validator. It binds a
// Unix domain socket, accepts one persistent connection per MevShard thread,
// and services TwoHopSimRequest frames by running the appropriate DEX price
// math and writing TwoHopSimResponse frames back. It never reads from the
// chain directly — all account data it needs is bundled inside each request
// by the validator shard, which snapshots it from the live Bank at the moment
// the pool update fires.
//
// The socket path is taken from the first command-line argument so that the
// validator and this process can be started with a shared path without
// hardcoding it in either binary.

mod account_map;
mod dex;
mod handler;
mod router;
mod server;

use tracing::info;

fn main() {
    // Initialise structured logging. The validator uses tracing throughout and
    // the sim-server follows the same convention so log output from both
    // processes is consistent in format and filterable by the same RUST_LOG
    // environment variable.
    tracing_subscriber::fmt::init();

    let socket_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/tmp/sim-server.sock".to_string());

    info!("sim_server: starting, socket={}", socket_path);

    server::SimServer::start(&socket_path)
        .unwrap_or_else(|e| panic!("sim_server: failed to start: {}", e));

    // The listener thread spawned by SimServer::start runs the accept loop for
    // the entire process lifetime. The main thread parks here indefinitely so
    // the process stays alive. All real work happens on the listener thread and
    // the per-connection handler threads it spawns.
    info!("sim_server: running — waiting for validator connections");
    loop {
        std::thread::park();
    }
}
