use std::time::Duration;
use tauri::Manager;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::TrayIconBuilder;
use tauri_plugin_autostart::MacosLauncher;

const OLLAMA_PORT: u16 = 11434;
const JARVIS_PORT: u16 = 8000;

// ---------------------------------------------------------------------------
// Setup status — reported to the SetupScreen on first launch
// ---------------------------------------------------------------------------

#[derive(serde::Serialize, Clone)]
struct SetupStatus {
    phase: String,
    detail: String,
    ollama_ready: bool,
    model_ready: bool,
    server_ready: bool,
    error: Option<String>,
}

/// Check whether at least one model is loaded in Ollama.
async fn ollama_has_model(client: &reqwest::Client) -> bool {
    let url = format!("http://127.0.0.1:{}/api/tags", OLLAMA_PORT);
    if let Ok(resp) = client.get(&url).send().await {
        if let Ok(body) = resp.json::<serde_json::Value>().await {
            return body["models"]
                .as_array()
                .map(|m| !m.is_empty())
                .unwrap_or(false);
        }
    }
    false
}

#[tauri::command]
async fn get_setup_status() -> SetupStatus {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap_or_else(|_| reqwest::Client::new());

    let ollama_ready = client
        .get(format!("http://127.0.0.1:{}/api/tags", OLLAMA_PORT))
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false);

    let model_ready = if ollama_ready {
        ollama_has_model(&client).await
    } else {
        false
    };

    let server_ready = client
        .get(format!("http://127.0.0.1:{}/health", JARVIS_PORT))
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false);

    let (phase, detail) = if ollama_ready && model_ready && server_ready {
        ("ready".to_string(), "All systems operational".to_string())
    } else if !ollama_ready {
        ("starting".to_string(), "Waiting for Ollama...".to_string())
    } else if !model_ready {
        ("starting".to_string(), "Loading AI model...".to_string())
    } else {
        ("starting".to_string(), "Waiting for API server...".to_string())
    };

    SetupStatus {
        phase,
        detail,
        ollama_ready,
        model_ready,
        server_ready,
        error: None,
    }
}

#[tauri::command]
fn get_api_base() -> String {
    format!("http://127.0.0.1:{}", JARVIS_PORT)
}

/// Fetch health status from the OpenJarvis API server.
#[tauri::command]
async fn check_health(api_url: String) -> Result<serde_json::Value, String> {
    let url = format!("{}/health", api_url);
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;
    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Invalid response: {}", e))?;
    Ok(body)
}

/// Fetch energy monitoring data from the API.
#[tauri::command]
async fn fetch_energy(api_url: String) -> Result<serde_json::Value, String> {
    let url = format!("{}/v1/telemetry/energy", api_url);
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;
    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Invalid response: {}", e))?;
    Ok(body)
}

/// Fetch telemetry statistics from the API.
#[tauri::command]
async fn fetch_telemetry(api_url: String) -> Result<serde_json::Value, String> {
    let url = format!("{}/v1/telemetry/stats", api_url);
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;
    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Invalid response: {}", e))?;
    Ok(body)
}

/// Fetch recent traces from the API.
#[tauri::command]
async fn fetch_traces(api_url: String, limit: u32) -> Result<serde_json::Value, String> {
    let url = format!("{}/v1/traces?limit={}", api_url, limit);
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;
    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Invalid response: {}", e))?;
    Ok(body)
}

/// Fetch a single trace by ID.
#[tauri::command]
async fn fetch_trace(api_url: String, trace_id: String) -> Result<serde_json::Value, String> {
    let url = format!("{}/v1/traces/{}", api_url, trace_id);
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;
    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Invalid response: {}", e))?;
    Ok(body)
}

/// Fetch learning system statistics.
#[tauri::command]
async fn fetch_learning_stats(api_url: String) -> Result<serde_json::Value, String> {
    let url = format!("{}/v1/learning/stats", api_url);
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;
    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Invalid response: {}", e))?;
    Ok(body)
}

/// Fetch learning policy configuration.
#[tauri::command]
async fn fetch_learning_policy(api_url: String) -> Result<serde_json::Value, String> {
    let url = format!("{}/v1/learning/policy", api_url);
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;
    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Invalid response: {}", e))?;
    Ok(body)
}

/// Fetch memory backend statistics.
#[tauri::command]
async fn fetch_memory_stats(api_url: String) -> Result<serde_json::Value, String> {
    let url = format!("{}/v1/memory/stats", api_url);
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;
    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Invalid response: {}", e))?;
    Ok(body)
}

