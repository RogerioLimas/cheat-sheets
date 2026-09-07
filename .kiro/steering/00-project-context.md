---
inclusion: always
---
# Project Context

**Cheat Sheets Launcher** — cross-platform desktop app (Windows, Linux, macOS) that
shows instant cheat sheets for other applications via a global shortcut and search
overlay.

## Stack

- Backend: Rust + Tauri v2 (window management, global shortcuts, OS integration, IPC
  commands)
- Frontend: Svelte + TypeScript (search overlay UI, cheat sheet viewer)
- Communication: Tauri IPC (`invoke`) between the Svelte frontend and Rust commands
- Data: cheat sheet content loaded locally (storage format TBD)

## Layers

1. **UI (Svelte)** — search overlay, results list, cheat sheet window. No direct
   OS/file access.
2. **IPC bridge (Tauri commands)** — thin, typed wrappers exposed via
   `#[tauri::command]`. No business logic here; delegate to core.
3. **Core (Rust)** — search/matching logic, cheat sheet loading, global shortcut
   handling, window lifecycle.
4. **Data** — cheat sheet definitions, loaded and indexed at startup.

## Status

Planning/early development. No production code yet — steering rules apply from the
first commit.