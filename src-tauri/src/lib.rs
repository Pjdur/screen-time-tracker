use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use tauri::Manager;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppStats {
    pub date: String,
    pub total_seconds: u64,
}

// Helper to ensure app data directory exists and return file path
fn get_stats_file_path(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("stats.json"))
}

#[tauri::command]
fn get_stats(app: tauri::AppHandle) -> Result<AppStats, String> {
    let path = get_stats_file_path(&app)?;
    let today = Local::now().format("%Y-%m-%d").to_string();

    if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        if let Ok(mut stats) = serde_json::from_str::<AppStats>(&content) {
            // Reset counter if midnight has passed
            if stats.date != today {
                stats.date = today;
                stats.total_seconds = 0;
            }
            return Ok(stats);
        }
    }

    Ok(AppStats {
        date: today,
        total_seconds: 0,
    })
}

#[tauri::command]
fn tick(app: tauri::AppHandle) -> Result<AppStats, String> {
    let path = get_stats_file_path(&app)?;
    let today = Local::now().format("%Y-%m-%d").to_string();

    let mut stats = get_stats(app.clone())?;

    // Check date rollover during an active session
    if stats.date != today {
        stats.date = today;
        stats.total_seconds = 0;
    }

    stats.total_seconds += 1;

    // Persist immediately on every tick
    let json = serde_json::to_string_pretty(&stats).map_err(|e| e.to_string())?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .map_err(|e| e.to_string())?;

    file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;

    Ok(stats)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_stats, tick])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
