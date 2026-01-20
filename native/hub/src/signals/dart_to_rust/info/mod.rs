use rinf::DartSignal;
use serde::Deserialize;

#[derive(Deserialize, DartSignal)]
pub struct GetCompatInfosSignal {
    pub rom_file: String,
}

#[derive(Deserialize, DartSignal)]
pub struct DownloadInfoSignal {
    pub force: bool,
}
