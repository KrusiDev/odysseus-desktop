# Odysseus Desktop

A native desktop app wrapper for [Odysseus](https://github.com/pewdiepie-archdaemon/odysseus) — the self-hosted AI workspace by PewDiePie (Felix Kjellberg).

Built with [Tauri](https://tauri.app). Lightweight, no Electron, cross-platform.

---

## Download

Go to the [Releases](https://github.com/KrusiDev/odysseus-desktop/releases) page and download the installer for your platform.

| Platform | File |
|----------|------|
| Windows | `.msi` or `.exe` |
| macOS | `.dmg` (coming soon) |
| Linux | `.AppImage` or `.deb` (coming soon) |

---

## Requirements

You need a running Odysseus instance. The desktop app is just a wrapper — it connects to your own server.

Follow [PewDiePie's setup guide](https://github.com/pewdiepie-archdaemon/odysseus) to get Odysseus running first.

---

## Setup

1. Download and run the installer
2. Enter the address of your Odysseus instance (e.g. `http://localhost:7000`)
3. Click **Set sail**

Your address is saved locally — you won't need to enter it again on future launches.

---

## Build from source

Requirements: [Node.js](https://nodejs.org), [Rust](https://rustup.rs), and the [Tauri CLI](https://tauri.app/start/prerequisites/).

```bash
git clone https://github.com/KrusiDev/odysseus-desktop.git
cd odysseus-desktop
npm install
npm run tauri build
```

---

## License

MIT — see [LICENSE](./LICENSE).

This project is not affiliated with PewDiePie or Odysseus. It is an independent community wrapper.
