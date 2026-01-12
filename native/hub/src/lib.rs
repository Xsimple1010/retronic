//! This `hub` crate is the
//! entry point of the Rust logic.

mod actors;
mod ipc;
mod signals;
// mod tinic_super_event_listener;

use actors::create_actors;
use rinf::{dart_shutdown, write_interface};
// use tinic_super::event::TinicSuperEventListener;
// use tinic_super::tinic_super::TinicSuper;
use tokio::spawn;
// Uncomment below to target the web.
// use tokio_with_wasm::alias as tokio;

write_interface!();

// You can go with any async library, not just `tokio`.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    spawn(create_actors());

    // Keep the main function running until Dart shutdown.
    dart_shutdown().await;
    println!("Shutting down... flutter");
}
