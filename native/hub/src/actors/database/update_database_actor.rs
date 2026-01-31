use async_trait::async_trait;
use messages::context::Context;
use messages::prelude::Notifiable;
use std::sync::Arc;
use tokio::task::JoinSet;

use crate::{
    app_state::AppState, impl_dart_actor, signals::dart_to_rust::database::UpdateDatabaseSignal,
};

pub struct UpdateDatabaseActor {
    app_state: Arc<AppState>,
    _owned_tasks: JoinSet<()>,
}

impl_dart_actor!(actor = UpdateDatabaseActor, signal = UpdateDatabaseSignal);

#[async_trait]
impl Notifiable<UpdateDatabaseSignal> for UpdateDatabaseActor {
    async fn notify(&mut self, input: UpdateDatabaseSignal, _: &Context<Self>) {
        let lock = self.app_state.tinic_super.lock().await;

        let tinic_super = match &*lock {
            Some(tinic_super) => tinic_super,
            None => return,
        };

        let res = tinic_super.rdb_helper.download(input.force).await;

        if let Err(e) = res {
            println!("{e:?}");
            return;
        }

        let res = tinic_super.rdb_helper.read_rdbs();

        if let Err(e) = res {
            println!("{e:?}");
            return;
        }
    }
}
