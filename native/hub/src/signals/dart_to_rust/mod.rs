use rinf::DartSignal;
use serde::Deserialize;
pub mod app;
pub mod core;
pub mod database;
pub mod game;
pub mod info;

#[derive(Deserialize, DartSignal)]
pub struct DownloadThumbnailSignal {
    pub sys_name: String,
    pub rom_name: String,
}






