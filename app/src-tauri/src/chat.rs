use std::fs;
use std::path::PathBuf;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ollama::generate_stream;
use crate::paths::chats_dir;

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatMeta {
    pub id: String,
    pub title: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct ChatFile {
    id: String,
    title: String,
    updated_at: String,
    messages: Vec<ChatMessage>,
}

#[derive(Serialize, Deserialize, Clone)]
struct ChatMessage {
    role: String,
    content: String,
}

fn chat_path(id: &str) -> PathBuf {
    chats_dir().join(format!("{id}.json"))
}

#[tauri::command]
pub fn chat_list() -> Result<Vec<ChatMeta>, String> {
    let dir = chats_dir();
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let mut items = Vec::new();
    for entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let chat: ChatFile = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
        items.push(ChatMeta {
            id: chat.id,
            title: chat.title,
            updated_at: chat.updated_at,
        });
    }
    items.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(items)
}

#[tauri::command]
pub fn chat_create(title: String) -> Result<String, String> {
    let id = Uuid::new_v4().to_string();
    let chat = ChatFile {
        id: id.clone(),
        title,
        updated_at: Utc::now().to_rfc3339(),
        messages: vec![],
    };
    fs::create_dir_all(chats_dir()).map_err(|e| e.to_string())?;
    fs::write(
        chat_path(&id),
        serde_json::to_string_pretty(&chat).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    Ok(id)
}

#[derive(Serialize, Clone)]
pub struct ChatMessageOut {
    pub role: String,
    pub content: String,
}

#[tauri::command]
pub fn chat_load(id: String) -> Result<Vec<ChatMessageOut>, String> {
    let raw = fs::read_to_string(chat_path(&id)).map_err(|e| e.to_string())?;
    let chat: ChatFile = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    Ok(chat
        .messages
        .into_iter()
        .map(|m| ChatMessageOut {
            role: m.role,
            content: m.content,
        })
        .collect())
}

#[tauri::command]
pub async fn chat_send(id: String, prompt: String) -> Result<String, String> {
    let path = chat_path(&id);
    let raw = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let mut chat: ChatFile = serde_json::from_str(&raw).map_err(|e| e.to_string())?;

    chat.messages.push(ChatMessage {
        role: "user".into(),
        content: prompt.clone(),
    });

    if chat.title == "new chat" {
        chat.title = prompt.chars().take(40).collect();
    }

    let reply = generate_stream(&prompt).await?;
    chat.messages.push(ChatMessage {
        role: "assistant".into(),
        content: reply.clone(),
    });
    chat.updated_at = Utc::now().to_rfc3339();

    fs::write(
        &path,
        serde_json::to_string_pretty(&chat).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;

    Ok(reply)
}
