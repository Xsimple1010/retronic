//! This module contains actors.
//! To build a solid app, avoid communicating by sharing memory.
//! Focus on message passing instead.

mod app_exit;
mod close_game;
mod load_game_actor;
mod start_tinic_ipc;

use crate::actors::app_exit::AppExitActor;
use crate::actors::close_game::CloseGameActor;
use crate::actors::load_game_actor::LoadGameActor;
use crate::actors::start_tinic_ipc::StartTinicIpcActor;
use crate::ipc::Ipc;
use messages::prelude::Context;
use std::sync::Arc;
use tokio::spawn;
// Uncomment below to target the web.
// use tokio_with_wasm::alias as tokio;

/// Creates and spawns the actors in the async system.
pub async fn create_actors() {
    // Though simple async tasks work, using the actor model
    // is highly recommended for state management
    // to achieve modularity and scalability in your app.
    // Actors keep ownership of their state and run in their own loops,
    // handling messages from other actors or external sources,
    // such as websockets or timers.

    let ipc = Arc::new(Ipc::new());

    // Create actor contexts.
    let context = Context::new();
    let addr = context.address();
    // let second_context = Context::new();

    // Spawn the actors.
    let load_game_actor = LoadGameActor::new(addr, ipc.clone());
    spawn(context.run(load_game_actor));

    let close_game_context = Context::new();
    let addr = close_game_context.address();
    let close_game_actor = CloseGameActor::new(addr, ipc.clone());
    spawn(close_game_context.run(close_game_actor));

    let app_exit_context = Context::new();
    let addr = app_exit_context.address();
    let app_exit_actor = AppExitActor::new(addr, ipc.clone());
    spawn(app_exit_context.run(app_exit_actor));

    let start_tinic_ipc_context = Context::new();
    let addr = start_tinic_ipc_context.address();
    let start_tinic_ipc_actor = StartTinicIpcActor::new(addr, ipc.clone());
    spawn(start_tinic_ipc_context.run(start_tinic_ipc_actor));

    // let second_actor = SecondActor::new(first_addr);
}
