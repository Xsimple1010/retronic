use rinf::DartSignal;
use serde::Deserialize;

#[derive(Deserialize, DartSignal)]
pub struct LoadGameSignal {
    pub rom_path: String,
    pub core_path: String,
}

#[derive(Deserialize, DartSignal)]
pub struct CloseGameSignal;
