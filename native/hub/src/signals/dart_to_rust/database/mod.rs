use rinf::DartSignal;
use serde::Deserialize;

#[derive(Deserialize, DartSignal)]
pub struct GetAllGameInfosFromRDBSignal;

#[derive(Deserialize, DartSignal)]
pub struct GetRomsFromDir {
    pub dir: String,
}

#[derive(Deserialize, DartSignal)]
pub struct UpdateDatabaseSignal {
    pub force: bool,
}
