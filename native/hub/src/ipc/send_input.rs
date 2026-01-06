use crate::ipc::protocol::input::ProtocolInput;
use std::io::Write;
use std::process::ChildStdin;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct IpcInput {
    stdin: Arc<Mutex<ChildStdin>>,
}

impl IpcInput {
    pub fn new(stdin: ChildStdin) -> Self {
        Self {
            stdin: Arc::new(Mutex::new(stdin)),
        }
    }

    pub fn send_command(&self, cmd: ProtocolInput) -> std::io::Result<()> {
        let json = serde_json::to_string(&cmd)?;
        let mut stdin = match self.stdin.lock() {
            Ok(stdin) => stdin,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "stdin lock failed",
                ));
            }
        };
        writeln!(stdin, "{json}")?;
        stdin.flush()?;
        Ok(())
    }

    pub fn load_game(
        &self,
        rom_path: String,
        core_path: String,
        base_retro_path: String,
    ) -> std::io::Result<()> {
        self.send_command(ProtocolInput::LoadGame {
            rom_path,
            core_path,
            base_retro_path,
        })
    }

    pub fn close_game(&self) -> std::io::Result<()> {
        self.send_command(ProtocolInput::GameClose)
    }

    pub fn app_exit(&self) -> std::io::Result<()> {
        self.send_command(ProtocolInput::Exit)
    }
}
