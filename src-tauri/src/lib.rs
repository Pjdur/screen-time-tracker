use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tauri::Manager;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DayStat {
    pub date: String,
    pub total_seconds: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppStats {
    pub date: String,
    pub total_seconds: u64,
    pub history: Vec<DayStat>,
}

fn get_stats_file_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("stats.json"))
}

fn check_day_reset(stats: &mut AppStats, today: &str) {
    if stats.date != today {
        if stats.total_seconds > 0 {
            stats.history.push(DayStat {
                date: stats.date.clone(),
                total_seconds: stats.total_seconds,
            });
        }
        stats.date = today.to_string();
        stats.total_seconds = 0;
    }
}

#[tauri::command]
fn get_stats(app: tauri::AppHandle) -> Result<AppStats, String> {
    let path = get_stats_file_path(&app)?;
    let backup_path = path.with_extension("bak");
    let today = Local::now().format("%Y-%m-%d").to_string();

    let content = fs::read_to_string(&path)
        .or_else(|_| fs::read_to_string(&backup_path));

    if let Ok(data) = content {
        if let Ok(mut stats) = serde_json::from_str::<AppStats>(&data) {
            check_day_reset(&mut stats, &today);
            return Ok(stats);
        }
    }

    Ok(AppStats {
        date: today,
        total_seconds: 0,
        history: Vec::new(),
    })
}

#[tauri::command]
fn tick(app: tauri::AppHandle) -> Result<AppStats, String> {
    let path = get_stats_file_path(&app)?;
    let temp_path = path.with_extension("tmp");
    let backup_path = path.with_extension("bak");
    let today = Local::now().format("%Y-%m-%d").to_string();

    let mut stats = get_stats(app.clone())?;
    check_day_reset(&mut stats, &today);
    stats.total_seconds += 1;

    let json = serde_json::to_string_pretty(&stats).map_err(|e| e.to_string())?;

    if path.exists() {
        let _ = fs::copy(&path, &backup_path);
    }

    let mut file = File::create(&temp_path).map_err(|e| e.to_string())?;
    file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
    
    file.sync_all().map_err(|e| e.to_string())?;

    fs::rename(temp_path, &path).map_err(|e| e.to_string())?;

    Ok(stats)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_stats, tick])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
