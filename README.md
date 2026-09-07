# CheatKeys — Cheat Sheets Launcher

A fast, lightweight, cross-platform desktop app that puts application cheat
sheets one keystroke away. Press a global shortcut, search for a tool, and read
its most important shortcuts and commands in a focused window — without leaving
your current task.

Built with Rust + Tauri v2 (tiny binary, native webview, low memory) and a
Svelte + TypeScript frontend.

## Features

- **Global shortcut** (`Ctrl+Shift+.`) toggles the launcher from anywhere.
- **Searchable overlay**, fully keyboard-navigable (`↑`/`↓`/`Enter`/`Esc`).
- **Multi-column cheat sheet view** rendered from Markdown.
- **Local, offline content** — cheat sheets are bundled as Markdown files.
- **Small footprint** — native OS webview instead of a bundled browser.
- **Extensible** — add a new cheat sheet by dropping in one Markdown file.

## Tech stack

| Layer | Technology |
|-------|-----------|
| Backend / shell | Rust, Tauri v2 |
| Global shortcut | `tauri-plugin-global-shortcut` |
| Frontend | Svelte 5, TypeScript (strict), Vite |
| Markdown → HTML | `pulldown-cmark` + `ammonia` (Rust), DOMPurify (frontend) |
| Tests | Rust (`cargo test`), Vitest (frontend) |

## Project layout

```
cheat-sheets/
├─ index.html                 # Vite entry
├─ package.json               # frontend + Tauri CLI scripts
├─ vite.config.ts             # Vite + Vitest config
├─ src/                       # Svelte + TypeScript frontend
│  ├─ App.svelte              # data-loading parent (state + API calls)
│  ├─ main.ts
│  └─ lib/
│     ├─ api.ts               # the single Tauri IPC seam (invoke wrapper)
│     ├─ types.ts             # payload types mirroring the Rust commands
│     ├─ SearchOverlay.svelte # search UI (presentational)
│     └─ CheatSheetView.svelte# cheat sheet renderer (presentational)
└─ src-tauri/                 # Cargo workspace
   ├─ core/                   # framework-agnostic core crate (no Tauri)
   │  ├─ src/                 # model, loader, render, catalog, error
   │  └─ tests/               # integration tests (real filesystem)
   └─ app/                    # Tauri application crate
      ├─ src/                 # main.rs, lib.rs, commands.rs (thin IPC)
      ├─ capabilities/        # scoped Tauri permissions
      ├─ cheatsheets/         # bundled Markdown cheat sheets
      └─ tauri.conf.json
```

The backend is split into two crates on purpose: `core` holds all business
logic (parsing, rendering, search) with no Tauri dependency, so it compiles and
tests without any system GUI libraries. `app` is the thin Tauri layer.

## Prerequisites

- **Node.js** 18+ and npm
- **Rust** (stable) via [rustup](https://rustup.rs)
- Platform-specific dependencies below

### Linux (Debian/Ubuntu, incl. WSL2)

```bash
sudo apt update && sudo apt install -y \
  libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev \
  librsvg2-dev libdbus-1-dev pkg-config build-essential libssl-dev
```

On WSL2, a GUI requires WSLg (bundled with recent Windows 10/11). The Tauri
window then opens as a native Windows window.

### Windows

Install [Rust](https://rustup.rs) (MSVC toolchain), the **Visual Studio Build
Tools** with the "Desktop development with C++" workload, and Node.js. WebView2
is preinstalled on Windows 11; on older Windows the Tauri installer provides it.

### macOS

Install [Rust](https://rustup.rs) and the Xcode Command Line Tools
(`xcode-select --install`).

## Getting started

```bash
# 1. Install frontend dependencies
npm install

# 2. Run the app in development (hot reload)
npm run tauri dev
```

`npm run tauri dev` starts Vite and launches the native CheatKeys window. The
app runs in that window — opening `localhost:1420` in a regular browser will
not work, because a browser has no Tauri IPC bridge.

### Using the app

1. Press `Ctrl+Shift+.` to show or hide the launcher.
2. Type to filter (e.g. `vim`, `code`).
3. Use `↑`/`↓` to move, `Enter` to open, `Esc` to go back.

## Building for release

```bash
npm run tauri build
```

Installers and binaries are written to
`src-tauri/target/release/bundle/`:

- **Linux**: `.deb`, `.rpm`, AppImage
- **Windows**: `.msi` and NSIS `.exe`
- **macOS**: `.app` and `.dmg`

Each target must be built on its own OS (or via CI). The release profile is
size-optimized (`opt-level = "s"`, LTO, stripped symbols).

### Building on Windows from a WSL checkout

The Windows filesystem can reach the WSL checkout directly. In **PowerShell**:

```powershell
cd \\wsl$\Ubuntu\eng\cheat-sheets
npm install          # regenerates platform-specific binaries
npm run tauri build
```

## Testing

```bash
# Frontend unit tests (Vitest)
npm test

# Rust core tests (no system GUI libs required)
cargo test -p cheatkeys-core --manifest-path src-tauri/Cargo.toml

# Full Rust workspace (requires the platform GUI libs above)
cargo test --manifest-path src-tauri/Cargo.toml

# TypeScript / Svelte type checking
npm run check
```

## Security scanning

The repo ships a local, offline security pipeline (Semgrep, Trivy, Gitleaks)
run via `make`:

```bash
make setup      # one-time: activate git hooks + check scanners
make security   # run all scans (SAST, dependencies, secrets)
```

These also run in the pre-commit hook and in CI. Code never leaves the machine.

## Adding a cheat sheet

Drop a Markdown file into `src-tauri/app/cheatsheets/`. Each file has a YAML
front matter block followed by the content:

```markdown
---
id: git
app: Git
version: "1.0"
tags: [vcs, cli]
source: https://example.com/git-cheatsheet
license: MIT
---

## Basics

| Command | Action |
|---------|--------|
| git status | Show working tree status |
| git add <file> | Stage a file |
```

Notes:
- `id` must be a slug (letters, digits, `-`, `_`); it is validated at the IPC
  boundary.
- Use tables for shortcut/action pairs — they render in a multi-column layout.
- Always keep `source` and `license` when the content comes from an external
  source, and the app will display the attribution.
- Content is sanitized twice (server-side with `ammonia`, client-side with
  DOMPurify) before rendering.

## How it works

1. On startup the app loads every `.md` file from the bundled `cheatsheets/`
   directory, parses the front matter, renders the body to sanitized HTML, and
   indexes summaries in memory.
2. The frontend calls two typed Tauri commands through a single API module:
   `list_cheat_sheets(query)` and `get_cheat_sheet(id)`.
3. The global shortcut toggles the window; the search overlay drives the rest.

## License

No license chosen yet. Pick one that fits your goals (MIT, Apache-2.0, or
GPL-3.0) before publishing. Note that bundled cheat sheet content keeps its own
source/license attribution in each file's front matter.
