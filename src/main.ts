import { invoke } from "@tauri-apps/api/core";

interface AppStats {
  date: string;
  total_seconds: number;
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
    // 1. Fetch saved initial stats from Rust backend
    const stats = await invoke<AppStats>("get_stats");
    let currentSeconds = stats.total_seconds;

    displayEl.textContent = formatTime(currentSeconds);

    // 2. Start live counter tick on the frontend
    setInterval(() => {
      currentSeconds += 1;
      displayEl.textContent = formatTime(currentSeconds);
    }, 1000);
  } catch (error) {
    console.error("Failed to load screen time stats:", error);
    displayEl.textContent = "Error Loading Time";
  }
}

window.addEventListener("DOMContentLoaded", initTracker);
