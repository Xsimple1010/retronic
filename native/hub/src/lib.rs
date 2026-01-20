//! This `hub` crate is the
//! entry point of the Rust logic.
//
mod actors;
mod app_state;
mod game_info_to_game_info_db;
mod ipc;
mod signals;
mod tinic_super_event_listener;

use std::sync::Arc;

use actors::create_actors;
use rinf::{dart_shutdown, write_interface};
use tokio::spawn;

use crate::app_state::AppState;
// Uncomment below to target the web.
// use tokio_with_wasm::alias as tokio;

write_interface!();

// You can go with any async library, not just `tokio`.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app_state = AppState::new();

    spawn(create_actors(Arc::new(app_state)));

    // Keep the main function running until Dart shutdown.
    dart_shutdown().await;
    println!("Shutting down... flutter");
}