/// Search memory for relevant chunks.
#[tauri::command]
async fn search_memory(
    api_url: String,
    query: String,
    top_k: u32,
) -> Result<serde_json::Value, String> {
    let url = format!("{}/v1/memory/search", api_url);
    let client = reqwest::Client::new();
    let resp = client
        .post(&url)
        .json(&serde_json::json!({"query": query, "top_k": top_k}))
        .send()
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;
    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Invalid response: {}", e))?;
    Ok(body)
}

/// Fetch list of available agents.
#[tauri::command]
async fn fetch_agents(api_url: String) -> Result<serde_json::Value, String> {
    let url = format!("{}/v1/agents", api_url);
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;
    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Invalid response: {}", e))?;
    Ok(body)
}

/// Fetch available models from the API server.
#[tauri::command]
async fn fetch_models(api_url: String) -> Result<serde_json::Value, String> {
    let url = format!("{}/v1/models", api_url);
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;
    resp.json().await.map_err(|e| format!("Invalid response: {}", e))
}

/// Pull a model via Ollama.
#[tauri::command]
async fn pull_ollama_model(model_name: String) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(600))
        .build()
        .map_err(|e| e.to_string())?;
    let resp = client
        .post(format!("http://127.0.0.1:{}/api/pull", OLLAMA_PORT))
        .json(&serde_json::json!({ "name": model_name, "stream": false }))
        .send()
        .await
        .map_err(|e| format!("Failed to pull model: {}", e))?;
    if resp.status().is_success() {
        Ok(serde_json::json!({ "status": "ok", "model": model_name }))
    } else {
        Err(format!("Pull failed with status: {}", resp.status()))
    }
}

/// Delete a model from Ollama.
#[tauri::command]
async fn delete_ollama_model(model_name: String) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| e.to_string())?;
    let resp = client
        .delete(format!("http://127.0.0.1:{}/api/delete", OLLAMA_PORT))
        .json(&serde_json::json!({ "name": model_name }))
        .send()
        .await
        .map_err(|e| format!("Failed to delete model: {}", e))?;
    if resp.status().is_success() {
        Ok(serde_json::json!({ "status": "ok", "model": model_name }))
    } else {
        Err(format!("Delete failed with status: {}", resp.status()))
    }
}

/// Transcribe audio via the speech API endpoint.
#[tauri::command]
async fn transcribe_audio(
    api_url: String,
    audio_data: Vec<u8>,
    filename: String,
) -> Result<serde_json::Value, String> {
    let url = format!("{}/v1/speech/transcribe", api_url);
    let part = reqwest::multipart::Part::bytes(audio_data)
        .file_name(filename)
        .mime_str("audio/webm")
        .map_err(|e| format!("Multipart error: {}", e))?;
    let form = reqwest::multipart::Form::new().part("file", part);
    let client = reqwest::Client::new();
    let resp = client
        .post(&url)
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;
    resp.json().await.map_err(|e| format!("Invalid response: {}", e))
}

/// Check speech backend health.
#[tauri::command]
async fn speech_health(api_url: String) -> Result<serde_json::Value, String> {
    let url = format!("{}/v1/speech/health", api_url);
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;
    resp.json().await.map_err(|e| format!("Invalid response: {}", e))
}

/// Launch the `jarvis` CLI command via shell.
#[tauri::command]
async fn run_jarvis_command(args: Vec<String>) -> Result<String, String> {
    let output = tokio::process::Command::new("jarvis")
        .args(&args)
        .output()
        .await
        .map_err(|e| format!("Failed to launch jarvis: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--hidden"]),
        ))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // Focus the main window if another instance is launched
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }
        }))
        .setup(|app| {
            let show = MenuItemBuilder::with_id("show", "Show / Hide")
                .build(app)?;
            let health = MenuItemBuilder::with_id("health", "Health: checking...")
                .enabled(false)
                .build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "Quit OpenJarvis")
                .build(app)?;

            let menu = MenuBuilder::new(app)
                .item(&show)
                .separator()
                .item(&health)
                .separator()
                .item(&quit)
                .build()?;

            let _tray = TrayIconBuilder::with_id("main")
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("OpenJarvis")
                .menu(&menu)
                .on_menu_event(move |app, event| {
                    match event.id().as_ref() {
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                if window.is_visible().unwrap_or(false) {
                                    let _ = window.hide();
                                } else {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_setup_status,
            get_api_base,
            check_health,
            fetch_energy,
            fetch_telemetry,
            fetch_traces,
            fetch_trace,
            fetch_learning_stats,
            fetch_learning_policy,
            fetch_memory_stats,
            search_memory,
            fetch_agents,
            fetch_models,
            pull_ollama_model,
            delete_ollama_model,
            transcribe_audio,
            speech_health,
            run_jarvis_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running OpenJarvis Desktop");
}
