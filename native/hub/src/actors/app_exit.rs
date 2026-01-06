use crate::ipc::IpcInput;
use crate::signals::AppExit;
use async_trait::async_trait;
use messages::actor::Actor;
use messages::address::Address;
use messages::context::Context;
use messages::prelude::Notifiable;
use rinf::DartSignal;
use tokio::task::JoinSet;

pub struct AppExitActor {
    _owned_tasks: JoinSet<()>,
    ipc_input: IpcInput,
}

impl Actor for AppExitActor {}

impl AppExitActor {
    pub fn new(self_addr: Address<Self>, ipc_input: IpcInput) -> Self {
        let mut _owned_tasks = JoinSet::new();
        _owned_tasks.spawn(Self::listen_to_dart(self_addr));

        AppExitActor {
            _owned_tasks,
            ipc_input,
        }
    }

    async fn listen_to_dart(mut self_addr: Address<Self>) {
        let receiver = AppExit::get_dart_signal_receiver();
        while let Some(signal_pack) = receiver.recv().await {
            let _ = self_addr.notify(signal_pack.message).await;
        }
    }
}

#[async_trait]
impl Notifiable<AppExit> for AppExitActor {
    async fn notify(&mut self, _: AppExit, _: &Context<Self>) {
        // o que fazer quando notificado!
        let _ = self.ipc_input.app_exit();
    }
}
