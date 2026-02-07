use async_trait::async_trait;
use messages::context::Context;
use messages::prelude::Notifiable;
use rinf::RustSignal;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tinic_database::query;
use tinic_super::GameIdentifier;
use tokio::task::JoinSet;

use crate::{
    app_state::AppState,
    impl_dart_actor,
    signals::{
        dart_to_rust::database::GetRomsFromDirSignal,
        rust_to_dart::database::{GameRom, OnGetRomsFromDirOutSignal},
    },
};

pub struct IdentifierRomsActor {
    app_state: Arc<AppState>,
    _owned_tasks: JoinSet<()>,
}

impl_dart_actor!(actor = IdentifierRomsActor, signal = GetRomsFromDirSignal);

#[async_trait]
impl Notifiable<GetRomsFromDirSignal> for IdentifierRomsActor {
    async fn notify(&mut self, input: GetRomsFromDirSignal, _: &Context<Self>) {
        let rom_identifiers = match GameIdentifier::from_dir(input.dir.into()).await {
            Ok(identifiers) => identifiers,
            Err(err) => {
                println!("Failed to identify ROMs: {:?}", err);
                return;
            }
        };

        let mut db_infos = {
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

        let mut map_identifiers = HashMap::<u32, GameIdentifier>::new();

        rom_identifiers.into_iter().for_each(|info| {
            map_identifiers.insert(info.crc, info);
        });

        for info in &mut db_infos {
            if let Some(crc32) = &info.crc32 {
                let identifier = map_identifiers.get(&crc32).unwrap();

                let look_conn = self.app_state.db_conn.lock().await;
                let conn = match &*look_conn {
                    Some(conn) => conn,
                    None => return,
                };

                let _lines = query::update_game_paths(
                    &conn,
                    info.crc32,
                    &identifier.file_name,
                    Some(&identifier.path.display().to_string()),
                    match &info.core_path {
                        Some(info_path) => Some(info_path.as_str()),
                        None => None,
                    },
                );

                info.rom_path = Some(identifier.path.display().to_string());
                map_identifiers.remove(crc32);
            }
        }

        if !map_identifiers.is_empty() {
            let mut map_identifiers = map_identifiers
                .into_iter()
                .map(|(_, info)| (info.file_name.clone(), info))
                .collect::<HashMap<String, GameIdentifier>>();

            for info in &mut db_infos {
                if let Some(rom_name) = &info.rom_name {
                    let identifier = map_identifiers.get(rom_name).unwrap();

                    let look_conn = self.app_state.db_conn.lock().await;
                    let conn = match &*look_conn {
                        Some(conn) => conn,
                        None => return,
                    };

                    let _lines = query::update_game_paths(
                        &conn,
                        info.crc32,
                        &identifier.file_name,
                        Some(&identifier.path.display().to_string()),
                        match &info.core_path {
                            Some(info_path) => Some(info_path.as_str()),
                            None => None,
                        },
                    );

                    info.rom_path = Some(identifier.path.display().to_string());
                    map_identifiers.remove(rom_name);
                }
            }
        }

        let retro_paths = match self.app_state.get_base_retro_path().await {
            Some(retro_paths) => retro_paths,
            None => return,
        };

        let art_dir: PathBuf = retro_paths.arts.to_string().into();

        OnGetRomsFromDirOutSignal {
            roms: db_infos
                .into_iter()
                .map(|r| GameRom::from_db(r, &art_dir))
                .collect(),
        }
        .send_signal_to_dart();
    }
}
