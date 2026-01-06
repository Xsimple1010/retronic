use crate::ipc::protocol::out::ProtocolOut;
use crate::signals::{AppExited, DeviceButtonPressed, DeviceConnected, DeviceDisconnected, GameClosed, GameLoadedResult, GamePaused, GameResumed, KeyboardState, LoadStateResult, SaveStateResult, WindowClosed, WindowOpened};
use rinf::RustSignal;

pub fn receive_output(protocol_out: ProtocolOut) {
    match protocol_out {
        ProtocolOut::DeviceConnected { id, name } => {
            DeviceConnected { id, name }.send_signal_to_dart()
        }
        ProtocolOut::DeviceDisconnected { id, name } => {
            DeviceDisconnected { id, name }.send_signal_to_dart()
        }
        ProtocolOut::DeviceButtonPressed { id, name, button } => {
            DeviceButtonPressed { id, name, button }.send_signal_to_dart()
        }
        ProtocolOut::WindowOpened => {
            WindowOpened.send_signal_to_dart();
        }
        ProtocolOut::WindowClosed => {
            WindowClosed.send_signal_to_dart();
        }
        ProtocolOut::GameLoadedResult { success } => {
            GameLoadedResult { success }.send_signal_to_dart()
        }
        ProtocolOut::GameClosed => {
            GameClosed.send_signal_to_dart();
        }
        ProtocolOut::GamePaused => {
            GamePaused.send_signal_to_dart()
        }
        ProtocolOut::GameResumed => {
            GameResumed.send_signal_to_dart()
        }
        ProtocolOut::SaveStateResult { success } => {
            SaveStateResult { success }.send_signal_to_dart()
        }
        ProtocolOut::LoadStateResult { success } => {
            LoadStateResult { success }.send_signal_to_dart()
        }
        ProtocolOut::KeyboardState { using } => {
            KeyboardState { using }.send_signal_to_dart()
        }
        ProtocolOut::AppExited => {
            AppExited.send_signal_to_dart();
        }
    }
}
