use rinf::RustSignal;
use serde::Serialize;

#[derive(Serialize, RustSignal)]
pub struct OnDownloadCompletedOutSignal {
    pub file_name: String,
}

#[derive(Serialize, RustSignal)]
pub struct OnExtractFileOutSignal {
    pub file_name: String,
}

#[derive(Serialize, RustSignal)]
pub struct OnDownloadOutSignal {
    pub file_name: String,
    pub progress: f32,
}
