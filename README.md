<div align="center">

# Rathole Launcher

The friendliest way to run [rathole](https://github.com/rathole-org/rathole) — a desktop app that turns reverse-proxy tunnels into a few clicks, on macOS, Windows and Linux.

**English** · [简体中文](README.zh-CN.md) · [日本語](README.ja.md) · [한국어](README.ko.md)

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)](../../releases)
[![Tauri](https://img.shields.io/badge/Tauri-2-FFC131?logo=tauri&logoColor=white)](https://tauri.app)
[![Vue 3](https://img.shields.io/badge/Vue-3-4FC08D?logo=vue.js&logoColor=white)](https://vuejs.org)
[![Rust](https://img.shields.io/badge/Rust-stable-DEA584?logo=rust&logoColor=white)](https://www.rust-lang.org)

[Download](../../releases) · [Highlights](#highlights) · [Quick start](#quick-start) · [Build from source](#build-from-source)

</div>

<p align="center">
  <img src="docs/screenshots/launcher-client.png" alt="Rathole Launcher main window" width="780" />
</p>

---

## Why we built this

If you've ever set up rathole, you know the drill: SSH into a server, edit two TOML files, copy tokens around, restart the daemon, tail stderr, hope nothing was off by a character. It's a tool that does its job beautifully — once you've fought through the configuration ritual.

Rathole Launcher exists to skip that ritual. Every option has a form field. Tokens auto-fill from your server config. Logs stream live. Start, stop and restart are a click each. When the launcher itself crashes, the tunnels keep running and the next launch quietly re-attaches to them. We wanted the tool we wished we had on day one — and we figured you might want it too.

---

## Highlights

### 🪟 A visual editor for the entire rathole spec

Every documented option is reachable through a friendly form: addresses, tokens, heartbeats, retries, per-service overrides, and the whole transport zoo (TCP, TLS, Noise, WebSocket) with their respective sub-fields. The advanced section is collapsed by default, so simple setups stay simple.

### 🚀 Start, stop, watch, restart — all in one window

A sticky control bar at the top of every editor exposes start, stop, save and restart. Logs stream beneath it in real time, color-coded by source, capped at 1000 lines per instance. Save changes while a tunnel is running and a yellow strip appears that turns "apply the new config" into a single click.

### 🤖 Bring your own rathole — or let us fetch it

If no rathole binary is found alongside the launcher, an in-app banner offers to download the latest release from `rathole-org/rathole`, picking the right archive for your OS and CPU. Apple Silicon transparently falls back to the Intel build via Rosetta 2 when no native ARM asset is published yet. The launcher also checks for new releases on every launch and lets you know in the same place.

### 📋 Drop in your `server.toml`, get a working client

Click **+** in the sidebar, switch to **Import from server TOML**, paste your server file, click **Parse**. Service names, tokens and transport settings are extracted automatically; you only need to map each remote service to a local address. Done.

### 🛟 Survives crashes — its own and yours

Every rathole PID started by the launcher is persisted to disk. If the launcher dies (power outage, OOM, an accidental `kill -9`), the rathole children keep tunneling — they get re-parented to launchd / init / systemd and carry on. When the launcher comes back it spots the orphans, re-attaches, and lets you manage them again.

### 🍴 Tray-resident on all three desktops

Closing the window doesn't quit the app; it slides into the macOS menu bar, the Windows tray or the Linux status notifier. From there you can open, start or stop any configuration, see the running count at a glance, or actually quit — which gracefully shuts down every tunnel first.

### 🌍 Speaks four languages

The full UI **and** the tray menu are translated into **Simplified Chinese, English, Japanese and Korean**. The active language is auto-detected from your system locale, persisted across launches, and switchable from the bottom-left language picker without a reload.

### 🤝 Plays nice with neighbors

The launcher only ever touches processes it started. Anything else running rathole on the box is left alone, and shown in **Settings** with a clear "external" badge. If you try to start a server profile whose ports are already in use, the launcher refuses and tells you exactly which addresses collide.

---

## Quick start

### 1. Install

Pre-built bundles are attached to every [GitHub Release](../../releases):

| Platform | Grab | Run |
| --- | --- | --- |
| **macOS** (Apple Silicon / Intel) | `*.dmg` | Drag to `Applications`, right-click → **Open** the first time |
| **Windows** (x64) | `*.msi` or `*-setup.exe` | Run the installer |
| **Linux** (x64) | `*.AppImage` / `*.deb` / `*.rpm` | `chmod +x` then run, or use your package manager |

Bundles are unsigned by default, so the OS will warn on first launch — that's expected. Sign with your own certificate if you'd like to skip the prompt.

### 2. Get a rathole binary

On first launch, if no `rathole` is found next to the app, you'll see this banner:

> **rathole binary not detected**
> Download rathole 0.5.0 from GitHub, or override the path under Settings.
> &nbsp; &nbsp; **[ Download now ]** &nbsp; **[ Open Settings ]**

Click **Download now**, wait a few seconds, you're done. Prefer your own copy? Drop it next to the launcher or point at any path under **Settings**.

### 3. Configure

Two paths, pick whichever feels less work:

**Manual** — Click **+** in the sidebar, fill in the form. Each saved profile becomes plain TOML in `server_conf/` or `client_conf/` next to the launcher, so you can hand-edit or version-control them later.

**Import from server TOML** *(recommended for clients)* — Click **+**, switch to **Import from server TOML**, paste your server file, click **Parse**. Map each remote service to a local address (e.g. `127.0.0.1:22`), click **Create**. A complete client config drops into place.

### 4. Run

Hit **Start**. Logs come alive. If everything checks out, you'll see services registering with the server within a second or two. If a port is already taken or a token is off by a character, you'll know — and where to fix it.

---

## Build from source

### Prerequisites

- **Node.js** ≥ 18
- **Rust** stable ≥ 1.77 — install via [rustup](https://rustup.rs)
- **Tauri 2 platform deps** — see the [official guide](https://tauri.app/start/prerequisites/) (Xcode CLT on macOS, MSVC build tools on Windows, GTK + WebKit2GTK 4.1 + libsoup-3 + libappindicator on Linux)

### Develop

```sh
git clone https://github.com/<owner>/rathole-gui-launcher.git
cd rathole-gui-launcher
npm install
npm run tauri:dev
```

The first compile pulls roughly 600 Rust crates and takes 5–15 minutes depending on the machine; later runs are incremental and quick.

### Bundle locally

The `scripts/` folder ships a one-shot builder per platform. Each script verifies the toolchain, installs dependencies, runs `tauri build` and prints the final bundle paths.

| Platform | Command |
| --- | --- |
| **macOS** | `./scripts/build-macos.sh` &nbsp; *(optional: `--silicon`, `--intel`, `--universal`)* |
| **Linux** | `./scripts/build-linux.sh` |
| **Windows** | `pwsh scripts/build-windows.ps1` |

### Cut a release via GitHub Actions

`.github/workflows/release.yml` builds for **macOS Apple Silicon, macOS Intel, Windows x64 and Linux x64** in parallel and attaches every artifact to a draft GitHub Release whenever you push a `v*` tag:

```sh
# Bump the version in src-tauri/tauri.conf.json, commit, then:
git tag v0.1.0
git push origin v0.1.0
```

Apple and Windows code signing kick in automatically when the matching repo secrets are present (`APPLE_CERTIFICATE`, `TAURI_SIGNING_PRIVATE_KEY`, …). Without them the workflow simply produces unsigned bundles — perfectly fine for early releases.

---

## Tech stack

| Layer | Tools |
| --- | --- |
| Window shell | [Tauri 2](https://tauri.app) |
| Frontend | [Vue 3](https://vuejs.org) with `<script setup>`, [Vue Router](https://router.vuejs.org), [Pinia](https://pinia.vuejs.org), [Vue I18n](https://vue-i18n.intlify.dev) |
| UI library | [Antdv Next](https://github.com/antdv-next/antdv-next) — modern Vue 3 rewrite of Ant Design Vue |
| Backend | Rust + Tokio · [reqwest](https://crates.io/crates/reqwest) (rustls) · [sysinfo](https://crates.io/crates/sysinfo) · [zip](https://crates.io/crates/zip) · [toml](https://crates.io/crates/toml) (order-preserving) · [nix](https://crates.io/crates/nix) |
| Build | Vite, vue-tsc, tauri-cli |

---

## Contributing

We'd love help. A few practical hints to make things smooth:

- Run the dev workflow above, then format with `cargo fmt` (Rust) and Prettier defaults (TS / Vue) before sending a PR.
- New i18n strings go into all four locale files under `src/i18n/locales/` — keep the keys in sync.
- Adding a rathole config field? Mirror the change in both the Rust models (`src-tauri/src/models/`) and the TS types (`src/types/rathole.ts`).
- Anything bigger than a fix? Open an issue first; we'd love to talk through the design before code lands.

## Roadmap

A few directions we're thinking about — happy to take help on any of them:

- Dark mode
- Per-service status chips inside the editor
- Built-in self-signed TLS cert generator for quick LAN setups
- Optional bundled `rathole` binary for fully offline distribution

## Acknowledgements

Built on the shoulders of the excellent [rathole](https://github.com/rathole-org/rathole) project. None of this would matter without their work.

## License

[MIT](LICENSE) — fork it, ship it, sell it. A mention in your release notes is appreciated but not required.
