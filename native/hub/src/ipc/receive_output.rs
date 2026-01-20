use rinf::RustSignal;
use tinic_ipc_protocol::out::{GameState, ProtocolOut, SaveStateInfo, WindowState};

use crate::signals::rust_to_dart::{
    app::OnAppExitOutSignal,
    device::{
        OnDeviceButtonPressedOutSignal, OnDeviceConnectedOutSignal, OnDeviceDisconnectedOutSignal,
    },
    game::{
        GameStateChange, OnGameStateChangeOutSignal, OnLoadStateResultOutSignal,
        OnSaveStateErrorOutSignal, OnSaveStateInfoOutSignal,
    },
    window::{OnKeyboardStateOutSignal, OnWindowClosedOutSignal, OnWindowOpenedOutSignal},
};

pub fn receive_output(protocol_out: ProtocolOut) {
    match protocol_out {
        ProtocolOut::DeviceConnected { id, name } => {
            OnDeviceConnectedOutSignal { id, name }.send_signal_to_dart()
        }
        ProtocolOut::DeviceDisconnected { id, name } => {
            OnDeviceDisconnectedOutSignal { id, name }.send_signal_to_dart()
        }
        ProtocolOut::DeviceButtonPressed { id, name, button } => {
            OnDeviceButtonPressedOutSignal { id, name, button }.send_signal_to_dart()
        }
        ProtocolOut::WindowStateChange { state } => match state {
            WindowState::Opened => OnWindowOpenedOutSignal.send_signal_to_dart(),
            WindowState::Closed => OnWindowClosedOutSignal.send_signal_to_dart(),
        },
        ProtocolOut::GameStateChange { state } => match state {
            GameState::Closed => OnGameStateChangeOutSignal {
                state: GameStateChange::Closed,
            }
            .send_signal_to_dart(),
            GameState::Paused => OnGameStateChangeOutSignal {
                state: GameStateChange::Paused,
            }
            .send_signal_to_dart(),
            GameState::Running => OnGameStateChangeOutSignal {
                state: GameStateChange::Running,
            }
            .send_signal_to_dart(),
        },
        ProtocolOut::SaveStateResult { info } => match info {
            SaveStateInfo::Susses {
                save_path,
                save_img_preview,
            } => OnSaveStateInfoOutSignal {
                save_path,
                save_img_preview,
            }
            .send_signal_to_dart(),
            SaveStateInfo::Failed => OnSaveStateErrorOutSignal.send_signal_to_dart(),
        },
        ProtocolOut::LoadStateResult { success } => {
            OnLoadStateResultOutSignal { success }.send_signal_to_dart()
        }
        ProtocolOut::KeyboardState { using } => {
            OnKeyboardStateOutSignal { using }.send_signal_to_dart()
        }
        ProtocolOut::AppExited => {
            OnAppExitOutSignal.send_signal_to_dart();
        }
    }
}
