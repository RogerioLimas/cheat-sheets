---
inclusion: fileMatch
fileMatchPattern: "src/**/*.{svelte,ts}"
---
# Svelte / Frontend Guidelines

1. TypeScript strict mode on. No `any` — type the payloads exchanged with
   Tauri commands explicitly (mirror the Rust command's input/output
   types).
2. Wrap every `invoke()` call in a typed function in one module (e.g.
   `src/lib/api.ts`). Components never call `invoke` directly — this is
   the single seam to mock in tests and to update if a command signature
   changes.
3. Prefer Svelte runes/stores for state; avoid manual DOM manipulation
   (`document.querySelector`, etc.) inside components.
4. The global shortcut / search overlay must stay keyboard-navigable: every
   action reachable from the overlay needs a keyboard path, not just
   mouse/click.
5. No `{@html ...}` with unsanitized content (see security steering, rule 5).
6. Keep components small and focused: a component that both fetches data
   and renders a complex list is a candidate to split into a data-loading
   parent and a presentational child.
