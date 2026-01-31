use crate::app_state::AppState;
use crate::impl_dart_actor;
use crate::signals::dart_to_rust::game::CloseGameSignal;
use async_trait::async_trait;
use messages::context::Context;
use messages::prelude::Notifiable;
use std::sync::Arc;
use tokio::task::JoinSet;

pub struct CloseGameActor {
    _owned_tasks: JoinSet<()>,
    app_state: Arc<AppState>,
}

impl_dart_actor!(actor = CloseGameActor, signal = CloseGameSignal);

#[async_trait]
impl Notifiable<CloseGameSignal> for CloseGameActor {
    async fn notify(&mut self, _: CloseGameSignal, _: &Context<Self>) {
        let _ = self.app_state.ipc.input.close_game();
    }
}
