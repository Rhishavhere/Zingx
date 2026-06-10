use std::io::{Read, Write};
use std::sync::Arc;
use std::thread;

use parking_lot::Mutex;
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use tauri::{AppHandle, Emitter, State};

pub struct PtyState {
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    line_buf: Arc<Mutex<String>>,
}

impl Clone for PtyState {
    fn clone(&self) -> Self {
        Self {
            writer: self.writer.clone(),
            line_buf: self.line_buf.clone(),
        }
    }
}

pub struct PtyManager(pub Mutex<Option<PtyState>>);

#[tauri::command]
pub fn pty_spawn(app: AppHandle, manager: State<PtyManager>) -> Result<(), String> {
    let mut guard = manager.0.lock();
    if guard.is_some() {
        return Ok(());
    }

    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| e.to_string())?;

    let shell = if cfg!(windows) {
        CommandBuilder::new("powershell.exe")
    } else {
        let mut cmd = CommandBuilder::new("/bin/bash");
        cmd.args(["--login"]);
        cmd
    };

    let mut child = pair.slave.spawn_command(shell).map_err(|e| e.to_string())?;
    drop(pair.slave);

    let mut reader = pair.master.try_clone_reader().map_err(|e| e.to_string())?;
    let writer = pair.master.take_writer().map_err(|e| e.to_string())?;
    let writer = Arc::new(Mutex::new(writer));

    let app_out = app.clone();
    thread::spawn(move || {
        let mut buf = [0u8; 4096];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    let s = String::from_utf8_lossy(&buf[..n]).to_string();
                    let _ = app_out.emit("pty-output", s);
                }
                Err(_) => break,
            }
        }
        let _ = child.wait();
        let _ = app_out.emit("pty-output", "\r\n[pty exited]\r\n");
    });

    *guard = Some(PtyState {
        writer,
        line_buf: Arc::new(Mutex::new(String::new())),
    });
    Ok(())
}

#[tauri::command]
pub async fn pty_write(
    app: AppHandle,
    manager: State<'_, PtyManager>,
    data: String,
) -> Result<(), String> {
    let state = {
        let guard = manager.0.lock();
        guard.as_ref().cloned()
    };

    let Some(state) = state else {
        return Err("PTY not spawned".into());
    };

    for ch in data.chars() {
        if ch == '\r' || ch == '\n' {
            let line = state.line_buf.lock().trim().to_string();
            state.line_buf.lock().clear();

            if line.starts_with("@zingx ") {
                let prompt = line[7..].trim().to_string();
                let _ = app.emit("pty-output", "\r\n");
                match crate::ollama::zingx_ask(prompt).await {
                    Ok(resp) => {
                        for l in resp.lines() {
                            let _ = app.emit("pty-output", format!("\x1b[90m{l}\x1b[0m\r\n"));
                        }
                    }
                    Err(e) => {
                        let _ = app.emit("pty-output", format!("\x1b[31m{e}\x1b[0m\r\n"));
                    }
                }
                continue;
            }

            state
                .writer
                .lock()
                .write_all(line.as_bytes())
                .map_err(|e| e.to_string())?;
            state
                .writer
                .lock()
                .write_all(b"\n")
                .map_err(|e| e.to_string())?;
        } else if ch == '\x7f' || ch == '\u{8}' {
            let mut lb = state.line_buf.lock();
            if !lb.is_empty() {
                lb.pop();
                state
                    .writer
                    .lock()
                    .write_all(&[0x08, b' ', 0x08])
                    .map_err(|e| e.to_string())?;
            }
        } else {
            state.line_buf.lock().push(ch);
            state
                .writer
                .lock()
                .write_all(&[ch as u8])
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
