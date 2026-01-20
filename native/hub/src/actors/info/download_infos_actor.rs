use async_trait::async_trait;
use messages::context::Context;
use messages::prelude::Notifiable;
use std::sync::Arc;
use tokio::task::JoinSet;

use crate::{
    app_state::AppState, impl_dart_actor, signals::dart_to_rust::info::DownloadInfoSignal,
};

pub struct DownloadInfoActor {
    app_state: Arc<AppState>,
    owned_tasks: JoinSet<()>,
}

impl_dart_actor!(actor = DownloadInfoActor, signal = DownloadInfoSignal);

#[async_trait]
impl Notifiable<DownloadInfoSignal> for DownloadInfoActor {
    async fn notify(&mut self, input: DownloadInfoSignal, _: &Context<Self>) {
        let lock = self.app_state.tinic_super.lock().await;

        let tinic_super = match &*lock {
            Some(tinic_super) => tinic_super,
            None => return,
        };

        let _ = tinic_super.info_helper.download(input.force).await;
    }
}
