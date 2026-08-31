import { invoke } from "@tauri-apps/api/core";

interface DayStat {
  date: string;
  total_seconds: number;
}

interface AppStats {
  date: string;
  total_seconds: number;
  history: DayStat[];
}

function formatTime(totalSeconds: number): string {
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;

  return [hours, minutes, seconds]
    .map((v) => v.toString().padStart(2, "0"))
    .join(":");
}

async function initTracker() {
  const displayEl = document.getElementById("time-display");
  if (!displayEl) return;

  try {
    // Initial fetch to render stored time immediately on launch
    const initialStats = await invoke<AppStats>("get_stats");
    displayEl.textContent = formatTime(initialStats.total_seconds);
  } catch (error) {
    console.error("Failed to fetch initial stats:", error);
    displayEl.textContent = "Error Loading Time";
    return;
  }

  // Active continuous tick: increments Rust state and saves atomically to disk
  setInterval(async () => {
    try {
      const updatedStats = await invoke<AppStats>("tick");
      displayEl.textContent = formatTime(updatedStats.total_seconds);
    } catch (error) {
      console.error("Failed to tick active session:", error);
    }
  }, 1000);
}

window.addEventListener("DOMContentLoaded", initTracker);
