---
inclusion: always
---
# Security Rules (mandatory, non-negotiable)

1. Never hardcode secrets, connection strings, API keys, or credentials in
   source, config files, or comments.
2. Tauri capabilities: grant each window/command only the exact permissions
   it needs via `src-tauri/capabilities/*.json`. Never enable a broad
   `core:default` or `shell:allow-execute`/`shell:allow-open` without a
   documented reason, and never pass user-controlled input to a shell or
   `open` command.
3. Treat all data crossing the IPC boundary (`invoke` calls from the
   frontend) as untrusted input. Validate and sanitize it in the Rust
   command before use, even though it originates from the app's own UI.
4. Never render untrusted or external content with Svelte's `{@html ...}`.
   If HTML rendering is unavoidable, sanitize first (e.g. DOMPurify).
5. Keep a strict Content-Security-Policy in `tauri.conf.json`. No
   `unsafe-inline` or `unsafe-eval` unless justified in a comment next to
   the setting.
6. Scope all filesystem access through Tauri's fs allowlist/scope. Never
   build file paths from unsanitized input (path traversal).
7. No `.unwrap()`/`.expect()` on values derived from IPC input, filesystem,
   network, or external processes. Handle errors explicitly with `Result`
   so a bad input can't panic and crash the app.
