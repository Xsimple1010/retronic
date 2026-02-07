mod app;
mod core;
mod database;
mod game;
mod info;
mod macros;
mod need_download_src_actor;
mod traits;

use crate::{
    actors::{
        app::{exit_actor::AppExitActor, start_actor::AppStartActor},
        core::install_core::InstallCoreActor,
        database::{
            get_recent_rom_actor::GetRecentRomActor, identfier_roms_actor::IdentifierRomsActor,
            update_database_actor::UpdateDatabaseActor,
        },
        game::{close_game_actor::CloseGameActor, load_game_actor::LoadGameActor},
        info::{download_infos_actor::DownloadInfoActor, get_campt_info_actor::GetCompatInfoActor},
        need_download_src_actor::NeedDownloadSrcActor,
        traits::RetronicDartActor,
    },
    app_state::AppState,
};
use std::sync::Arc;

pub async fn create_actors(app_state: Arc<AppState>) {
    // // APP
    AppStartActor::run_actor(app_state.clone());
    AppExitActor::run_actor(app_state.clone());

    // // GAME
    LoadGameActor::run_actor(app_state.clone());
    CloseGameActor::run_actor(app_state.clone());

    // CORE
    InstallCoreActor::run_actor(app_state.clone());
    // start_actor::<HasCoreInstalledActor>(app_state.clone());
    // start_actor::<DownloadCoreActor>(app_state.clone());
    // start_actor::<InstallCoreActor>(app_state.clone());

    // INFO
    GetCompatInfoActor::run_actor(app_state.clone());
    DownloadInfoActor::run_actor(app_state.clone());

    // DATABASE
    IdentifierRomsActor::run_actor(app_state.clone());
    UpdateDatabaseActor::run_actor(app_state.clone());
    GetRecentRomActor::run_actor(app_state.clone());

    NeedDownloadSrcActor::run_actor(app_state.clone());
}
