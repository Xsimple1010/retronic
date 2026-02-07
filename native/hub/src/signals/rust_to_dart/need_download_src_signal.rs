use rinf::RustSignal;
use serde::Serialize;

#[derive(Serialize, RustSignal)]
pub struct OnNeedDownloadSrcSignalOut {
    pub has_core: bool,
    pub has_info: bool,
    pub has_item_in_db: bool,
}
