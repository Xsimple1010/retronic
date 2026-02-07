use std::path::PathBuf;

use rinf::{RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};
use tinic_database::model::GameInfoInDb;
use tinic_super::art::{get_thumbnail_url, thumbnail::ThumbnailType};

use crate::game_info_to_game_info_db::GameInfoPartial;

#[derive(RustSignal, Serialize, Deserialize)]
pub struct OnGetRomsFromDirOutSignal {
    pub roms: Vec<GameRom>,
}

#[derive(SignalPiece, Serialize, Deserialize, Default, Clone)]
pub struct RomThumbnail {
    pub local: Option<String>,
    pub remote: String,
}

impl RomThumbnail {
    pub fn new(
        thumbnail_type: &ThumbnailType,
        console_name: &String,
        rom_name: &String,
        art_dir: &PathBuf,
    ) -> Self {
        let remote = get_thumbnail_url(thumbnail_type, &console_name, &rom_name);
        let local = art_dir.join(format!("{}.{}.png", thumbnail_type, rom_name));

        let local = if local.exists() {
            Some(local.to_string_lossy().to_string())
        } else {
            None
        };

        Self { local, remote }
    }
}

#[derive(RustSignal, Serialize, Deserialize)]
pub struct OnReadRdbProgressOutSignal {
    pub total: u16,
    pub remaining: u16,
    pub console_name: String,
}

#[derive(SignalPiece, Serialize, Deserialize)]
pub struct GameRom {
    pub name: Option<String>,
    pub description: Option<String>,
    pub box_img: RomThumbnail,
    pub snap_img: RomThumbnail,
    pub title_img: RomThumbnail,
    pub genre: Option<String>,
    pub developer: Option<String>,
    pub publisher: Option<String>,
    pub franchise: Option<String>,
    pub origin: Option<String>,
    pub rom_name: String,
    pub release_year: Option<u32>,
    pub release_month: Option<u32>,
    pub size: Option<u64>,
    pub crc32: Option<u32>,
    pub serial: Option<String>,
    pub core_path: Option<String>,
    pub rom_path: Option<String>,
    pub rumble: bool,
    pub console_name: String,
}

impl GameRom {
    pub fn from_db(value: GameInfoInDb, art_dir: &PathBuf) -> Self {
        let console_name = value.console_name.unwrap();
        let rom_name = value.rom_name.unwrap();

        Self {
            name: value.name,
            description: value.description,
            box_img: RomThumbnail::new(&ThumbnailType::Box, &console_name, &rom_name, art_dir),
            snap_img: RomThumbnail::new(&ThumbnailType::Snap, &console_name, &rom_name, art_dir),
            title_img: RomThumbnail::new(&ThumbnailType::Titles, &console_name, &rom_name, art_dir),
            genre: value.genre,
            developer: value.developer,
            publisher: value.publisher,
            franchise: value.franchise,
            origin: value.origin,
            rom_name,
            release_year: value.release_year,
            release_month: value.release_month,
            size: value.size,
            crc32: value.crc32,
            serial: value.serial,
            core_path: value.core_path,
            rom_path: value.rom_path,
            rumble: value.rumble,
            console_name,
        }
    }
}

#[derive(Serialize, Deserialize, RustSignal)]
pub struct OnGetRecentRomOutSignal {
    pub items: Vec<GameInfoPartial>,
}
