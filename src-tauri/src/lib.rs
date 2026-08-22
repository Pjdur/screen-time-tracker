use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::sync::Mutex;
use tauri::{Manager, RunEvent};

// 1. Data Structure for our JSON file
#[derive(Serialize, Deserialize, Debug, Clone)]
struct AppStats {
    date: String,
    total_seconds: u64,
}

// 2. Shared State to track when the app started
struct TimerState {
    start_time: Mutex<chrono::DateTime<Local>>,
}

#[tauri::command]
fn get_stats(app: tauri::AppHandle) -> Result<AppStats, String> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("stats.json");

    let today = Local::now().format("%Y-%m-%d").to_string();

    // Load existing data or create new
    let mut stats: AppStats = if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let mut s: AppStats = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        
        // RESET LOGIC: If the saved date is not today, reset counter
        if s.date != today {
            s.date = today;
            s.total_seconds = 0;
        }
        s
    } else {
        AppStats { date: today, total_seconds: 0 }
    };

    Ok(stats)
}

fn save_stats(app: &tauri::AppHandle, seconds_to_add: u64) -> Result<(), String> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("stats.json");

    let today = Local::now().format("%Y-%m-%d").to_string();
    
    // Load current stats
    let mut stats: AppStats = if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let mut s: AppStats = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        if s.date != today {
            s.date = today;
            s.total_seconds = 0;
        }
        s
    } else {
        AppStats { date: today, total_seconds: 0 }
    };

    // Add the session time
    stats.total_seconds += seconds_to_add;

    // Write to file
    let json = serde_json::to_string_pretty(&stats).map_err(|e| e.to_string())?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .map_err(|e| e.to_string())?;
    
    file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(TimerState {
            start_time: Mutex::new(Local::now()),
        })
        .invoke_handler(tauri::generate_handler![get_stats])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| {
            match event {
                // 3. Capture Exit Event to calculate duration
                RunEvent::Exit => {
                    let state = app.state::<TimerState>();
                    let start = *state.start_time.lock().unwrap();
                    let duration = Local::now().signed_duration_since(start).num_seconds();
                    
                    if duration > 0 {
                        let _ = save_stats(&app, duration as u64);
                        println!("Session saved: {} seconds", duration);
                    }
                }
                _ => {}
            }
        });
}   
