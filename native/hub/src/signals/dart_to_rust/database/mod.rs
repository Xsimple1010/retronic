use rinf::DartSignal;
use serde::Deserialize;

#[derive(Deserialize, DartSignal)]
pub struct GetAllGameInfosFromRDBSignal;

#[derive(Deserialize, DartSignal)]
pub struct IdentifyRomSignal {
    pub rom_file: String,
}
