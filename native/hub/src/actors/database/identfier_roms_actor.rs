use async_trait::async_trait;
use messages::context::Context;
use messages::prelude::Notifiable;
use rinf::RustSignal;
use std::{path::PathBuf, sync::Arc};
use tinic_database::query;
use tinic_super::GameIdentifier;
use tokio::task::JoinSet;

use crate::{
    app_state::AppState,
    impl_dart_actor,
    signals::{
        dart_to_rust::database::GetRomsFromDir,
        rust_to_dart::database::{GameRom, OnGetRomsFromDirOutSignal},
    },
};

pub struct IdentifierRomsActor {
    app_state: Arc<AppState>,
    _owned_tasks: JoinSet<()>,
}

impl_dart_actor!(actor = IdentifierRomsActor, signal = GetRomsFromDir);

#[async_trait]
impl Notifiable<GetRomsFromDir> for IdentifierRomsActor {
    async fn notify(&mut self, input: GetRomsFromDir, _: &Context<Self>) {
        println!("Received GetRomsFromDir signal");

        let rom_identifiers = match GameIdentifier::from_dir(input.dir.into()).await {
            Ok(identifiers) => identifiers,
            Err(err) => {
                println!("Failed to identify ROMs: {:?}", err);
                return;
            }
        };
        println!("Received GetRomsFromDir signal");

        let out = {
            let look_conn = self.app_state.db_conn.lock().await;
            let conn = match &*look_conn {
                Some(conn) => conn,
                None => return,
            };

            let crc32s = rom_identifiers.iter().map(|r| r.crc).collect::<Vec<u32>>();

            match query::select_by_crc32_list(conn, &crc32s) {
                Ok(roms) => roms,
                Err(err) => {
                    println!("Failed to query ROMs: {:?}", err);
                    return;
                }
            }
        };
        println!("Received GetRomsFromDir signal");

        let retro_paths = match self.app_state.get_base_retro_path().await {
            Some(retro_paths) => retro_paths,
            None => return,
        };

        let art_dir: PathBuf = retro_paths.arts.to_string().into();
        println!("Received GetRomsFromDir signal");

        OnGetRomsFromDirOutSignal {
            roms: out
                .into_iter()
                .map(|r| GameRom::from_db(r, &art_dir))
                .collect(),
        }
        .send_signal_to_dart();
    }
}
