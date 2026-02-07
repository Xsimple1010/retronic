use async_trait::async_trait;
use messages::context::Context;
use messages::prelude::Notifiable;
use rinf::RustSignal;
use std::sync::Arc;
use tinic_database::query;
use tokio::task::JoinSet;

use crate::{
    app_state::AppState,
    game_info_to_game_info_db::GameInfoPartial,
    impl_dart_actor,
    signals::{
        dart_to_rust::database::GetRecentGamesSignal,
        rust_to_dart::database::OnGetRecentRomOutSignal,
    },
};

pub struct GetRecentRomActor {
    app_state: Arc<AppState>,
    _owned_tasks: JoinSet<()>,
}

impl_dart_actor!(actor = GetRecentRomActor, signal = GetRecentGamesSignal);

#[async_trait]
impl Notifiable<GetRecentGamesSignal> for GetRecentRomActor {
    async fn notify(&mut self, input: GetRecentGamesSignal, _: &Context<Self>) {
        let look_conn = self.app_state.db_conn.lock().await;
        let conn = match &*look_conn {
            Some(conn) => conn,
            None => return,
        };

        let res = query::list_games_with_rom_path_paginated(&conn, input.page, 50);

        if let Ok(items) = res {
            OnGetRecentRomOutSignal {
                items: items
                    .into_iter()
                    .map(|item| GameInfoPartial {
                        crc32: item.crc32,
                        name: item.name,
                        rom_path: item.rom_path,
                        core_path: item.core_path,
                        console_name: item.console_name,
                        last_played_at: item.last_played_at,
                    })
                    .collect(),
            }
            .send_signal_to_dart();
        }
    }
}
