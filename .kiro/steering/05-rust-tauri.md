---
inclusion: fileMatch
fileMatchPattern: "src-tauri/**/*.rs"
---
# Rust / Tauri Backend Guidelines

1. Keep `#[tauri::command]` functions thin: parse/validate input, delegate
   to a core function, map the result/error to the command's return type.
   No business logic inside the command handler itself.
2. Model errors with `thiserror` for library/core code and surface them to
   the frontend as serializable, typed errors — never a raw
   `anyhow`/string dump of an internal error.
3. Never panic on recoverable errors. `unwrap`/`expect` are only allowed in
   tests or on invariants that are truly unreachable, with a comment
   explaining why.
4. Long-running or blocking work (file I/O, search indexing) must not run
   on the main/UI thread — use `tauri::async_runtime::spawn` or an async
   command.
5. Use `tracing` for logging, not `println!`/`eprintln!`. No sensitive data
   (paths outside the app, user input) in log messages at `info` level or
   above.
6. Global state shared across commands goes through `tauri::State`, wrapped
   in a `Mutex`/`RwLock` no larger than the section that needs protecting.
