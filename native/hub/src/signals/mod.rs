use rinf::{DartSignal, RustSignal};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, DartSignal)]
pub struct LoadGame {
    pub rom_path: String,
    pub core_path: String,
    pub base_retro_path: String,
}

#[derive(Deserialize, DartSignal)]
pub struct CloseGame;

#[derive(Deserialize, DartSignal)]
pub struct AppExit;

// GAMEPAD

#[derive(Serialize, RustSignal)]
pub struct DeviceConnected {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, RustSignal)]
pub struct DeviceDisconnected {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, RustSignal)]
pub struct DeviceButtonPressed {
    pub id: String,
    pub name: String,
    pub button: String,
}

#[derive(Serialize, RustSignal)]
pub struct WindowOpened;

#[derive(Serialize, RustSignal)]
pub struct WindowClosed;

#[derive(Serialize, RustSignal)]
pub struct GameLoadedResult {
    pub success: bool,
}

#[derive(Serialize, RustSignal)]
pub struct GameClosed;

#[derive(Serialize, RustSignal)]
pub struct GamePaused;

#[derive(Serialize, RustSignal)]
pub struct GameResumed;

#[derive(Serialize, RustSignal)]
pub struct SaveStateResult {
    pub success: bool,
}

#[derive(Serialize, RustSignal)]
pub struct LoadStateResult {
    pub success: bool,
}

#[derive(Serialize, RustSignal)]
pub struct KeyboardState {
    pub using: bool,
}

#[derive(Serialize, RustSignal)]
pub struct AppExited;
