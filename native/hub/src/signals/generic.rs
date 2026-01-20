use rinf::SignalPiece;
use serde::{Deserialize, Serialize};
use tinic_super::infos::model::CoreInfo;

#[derive(Deserialize, Serialize, SignalPiece, Debug)]
pub struct CoreInfoSignalPiece {
    pub file_name: String,
    pub is_installed: bool,

    // Informações de Software
    pub display_name: String,
    pub authors: String,
    pub supported_extensions: String,
    pub core_name: String,
    pub license: String,
    pub permissions: String,
    pub display_version: String,
    pub categories: String,
    pub description: String,

    // Informações de Hardware
    pub manufacturer: String,
    pub system_name: String,
    pub system_id: String,

    // // Recursos do Libretro
    pub save_state: bool,
    pub save_state_features: String,
    pub cheats: bool,
    pub input_descriptors: bool,
    pub memory_descriptors: bool,
    pub libretro_saves: bool,
    pub core_options: bool,
    pub core_options_version: String,
    pub supports_no_game: bool,
    pub database: Vec<String>,
    pub hw_render: bool,
    pub needs_full_path: bool,
    pub disk_control: bool,
    pub load_subsystem: bool,
    pub required_hw_api: String,
    pub is_experimental: bool,
}

pub fn core_info_to_signal_piece(core: CoreInfo) -> CoreInfoSignalPiece {
    CoreInfoSignalPiece {
        file_name: core.file_name,
        is_installed: core.is_installed,

        // Informações de Software
        display_name: core.display_name,
        authors: core.authors,
        supported_extensions: core.supported_extensions,
        core_name: core.core_name,
        license: core.license,
        permissions: core.permissions,
        display_version: core.display_version,
        categories: core.categories,
        description: core.description,

        // Informações de Hardware
        manufacturer: core.manufacturer,
        system_name: core.system_name,
        system_id: core.system_id,

        // Recursos do Libretro
        save_state: core.save_state,
        save_state_features: core.save_state_features,
        cheats: core.cheats,
        input_descriptors: core.input_descriptors,
        memory_descriptors: core.memory_descriptors,
        libretro_saves: core.libretro_saves,
        core_options: core.core_options,
        core_options_version: core.core_options_version,
        supports_no_game: core.supports_no_game,
        database: core.database,
        hw_render: core.hw_render,
        needs_full_path: core.needs_full_path,
        disk_control: core.disk_control,
        load_subsystem: core.load_subsystem,
        required_hw_api: core.required_hw_api,
        is_experimental: core.is_experimental,
    }
}
