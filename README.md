# Odysseus Desktop

A polished native desktop app for [Odysseus](https://github.com/pewdiepie-archdaemon/odysseus) — the self-hosted AI workspace created by Felix Kjellberg (PewDiePie).

Built with [Tauri](https://tauri.app). Lightweight, no Electron, cross-platform.

> This application is not affiliated with, endorsed by, or associated with Odysseus created by Felix Kjellberg (PewDiePie), however this application is heavily inspired by Odysseus and meant to work in tandem with it.

---

## Download

Go to the [Releases](https://github.com/KrusiDev/odysseus-desktop/releases) page and download the installer for your platform.

| Platform | File |
|----------|------|
| Windows | `Odysseus_x.x.x_x64-setup.exe` or `.msi` |
| macOS | `Odysseus_x.x.x_x64.dmg` |
| Ubuntu / Debian | `Odysseus_x.x.x_amd64.deb` |
| Fedora / Red Hat | `Odysseus-x.x.x-1.x86_64.rpm` |
| Other Linux | `Odysseus_x.x.x_amd64.AppImage` |

> **Note:** Windows and macOS may show a security warning since the app is not code signed. On Windows click "More info" → "Run anyway". On macOS right-click the app → Open.
>
> **Linux note:** Ubuntu and Debian users must use the `.deb` installer. The `.AppImage` is known to have rendering issues on Ubuntu. macOS stability has not been fully tested yet.
>
> **Linux keyboard shortcuts:** Global keyboard shortcuts (Ctrl+/-, F11) may not work on all Linux desktop environments. Use the system tray icon instead.

---

## Requirements

You need a running Odysseus instance. Odysseus Desktop is a launcher — it connects to your own self-hosted server.

Follow [the Odysseus setup guide](https://github.com/pewdiepie-archdaemon/odysseus) to get Odysseus running first.

---

## Features

- **Animated splash screen** with constellation background
- **Smart setup form** — protocol dropdown, address field, optional port
  - Tooltips on every field explaining what to enter
  - OS-aware IP address discovery instructions
- **Silent connection probing** — no jarring browser error pages
- **Connection resilience**
  - Health checks every 60 seconds detect if your server goes down
  - Automatic reconnect screen with one-click retry
  - Distinct messages for wrong address vs lost connection
  - Works behind Nginx Proxy Manager (502/503 detection)
- **System tray** — always accessible regardless of what's open in the app
  - Zoom In / Zoom Out / Reset Zoom
  - Disconnect
  - Quit
- **Global keyboard shortcuts**
  - `Ctrl` + `=` — Zoom in
  - `Ctrl` + `-` — Zoom out
  - `Ctrl` + `0` — Reset zoom
  - `F11` — Toggle fullscreen

---

## Setup

1. Download and run the installer for your platform
2. Click **Ready to set sail?** on the splash screen
3. Select your protocol (`http` for most setups, `https` if behind a reverse proxy with SSL)
4. Enter the address of your Odysseus instance (e.g. `localhost` or `192.168.1.10`)
5. Enter the port if connecting directly (default: `7000`). Leave blank if using a reverse proxy
6. Click **Set sail**

Your address is saved — future launches connect automatically without showing the setup screen.

---

## Build from source

Requirements: [Node.js](https://nodejs.org), [Rust](https://rustup.rs), and the [Tauri prerequisites](https://tauri.app/start/prerequisites/).

```bash
git clone https://github.com/KrusiDev/odysseus-desktop.git
cd odysseus-desktop
npm install
npm run tauri dev     # development
npm run tauri build   # production build
```

---

## License

MIT — see [LICENSE](./LICENSE).
