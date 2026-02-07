use rinf::DartSignal;
use serde::Deserialize;

#[derive(Deserialize, DartSignal)]
pub struct GetAllGameInfosFromRDBSignal;

#[derive(Deserialize, DartSignal)]
pub struct GetRomsFromDirSignal {
    pub dir: String,
}

#[derive(Deserialize, DartSignal)]
pub struct UpdateDatabaseSignal {
    pub force: bool,
}

#[derive(Deserialize, DartSignal)]
pub struct GetRecentGamesSignal {
    pub page: u32,
}
