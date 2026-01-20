use rinf::RustSignal;
use serde::Serialize;

use crate::signals::generic::CoreInfoSignalPiece;

#[derive(Serialize, RustSignal)]
pub struct OnCoreInstalledOutSignal {
    pub core_name: String,
}

#[derive(Serialize, RustSignal)]
pub struct OnGetCompatCoreInfosOutSignal {
    pub infos: Vec<CoreInfoSignalPiece>,
}
