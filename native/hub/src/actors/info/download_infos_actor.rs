use async_trait::async_trait;
use messages::context::Context;
use messages::prelude::Notifiable;
use std::sync::Arc;
use tokio::task::JoinSet;

use crate::{
    app_state::AppState, impl_dart_actor, signals::dart_to_rust::info::UpdateInfoSignal,
};

pub struct DownloadInfoActor {
    app_state: Arc<AppState>,
    _owned_tasks: JoinSet<()>,
}

impl_dart_actor!(actor = DownloadInfoActor, signal = UpdateInfoSignal);

#[async_trait]
impl Notifiable<UpdateInfoSignal> for DownloadInfoActor {
    async fn notify(&mut self, input: UpdateInfoSignal, _: &Context<Self>) {
        let lock = self.app_state.tinic_super.lock().await;

        let tinic_super = match &*lock {
            Some(tinic_super) => tinic_super,
            None => return,
        };

        let _ = tinic_super.info_helper.download(input.force).await;
    }
}
