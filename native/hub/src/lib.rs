//! This `hub` crate is the
//! entry point of the Rust logic.

mod actors;
mod ipc;
mod signals;

use crate::ipc::Ipc;
use actors::create_actors;
use rinf::{dart_shutdown, write_interface};
use tokio::spawn;
// Uncomment below to target the web.
// use tokio_with_wasm::alias as tokio;

write_interface!();

// You can go with any async library, not just `tokio`.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (ipc, ipc_input) =
        Ipc::new("/home/aderval/RustroverProjects/Tinic/target/debug/tinic_ipc".to_string())
            .unwrap();

    spawn(create_actors(ipc_input.clone()));

    // Keep the main function running until Dart shutdown.
    dart_shutdown().await;
    println!("Shutting down... flutter");
}
