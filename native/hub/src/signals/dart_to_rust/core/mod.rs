use rinf::DartSignal;
use serde::Deserialize;

#[derive(Deserialize, DartSignal)]
pub struct InstallCoresSignal {
    pub cores: Vec<String>,
}
