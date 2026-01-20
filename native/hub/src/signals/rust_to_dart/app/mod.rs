use rinf::RustSignal;
use serde::Serialize;

#[derive(Serialize, RustSignal)]
pub struct OnAppExitOutSignal;

#[derive(Serialize, RustSignal)]
pub struct OnAppStartOutSignal {
    pub tinic_ipc_file: String,
    pub base_retro_path: String,
}
