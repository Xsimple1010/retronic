use rinf::RustSignal;
use serde::Serialize;

#[derive(Serialize, RustSignal)]
pub struct OnExtractFinishedOutSignal {
    pub origin_file: String,
}

#[derive(Serialize, RustSignal)]
pub struct OnExtractingOutSignal {
    pub origin_file: String,
    pub inner_file_name: String,
}
