use async_trait::async_trait;
use messages::context::Context;
use messages::prelude::Notifiable;
use rinf::RustSignal;
use std::{path::PathBuf, sync::Arc};
use tokio::task::JoinSet;

use crate::{
    app_state::AppState,
    impl_dart_actor,
    signals::{
        dart_to_rust::info::GetCompatInfosSignal,
        generic::{CoreInfoSignalPiece, core_info_to_signal_piece},
        rust_to_dart::core::OnGetCompatCoreInfosOutSignal,
    },
};

pub struct GetCompatInfoActor {
    app_state: Arc<AppState>,
    _owned_tasks: JoinSet<()>,
}

impl_dart_actor!(actor = GetCompatInfoActor, signal = GetCompatInfosSignal);

#[async_trait]
impl Notifiable<GetCompatInfosSignal> for GetCompatInfoActor {
    async fn notify(&mut self, input: GetCompatInfosSignal, _: &Context<Self>) {
        let lock = self.app_state.tinic_super.lock().await;

        let tinic_super = match &*lock {
            Some(tinic_super) => tinic_super,
            None => return,
        };

        let has_infos = tinic_super.info_helper.has_infos_installed();

        if has_infos {
            let infos: Vec<CoreInfoSignalPiece> = tinic_super
                .info_helper
                .get_compatibility_core_infos(&PathBuf::from(&input.rom_file))
                .await
                .into_iter()
                .map(|core| core_info_to_signal_piece(core))
                .collect();

            OnGetCompatCoreInfosOutSignal { infos }.send_signal_to_dart()
        } else {
            let _ = tinic_super.info_helper.download(true).await;
        }
    }
}
