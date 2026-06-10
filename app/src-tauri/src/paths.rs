use std::path::PathBuf;

pub fn data_dir() -> PathBuf {
    std::env::var("ZINGX_DATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("./dev-data"))
}

pub fn vault_dir() -> PathBuf {
    std::env::var("ZINGX_VAULT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("./dev-vault"))
}

pub fn chats_dir() -> PathBuf {
    data_dir().join("chats")
}

pub fn ollama_host() -> String {
    std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://127.0.0.1:11434".into())
}

pub fn ensure_dev_dirs(app: &tauri::AppHandle) {
    let _ = app;
    let _ = std::fs::create_dir_all(chats_dir());
    let _ = std::fs::create_dir_all(data_dir().join("notes"));
    let _ = std::fs::create_dir_all(vault_dir().join("browser"));
}

#[derive(serde::Serialize)]
pub struct UtilsStatus {
    pub ollama: bool,
    pub data: String,
    pub vault: String,
}

#[tauri::command]
pub async fn utils_status() -> UtilsStatus {
    let ollama = reqwest::Client::new()
        .get(format!("{}/api/tags", ollama_host()))
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false);

    UtilsStatus {
        ollama,
        data: data_dir().display().to_string(),
        vault: vault_dir().display().to_string(),
    }
}
