use rinf::{DartSignal, RustSignal, SignalPiece};
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
pub struct DeviceConnectedSignal {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, RustSignal)]
pub struct DeviceDisconnectedSignal {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, RustSignal)]
pub struct DeviceButtonPressedSignal {
    pub id: String,
    pub name: String,
    pub button: String,
}

#[derive(Serialize, RustSignal)]
pub struct WindowOpenedSignal;

#[derive(Serialize, RustSignal)]
pub struct WindowClosedSignal;

#[derive(Serialize, RustSignal)]
pub struct GameStateChangeSignal {
    pub state: GameStateChange,
}

#[derive(Serialize, SignalPiece)]
pub enum GameStateChange {
    Running,
    Closed,
    Paused,
}

#[derive(Serialize, RustSignal)]
pub struct SaveStateInfoSignal {
    pub save_path: String,
    pub save_img_preview: String,
}

#[derive(Serialize, RustSignal)]
pub struct SaveStateErroSignal;

#[derive(Serialize, RustSignal)]
pub struct LoadStateResultSignal {
    pub success: bool,
}

#[derive(Serialize, RustSignal)]
pub struct KeyboardStateSignal {
    pub using: bool,
}

#[derive(Serialize, RustSignal)]
pub struct AppExitedSignal;

#[derive(Deserialize, DartSignal)]
pub struct StartTinicIpc {
    pub path: String,
}
