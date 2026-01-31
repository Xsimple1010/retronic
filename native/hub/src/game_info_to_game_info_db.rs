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

        // campos que não existem no domínio
        core_path: None,
        rom_path: None,
        rdb_name: None,

        rumble: game.rumble,
    }
}
