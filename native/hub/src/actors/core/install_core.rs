use crate::app_state::AppState;
use crate::impl_dart_actor;
use crate::signals::dart_to_rust::core::InstallCoresSignal;
use async_trait::async_trait;
use messages::context::Context;
use messages::prelude::Notifiable;
use std::sync::Arc;
use tokio::task::JoinSet;

pub struct InstallCoreActor {
    _owned_tasks: JoinSet<()>,
    app_state: Arc<AppState>,
}
impl_dart_actor!(actor = InstallCoreActor, signal = InstallCoresSignal);

#[async_trait]
impl Notifiable<InstallCoresSignal> for InstallCoreActor {
    async fn notify(&mut self, input: InstallCoresSignal, _: &Context<Self>) {
        let lock = self.app_state.tinic_super.lock().await;

        let tinic_super = match &*lock {
            Some(tinic_super) => tinic_super,
            None => return,
        };

        match tinic_super.core_helper.has_installed() {
            Ok(has_installed) => {
                if has_installed {
                    tinic_super.core_helper.install(input.cores).await;
                } else {
                    let res = tinic_super.core_helper.download_blocking(false).await;
                    if let Ok(_) = res {
                        tinic_super.core_helper.install(input.cores).await;
                    }
                }
            }
            Err(err) => {
                println!("Error checking if core is installed: {:?}", err);
            }
        }
    }
}
