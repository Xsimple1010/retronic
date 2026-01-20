use rinf::DartSignal;
use serde::Deserialize;

#[derive(Deserialize, DartSignal)]
pub struct AppExitSignal;

#[derive(Deserialize, DartSignal)]
pub struct AppStartSignal {
    pub tinic_ipc_file: String,
    pub base_retro_path: String,
}
