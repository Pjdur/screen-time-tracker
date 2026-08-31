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

function renderHistory(history: DayStat[]) {
  const tbody = document.getElementById("history-body");
  if (!tbody) return;

  if (history.length === 0) {
    tbody.innerHTML = `<tr><td colspan="2" style="color: #888;">No past history yet</td></tr>`;
    return;
  }

  // Show newest dates at the top
  const rowsHtml = [...history]
    .reverse()
    .map(
      (day) => `
      <tr>
        <td>${day.date}</td>
        <td>${formatTime(day.total_seconds)}</td>
      </tr>
    `
    )
    .join("");

  tbody.innerHTML = rowsHtml;
}

async function initTracker() {
  const displayEl = document.getElementById("time-display");
  if (!displayEl) return;

  try {
    const initialStats = await invoke<AppStats>("get_stats");
    displayEl.textContent = formatTime(initialStats.total_seconds);
    renderHistory(initialStats.history);
  } catch (error) {
    console.error("Failed to fetch initial stats:", error);
    displayEl.textContent = "Error Loading Time";
    return;
  }

  setInterval(async () => {
    try {
      const updatedStats = await invoke<AppStats>("tick");
      displayEl.textContent = formatTime(updatedStats.total_seconds);
      // Re-render history only if midnight reset happened
      renderHistory(updatedStats.history);
    } catch (error) {
      console.error("Failed to tick active session:", error);
    }
  }, 1000);
}

window.addEventListener("DOMContentLoaded", initTracker);
