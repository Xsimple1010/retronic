use crate::ipc::protocol::out::ProtocolOut;
use std::io::{BufRead, BufReader, Write};
use std::process::{ChildStdout, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use crate::ipc::send_input::IpcInput;

pub mod protocol;
pub mod send_input;

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

        let ipc_input = IpcInput::new(stdin);
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
