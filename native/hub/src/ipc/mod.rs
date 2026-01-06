use crate::ipc::protocol::input::ProtocolInput;
use crate::ipc::protocol::out::ProtocolOut;
use std::io::{BufRead, BufReader, Write};
use std::process::{ChildStdin, ChildStdout, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

pub mod protocol;

pub struct Ipc {
    alive_thread: Arc<AtomicBool>,
}

impl Ipc {
    pub fn new(path: String) -> Result<(Ipc, IpcInput), String> {
        let mut child = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            // .stderr(Stdio::inherit()) // logs aparecem no console
            .spawn()
            .map_err(|e| e.to_string())?;

        let stdin = child.stdin.take().ok_or("stdin não disponível")?;
        let stdout = child.stdout.take().ok_or("stdout não disponível")?;

        let ipc_input = IpcInput {
            stdin: Arc::new(Mutex::new(stdin)),
        };
        let alive_thread = Arc::new(AtomicBool::new(true));

        Self::spawn_thread(stdout, alive_thread.clone());

        Ok((Self { alive_thread }, ipc_input))
    }

    fn spawn_thread(stdout: ChildStdout, alive_thread: Arc<AtomicBool>) {
        std::thread::spawn(move || {
            let mut reader = BufReader::new(stdout);

            while alive_thread.load(Ordering::SeqCst) {
                sleep(Duration::from_millis(16));
                let mut line = String::new();

                if reader.read_line(&mut line).is_err() {
                    break;
                }

                if line.is_empty() {
                    continue;
                }

                match serde_json::from_str::<ProtocolOut>(&line) {
                    Ok(event) => {
                        println!("{event:?}");
                    }
                    Err(err) => {
                        eprintln!("JSON inválido: {err}");
                    }
                }
            }
        });
    }
}

#[derive(Clone)]
pub struct IpcInput {
    stdin: Arc<Mutex<ChildStdin>>,
}

impl IpcInput {
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
