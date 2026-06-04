import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebview } from "@tauri-apps/api/webview";

window.addEventListener("DOMContentLoaded", () => {
  const protocolSelect = document.getElementById("protocol-select") as HTMLSelectElement;
  const hostInput = document.getElementById("host-input") as HTMLInputElement;
  const portInput = document.getElementById("port-input") as HTMLInputElement;
  const btn = document.getElementById("connect-btn") as HTMLButtonElement;

  if (!protocolSelect || !hostInput || !portInput || !btn) return;

  hostInput.focus();

  function buildUrl(): string {
    const protocol = protocolSelect.value;
    const host = hostInput.value.trim();
    const port = portInput.value.trim();
    return port ? `${protocol}://${host}:${port}` : `${protocol}://${host}`;
  }

  function update() {
    const host = hostInput.value.trim();
    btn.disabled = !host;
  }

  protocolSelect.addEventListener("change", update);
  hostInput.addEventListener("input", update);
  portInput.addEventListener("input", update);

  btn.addEventListener("click", navigate);
  hostInput.addEventListener("keydown", (e) => { if (e.key === "Enter" && !btn.disabled) navigate(); });
  portInput.addEventListener("keydown", (e) => { if (e.key === "Enter" && !btn.disabled) navigate(); });

  async function navigate() {
    await invoke("save_and_navigate", { url: buildUrl() });
  }

  update();
});

const webview = getCurrentWebview();
let zoom = 1.0;


// Zoom via Tauri's built-in setZoom API (Ctrl+/-, Ctrl+0)
window.addEventListener("keydown", async (e) => {
  if (!e.ctrlKey) return;
  if (e.key === "+" || e.key === "=") {
    e.preventDefault();
    zoom = Math.min(zoom + 0.1, 3.0);
    await webview.setZoom(zoom);
  } else if (e.key === "-") {
    e.preventDefault();
    zoom = Math.max(zoom - 0.1, 0.3);
    await webview.setZoom(zoom);
  } else if (e.key === "0") {
    e.preventDefault();
    zoom = 1.0;
    await webview.setZoom(zoom);
  }
});
