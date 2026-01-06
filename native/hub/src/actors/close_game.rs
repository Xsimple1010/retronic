use crate::ipc::send_input::IpcInput;
use crate::signals::CloseGame;
use async_trait::async_trait;
use messages::actor::Actor;
use messages::address::Address;
use messages::context::Context;
use messages::prelude::Notifiable;
use rinf::DartSignal;
use tokio::task::JoinSet;

pub struct CloseGameActor {
    _owned_tasks: JoinSet<()>,
    ipc_input: IpcInput,
}

impl Actor for CloseGameActor {}

impl CloseGameActor {
    pub fn new(self_addr: Address<Self>, ipc_input: IpcInput) -> Self {
        let mut _owned_tasks = JoinSet::new();
        _owned_tasks.spawn(Self::listen_to_dart(self_addr));

        CloseGameActor {
            _owned_tasks,
            ipc_input,
        }
    }

    async fn listen_to_dart(mut self_addr: Address<Self>) {
        let receiver = CloseGame::get_dart_signal_receiver();
        while let Some(signal_pack) = receiver.recv().await {
            let _ = self_addr.notify(signal_pack.message).await;
        }
    }
}

#[async_trait]
impl Notifiable<CloseGame> for CloseGameActor {
    async fn notify(&mut self, _: CloseGame, _: &Context<Self>) {
        // o que fazer quando notificado!
        let _ = self.ipc_input.close_game();
    }
}
