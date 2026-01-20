use rinf::RustSignal;
use serde::Serialize;

#[derive(Serialize, RustSignal)]
pub struct OnDeviceConnectedOutSignal {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, RustSignal)]
pub struct OnDeviceDisconnectedOutSignal {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, RustSignal)]
pub struct OnDeviceButtonPressedOutSignal {
    pub id: String,
    pub name: String,
    pub button: String,
}
