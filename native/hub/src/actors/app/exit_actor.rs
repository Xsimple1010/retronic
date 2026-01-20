use crate::app_state::AppState;
use crate::impl_dart_actor;
use crate::signals::dart_to_rust::app::AppExitSignal;
use async_trait::async_trait;
use messages::context::Context;
use messages::prelude::Notifiable;
use std::sync::Arc;
use tokio::task::JoinSet;

pub struct AppExitActor {
    owned_tasks: JoinSet<()>,
    app_state: Arc<AppState>,
}
impl_dart_actor!(actor = AppExitActor, signal = AppExitSignal);

#[async_trait]
impl Notifiable<AppExitSignal> for AppExitActor {
    async fn notify(&mut self, _: AppExitSignal, _: &Context<Self>) {
        let _ = self.app_state.ipc.input.app_exit();
    }
}
