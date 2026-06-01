console.log('main.ts loaded');
import { invoke } from "@tauri-apps/api/core";

window.addEventListener("DOMContentLoaded", () => {
  const input = document.getElementById("url-input") as HTMLInputElement;
  const btn = document.getElementById("connect-btn");

  if (!input || !btn) return;

  input.focus();

  btn.addEventListener("click", navigate);
  input.addEventListener("keydown", (e) => {
    if (e.key === "Enter") navigate();
  });

  async function navigate() {
    let url = input.value.trim();
    if (!url) return;
    if (!url.startsWith("http")) url = "http://" + url;
    await invoke("save_and_navigate", { url });
  }
});