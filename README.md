# Rathole Launcher

A cross-platform graphical launcher for [rathole](https://github.com/rathole-org/rathole), the high-performance reverse proxy.

The launcher is meant for users who want to run rathole on their own machines without dealing with the command line. It edits TOML configuration files through a friendly UI, starts and stops local instances, surfaces real-time logs and lets you keep multiple server / client profiles side by side.

The launcher does **not** ship rathole itself. You drop a `rathole` binary next to the launcher executable and it picks it up automatically.

## Features

- Server and Client modes, each with as many named configurations as you need.
- Visual editing for every documented rathole option:
  - `bind_addr` / `remote_addr`, `default_token`, heartbeat / retry tuning.
  - Per-service `type`, `bind_addr` / `local_addr`, `token`, `nodelay`, `retry_interval`.
  - Transport selection (`tcp`, `tls`, `noise`, `websocket`) with all matching subfields.
- Start / stop with graceful shutdown (SIGTERM → SIGKILL on Unix, TerminateProcess on Windows).
- Live log streaming with stdout / stderr / system tagging and a 1000-line ring buffer.
- Pre-flight port-conflict check before starting a server profile.
- Detection of other rathole processes on the system (the launcher never touches anything it did not start itself).
- Configuration files are stored as plain TOML in `server_conf/` and `client_conf/`, so you can hand-edit or version-control them if you wish.
- UI translated into Simplified Chinese, English, Japanese and Korean. The selection is auto-detected on first launch and can be changed at any time from the sidebar or `Settings`.

## Repository layout

```
rathole-gui-launcher/
├── package.json              Frontend dependencies and scripts
├── vite.config.ts            Vite + Vue plugin configuration
├── tsconfig*.json            TypeScript project references
├── index.html                Vite entry
├── src/                      Vue 3 application
│   ├── main.ts
│   ├── App.vue
│   ├── api/                  invoke wrappers and event listeners
│   ├── components/           UI components (layout, editors, panels)
│   ├── composables/          useLanguage, etc.
│   ├── i18n/                 vue-i18n setup + zh-CN / en / ja / ko locale JSON
│   ├── router/
│   ├── stores/               Pinia stores (app, configs, runtime)
│   ├── styles/
│   ├── types/                TS types mirroring the Rust models
│   ├── utils/
│   └── views/
└── src-tauri/                Tauri 2 / Rust backend
    ├── Cargo.toml
    ├── tauri.conf.json
    ├── build.rs
    ├── capabilities/
    ├── icons/                Placeholder icons (regenerate with `tauri icon`)
    └── src/
        ├── main.rs
        ├── lib.rs            Builder, command registration, lifecycle
        ├── commands.rs       All `#[tauri::command]` handlers
        ├── error.rs          Serializable error type
        ├── paths.rs          Cross-platform path resolution
        ├── models/           Server / client TOML models
        ├── store/            Config CRUD, settings persistence
        └── runtime/          Process manager, port checks, ps lookup
```

## Toolchain

- Node.js 18+
- Rust 1.77+ (stable)
- Platform prerequisites for Tauri 2 — see <https://tauri.app/start/prerequisites/>

## Development

```sh
npm install
npm run tauri:dev
```

The first `tauri:dev` build will compile the Rust backend; subsequent runs are incremental. `server_conf/` and `client_conf/` are created next to the dev binary the first time the launcher starts.

To work on the Vue layer in isolation:

```sh
npm run dev
```

(Note: most actions invoke Tauri commands, so the standalone web mode is for layout work only.)

## Building release bundles

```sh
npm run tauri:build
```

This produces an installer / app bundle under `src-tauri/target/release/bundle/`:

- macOS — `.app` and `.dmg`
- Windows — `.msi` and `.exe`
- Linux — `.deb`, `.rpm`, `.AppImage` (depending on packaging tools available on the host)

### One-shot local build scripts

The `scripts/` folder contains a single-command builder for each platform. Each script verifies the toolchain, installs dependencies and produces the platform's bundles.

| Platform | Command |
| --- | --- |
| macOS  | `./scripts/build-macos.sh` (optional flags: `--silicon`, `--intel`, `--universal`) |
| Linux  | `./scripts/build-linux.sh` |
| Windows | `pwsh scripts/build-windows.ps1` (or `powershell -ExecutionPolicy Bypass -File scripts/build-windows.ps1`) |

When everything is set up, the launcher binary lives at `src-tauri/target/release/` (or under a target-specific subdirectory when cross-compiling), and the bundles are inside `src-tauri/target/.../release/bundle/`. Each script prints the exact paths when it finishes.

### Icons

The repository ships placeholder icons that satisfy the Tauri config schema during `tauri dev`. Before publishing release bundles, replace them with your own:

```sh
npm run tauri -- icon path/to/source.png
```

This regenerates `src-tauri/icons/` with platform-correct PNG / ICNS / ICO files.

## Distributing alongside rathole

The launcher resolves the rathole binary (and its config folders) relative to its own executable:

| Platform | Default rathole location | Config folders |
| --- | --- | --- |
| macOS  | next to the `.app` bundle | next to the `.app` bundle |
| Windows | next to `rathole-launcher.exe` | next to the `.exe` |
| Linux  | next to the binary | next to the binary |

Drop the rathole binary into that location after installing the launcher and the launcher will pick it up automatically. The path can also be overridden under `Settings`.

## Cutting a release on GitHub

A release workflow is committed at `.github/workflows/release.yml`. It builds the launcher for macOS (Apple Silicon and Intel), Windows x64 and Linux x64 in parallel and attaches every bundle to a **draft** GitHub release.

To cut a release:

1. Bump the version in [`src-tauri/tauri.conf.json`](src-tauri/tauri.conf.json) (and optionally `package.json`) and commit.
2. Tag the commit and push the tag:

   ```sh
   git tag v0.1.0
   git push origin v0.1.0
   ```
3. The workflow runs automatically. When all jobs finish, a draft release named `Rathole Launcher 0.1.0` will appear under the repository's **Releases** page with the bundles attached.
4. Review the draft, edit the notes if needed, and publish it.

The matrix produces:

| Job | Bundle artifacts |
| --- | --- |
| `aarch64-apple-darwin` | `.app.tar.gz`, `.dmg` |
| `x86_64-apple-darwin` | `.app.tar.gz`, `.dmg` |
| `x86_64-unknown-linux-gnu` | `.deb`, `.AppImage`, `.rpm` |
| `x86_64-pc-windows-msvc` | `.msi`, NSIS `setup.exe` |

The workflow also accepts a manual `workflow_dispatch` trigger from the Actions tab. When triggered manually it only builds and uploads the artifacts to the workflow run (nothing is released), which is convenient for verifying changes without cutting a tag.

### Code signing

The workflow reads the Apple and Windows signing secrets if they are set, otherwise it produces unsigned bundles. Add these repository secrets to enable signing:

- macOS: `APPLE_CERTIFICATE`, `APPLE_CERTIFICATE_PASSWORD`, `APPLE_SIGNING_IDENTITY`, `APPLE_ID`, `APPLE_PASSWORD`, `APPLE_TEAM_ID`
- Windows: `TAURI_SIGNING_PRIVATE_KEY`, `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`

See the [tauri-action documentation](https://github.com/tauri-apps/tauri-action) for the full list of supported signing variables.

## Coexisting with other rathole instances

The launcher only manages processes it spawned itself; any rathole already running on the machine is left untouched. The `Settings` page lists every detected rathole process so you can tell which ones are managed by the launcher and which are external. When you try to start a server configuration whose ports are already in use the launcher refuses to spawn the process and reports the conflicting addresses.

## License

MIT.
