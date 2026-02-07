use async_trait::async_trait;
use messages::context::Context;
use messages::prelude::Notifiable;
use rinf::RustSignal;
use std::sync::Arc;
use tinic_database::query;
use tokio::task::JoinSet;

use crate::{
    app_state::AppState,
    impl_dart_actor,
    signals::{
        dart_to_rust::need_download_src_signal::NeedDownloadSrcSignal,
        rust_to_dart::need_download_src_signal::OnNeedDownloadSrcSignalOut,
    },
};

pub struct NeedDownloadSrcActor {
    app_state: Arc<AppState>,
    _owned_tasks: JoinSet<()>,
}

impl_dart_actor!(actor = NeedDownloadSrcActor, signal = NeedDownloadSrcSignal);

#[async_trait]
impl Notifiable<NeedDownloadSrcSignal> for NeedDownloadSrcActor {
    async fn notify(&mut self, _: NeedDownloadSrcSignal, _: &Context<Self>) {
        let lock = self.app_state.tinic_super.lock().await;

        let tinic_super = match &*lock {
            Some(tinic_super) => tinic_super,
            None => return,
        };

        let has_core = match tinic_super.core_helper.has_installed() {
            Ok(has_core) => has_core,
            Err(_) => false,
        };

        let has_info = tinic_super.info_helper.has_infos_installed();

        let has_item_in_db = {
            let look_conn = self.app_state.db_conn.lock().await;
            let conn = match &*look_conn {
                Some(conn) => conn,
                None => return,
            };

            match query::list_consoles(conn) {
                Ok(e) => {
                    println!("e: {:?}", e);
                    !e.is_empty()
                }
                Err(_) => false,
            }
        };

        OnNeedDownloadSrcSignalOut {
            has_core,
            has_info,
            has_item_in_db,
        }
        .send_signal_to_dart();
    }
}
