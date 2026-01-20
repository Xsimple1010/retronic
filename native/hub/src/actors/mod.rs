mod app;
mod game;
mod info;
mod macros;
mod traits;

use crate::{
    actors::{
        app::{exit_actor::AppExitActor, start_actor::AppStartActor},
        game::{close_game_actor::CloseGameActor, load_game_actor::LoadGameActor},
        info::{download_infos_actor::DownloadInfoActor, get_campt_info_actor::GetCompatInfoActor},
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
    // start_actor::<HasCoreInstalledActor>(app_state.clone());
    // start_actor::<DownloadCoreActor>(app_state.clone());
    // start_actor::<InstallCoreActor>(app_state.clone());

    // INFO
    GetCompatInfoActor::run_actor(app_state.clone());
    DownloadInfoActor::run_actor(app_state.clone());

    // // DATABASE
    // start_actor::<UpdateDatabaseActor>(app_state.clone());
}
