use crate::ipc::send_input::IpcInput;
use crate::signals::LoadGame;
use async_trait::async_trait;
use messages::actor::Actor;
use messages::address::Address;
use messages::context::Context;
use messages::prelude::Notifiable;
use rinf::DartSignal;
use tokio::task::JoinSet;

pub struct LoadGameActor {
    _owned_tasks: JoinSet<()>,
    ipc_input: IpcInput,
}

impl Actor for LoadGameActor {}

impl LoadGameActor {
    pub fn new(self_addr: Address<Self>, ipc_input: IpcInput) -> Self {
        let mut _owned_tasks = JoinSet::new();
        _owned_tasks.spawn(Self::listen_to_dart(self_addr));

        LoadGameActor {
            _owned_tasks,
            ipc_input,
        }
    }

    async fn listen_to_dart(mut self_addr: Address<Self>) {
        let receiver = LoadGame::get_dart_signal_receiver();
        while let Some(signal_pack) = receiver.recv().await {
            let _ = self_addr.notify(signal_pack.message).await;
        }
    }
}

#[async_trait]
impl Notifiable<LoadGame> for LoadGameActor {
    async fn notify(&mut self, input: LoadGame, _: &Context<Self>) {
        // o que fazer quando notificado!
        let _ = self
            .ipc_input
            .load_game(input.rom_path, input.core_path, input.base_retro_path);
    }
}
