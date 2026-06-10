use std::sync::atomic::{AtomicBool, Ordering};

static GALLERY_RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(serde::Serialize)]
pub struct GalleryInfo {
    pub url: String,
    pub qr: String,
}

#[tauri::command]
pub fn host_gallery_start() -> Result<GalleryInfo, String> {
    if GALLERY_RUNNING.swap(true, Ordering::SeqCst) {
        return Err("Gallery already running".into());
    }

    // TODO: mount host volumes read-only + axum file server
    let url = "http://192.168.1.1:8765/gallery".to_string();
    let qr = "▄▄▄▄▄▄▄\n█ ▄▄▄ █\n█ ███ █\n▀▀▀▀▀▀▀".to_string();
    Ok(GalleryInfo { url, qr })
}

#[tauri::command]
pub fn host_gallery_stop() -> Result<(), String> {
    GALLERY_RUNNING.store(false, Ordering::SeqCst);
    Ok(())
}
