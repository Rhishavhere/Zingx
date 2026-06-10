use crate::paths::ollama_host;
use anyhow::Result;

#[tauri::command]
pub async fn zingx_ask(prompt: String) -> Result<String, String> {
    let model = std::env::var("ZINGX_MODEL").unwrap_or_else(|_| "gemma3:1b".into());
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "model": model,
        "prompt": prompt,
        "stream": false
    });

    let resp = client
        .post(format!("{}/api/generate", ollama_host()))
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Ollama unavailable: {e}"))?;

    let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    json.get("response")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .ok_or_else(|| "empty response".into())
}

pub async fn generate_stream(prompt: &str) -> Result<String, String> {
    let model = std::env::var("ZINGX_MODEL").unwrap_or_else(|_| "gemma3:1b".into());
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "model": model,
        "prompt": prompt,
        "stream": true
    });

    let mut resp = client
        .post(format!("{}/api/generate", ollama_host()))
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Ollama unavailable: {e}"))?;

    let mut out = String::new();
    while let Some(chunk) = resp
        .chunk()
        .await
        .map_err(|e| e.to_string())?
    {
        if let Ok(v) = serde_json::from_slice::<serde_json::Value>(&chunk) {
            if let Some(p) = v.get("response").and_then(|x| x.as_str()) {
                out.push_str(p);
            }
        }
    }
    Ok(out)
}
