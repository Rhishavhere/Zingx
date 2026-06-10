mod chat;
mod ollama;
mod paths;
mod pty;
mod host;

use pty::PtyManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(PtyManager(parking_lot::Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            pty::pty_spawn,
            pty::pty_write,
            ollama::zingx_ask,
            chat::chat_list,
            chat::chat_create,
            chat::chat_load,
            chat::chat_send,
            paths::utils_status,
            host::host_gallery_start,
            host::host_gallery_stop,
        ])
        .setup(|app| {
            paths::ensure_dev_dirs(app.handle());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
