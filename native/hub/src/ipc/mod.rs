use crate::ipc::receive_output::receive_output;
use crate::ipc::send_input::IpcInput;
use std::io::{BufRead, BufReader};
use std::process::{ChildStdout, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use tinic_ipc_protocol::out::ProtocolOut;

mod receive_output;
pub mod send_input;

pub struct Ipc {
    _alive_thread: Arc<AtomicBool>,
    pub input: IpcInput,
}

impl Drop for Ipc {
    fn drop(&mut self) {
        self._alive_thread.store(false, Ordering::SeqCst);
    }
}

impl Ipc {
    pub fn new() -> Self {
        Self {
            _alive_thread: Arc::new(AtomicBool::new(false)),
            input: IpcInput::default(),
        }
    }

    pub fn start(&self, path: String) -> Result<(), String> {
        let mut child = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            // .stderr(Stdio::inherit()) // logs aparecem no console
            .spawn()
            .map_err(|e| e.to_string())?;

        let stdin = child.stdin.take().ok_or("stdin não disponível")?;
        let stdout = child.stdout.take().ok_or("stdout não disponível")?;

        self.input.set_stdin(stdin);
        self.spawn_thread(stdout);

        Ok(())
    }

    fn spawn_thread(&self, stdout: ChildStdout) {
        self._alive_thread.store(true, Ordering::SeqCst);
        let alive_thread = self._alive_thread.clone();

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
                        receive_output(event);
                    }
                    Err(err) => {
                        eprintln!("JSON inválido: {err}");
                    }
                }
            }
        });
    }
}
