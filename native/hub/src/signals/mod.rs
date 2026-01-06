use rinf::DartSignal;
use serde::Deserialize;

#[derive(Deserialize, DartSignal)]
pub struct LoadGame {
    pub rom_path: String,
    pub core_path: String,
    pub base_retro_path: String,
}

#[derive(Deserialize, DartSignal)]
pub struct CloseGame;


#[derive(Deserialize, DartSignal)]
pub struct AppExit;