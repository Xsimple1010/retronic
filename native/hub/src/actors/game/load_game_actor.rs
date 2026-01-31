use crate::app_state::AppState;
use crate::impl_dart_actor;
use crate::signals::dart_to_rust::game::LoadGameSignal;
use async_trait::async_trait;
use messages::context::Context;
use messages::prelude::Notifiable;
use std::sync::Arc;
use tokio::task::JoinSet;

pub struct LoadGameActor {
    _owned_tasks: JoinSet<()>,
    app_state: Arc<AppState>,
}

impl_dart_actor!(actor = LoadGameActor, signal = LoadGameSignal);

#[async_trait]
impl Notifiable<LoadGameSignal> for LoadGameActor {
    async fn notify(&mut self, input: LoadGameSignal, _: &Context<Self>) {
        if let Some(retro_paths) = self.app_state.get_base_retro_path().await {
            let _ = self.app_state.ipc.input.load_game(
                input.rom_path,
                input.core_path,
                retro_paths.base_dir.to_string(),
            );
        }
    }
}
