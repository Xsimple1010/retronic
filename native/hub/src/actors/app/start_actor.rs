use crate::app_state::AppState;
use crate::impl_dart_actor;
use crate::signals::dart_to_rust::app::AppStartSignal;
use async_trait::async_trait;
use messages::context::Context;
use messages::prelude::Notifiable;
use std::sync::Arc;
use tokio::task::JoinSet;

pub struct AppStartActor {
    owned_tasks: JoinSet<()>,
    app_state: Arc<AppState>,
}

impl_dart_actor!(actor = AppStartActor, signal = AppStartSignal);

#[async_trait]
impl Notifiable<AppStartSignal> for AppStartActor {
    async fn notify(&mut self, input: AppStartSignal, _: &Context<Self>) {
        println!("AppStartSignal: {:?}", input.tinic_ipc_file);
        let res = self
            .app_state
            .start_app(input.tinic_ipc_file, input.base_retro_path)
            .await;

        if let Err(e) = res {
            print!("{e:?}");
        }
    }
}
