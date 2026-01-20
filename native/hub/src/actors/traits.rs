use crate::app_state::AppState;
use async_trait::async_trait;
use messages::{
    actor::Actor,
    prelude::{Address, Context, Notifiable},
};
use rinf::DartSignal;
use std::sync::Arc;
use tokio::{spawn, task::JoinSet};

pub trait RetronicDartActor<S: DartSignal + Send + 'static>: Actor + DartActorNotify<S> {
    fn run_actor(app_state: Arc<AppState>) {
        let actor_context = Context::new();
        let addr = actor_context.address();

        let owned_tasks = Self::setup(addr);
        let actor = Self::build(app_state, owned_tasks);

        spawn(actor_context.run(actor));
    }

    fn build(app_state: Arc<AppState>, owned_tasks: JoinSet<()>) -> Self;

    fn setup(self_addr: Address<Self>) -> JoinSet<()> {
        let mut _owned_tasks = JoinSet::new();
        _owned_tasks.spawn(Self::listen_to_dart(self_addr));
        _owned_tasks
    }
}

#[async_trait]
pub trait DartActorNotify<S: DartSignal + Send + 'static>: Sized + Actor + Notifiable<S> {
    async fn listen_to_dart(mut self_addr: Address<Self>) {
        let receiver = S::get_dart_signal_receiver();

        while let Some(signal_pack) = receiver.recv().await {
            let _ = self_addr.notify(signal_pack.message).await;
        }
    }
}
