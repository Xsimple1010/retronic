use rinf::SignalPiece;
use serde::{Deserialize, Serialize};
use tinic_database::model::GameInfoInDb;
use tinic_super::rdb_manager::game_model::GameInfo;

pub fn game_info_to_db(game: GameInfo) -> GameInfoInDb {
    GameInfoInDb {
        name: game.name,
        description: game.description,
        genre: game.genre,
        developer: game.developer,
        publisher: game.publisher,
        franchise: game.franchise,
        origin: game.origin,
        rom_name: game.rom_name,

        release_year: game.release_year,
        release_month: game.release_month,

        size: game.size,
        crc32: game.crc32,
        serial: game.serial,
        console_name: Some(game.rdb_name),

        // campos que não existem no domínio
        core_path: None,
        rom_path: None,

        rumble: game.rumble,
        last_played_at: Some(game.last_played_at),
    }
}

#[derive(SignalPiece, Serialize, Deserialize)]
pub struct GameInfoPartial {
    pub crc32: Option<u32>,
    pub name: Option<String>,
    pub rom_path: String,
    pub core_path: Option<String>,
    pub console_name: Option<String>,
    pub last_played_at: Option<i64>,
}
