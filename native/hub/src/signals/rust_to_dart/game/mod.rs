use rinf::{RustSignal, SignalPiece};
use serde::Serialize;

#[derive(Serialize, RustSignal)]
pub struct OnGameStateChangeOutSignal {
    pub state: GameStateChange,
}

#[derive(Serialize, SignalPiece)]
pub enum GameStateChange {
    Running,
    Closed,
    Paused,
}

#[derive(Serialize, RustSignal)]
pub struct OnSaveStateInfoOutSignal {
    pub save_path: String,
    pub save_img_preview: String,
}

#[derive(Serialize, RustSignal)]
pub struct OnSaveStateErrorOutSignal;

#[derive(Serialize, RustSignal)]
pub struct OnLoadStateResultOutSignal {
    pub success: bool,
}
