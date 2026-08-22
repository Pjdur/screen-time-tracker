import { invoke } from "@tauri-apps/api/core";

// Define the shape of the data coming from Rust
interface AppStats {
  date: string;
  total_seconds: number;
}

async function showScreenTime() {
  try {
    // Call the Rust command
    const stats = await invoke<AppStats>("get_stats");
    
    // Calculate hours and minutes
    const hours = Math.floor(stats.total_seconds / 3600);
    const minutes = Math.floor((stats.total_seconds % 3600) / 60);
    
    const display = document.getElementById("time-display");
    if (display) {
      display.innerText = `Today's Screen Time: ${hours}h ${minutes}m`;
    }

    // Visual warning if over 10 hours (36000 seconds)
    if (stats.total_seconds > 36000) {
      document.body.style.backgroundColor = "#ffcccc"; // Light red
      document.body.style.color = "#550000";
    }
  } catch (e) {
    console.error("Failed to load stats:", e);
    const display = document.getElementById("time-display");
    if (display) display.innerText = "Error loading stats.";
  }
}
   
