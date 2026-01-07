use crate::signals::{
    AppExitedSignal, DeviceButtonPressedSignal, DeviceConnectedSignal, DeviceDisconnectedSignal,
    GameStateChange, GameStateChangeSignal, KeyboardStateSignal, LoadStateResultSignal,
    SaveStateErroSignal, SaveStateInfoSignal, WindowClosedSignal, WindowOpenedSignal,
};
use rinf::RustSignal;
use tinic_ipc_protocol::out::{GameState, ProtocolOut, SaveStateInfo, WindowState};

pub fn receive_output(protocol_out: ProtocolOut) {
    match protocol_out {
        ProtocolOut::DeviceConnected { id, name } => {
            DeviceConnectedSignal { id, name }.send_signal_to_dart()
        }
        ProtocolOut::DeviceDisconnected { id, name } => {
            DeviceDisconnectedSignal { id, name }.send_signal_to_dart()
        }
        ProtocolOut::DeviceButtonPressed { id, name, button } => {
            DeviceButtonPressedSignal { id, name, button }.send_signal_to_dart()
        }
        ProtocolOut::WindowStateChange { state } => match state {
            WindowState::Opened => WindowOpenedSignal.send_signal_to_dart(),
            WindowState::Closed => WindowClosedSignal.send_signal_to_dart(),
        },
        ProtocolOut::GameStateChange { state } => match state {
            GameState::Closed => GameStateChangeSignal {
                state: GameStateChange::Closed,
            }
            .send_signal_to_dart(),
            GameState::Paused => GameStateChangeSignal {
                state: GameStateChange::Paused,
            }
            .send_signal_to_dart(),
            GameState::Running => GameStateChangeSignal {
                state: GameStateChange::Running,
            }
            .send_signal_to_dart(),
        },
        ProtocolOut::SaveStateResult { info } => match info {
            SaveStateInfo::Susses {
                save_path,
                save_img_preview,
            } => SaveStateInfoSignal {
                save_path,
                save_img_preview,
            }
            .send_signal_to_dart(),
            SaveStateInfo::Failed => SaveStateErroSignal.send_signal_to_dart(),
        },
        ProtocolOut::LoadStateResult { success } => {
            LoadStateResultSignal { success }.send_signal_to_dart()
        }
        ProtocolOut::KeyboardState { using } => KeyboardStateSignal { using }.send_signal_to_dart(),
        ProtocolOut::AppExited => {
            AppExitedSignal.send_signal_to_dart();
        }
    }
}
