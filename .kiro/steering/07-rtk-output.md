---
inclusion: auto
name: RTK Output Compression
description: >-
  How to run shell/terminal commands through the `rtk` proxy to compress
  output and save tokens. Apply whenever running ls, tree, find, grep, rg,
  git, gh, cargo, pnpm, npm, npx, tsc, vitest, lint, or other CLI commands in
  the terminal.
---
# RTK Output Compression (token savings)

`rtk` is a CLI proxy that filters and compresses command output before it
reaches the model context. Prefer the `rtk` equivalent for the commands below
to reduce token usage. `rtk` has no native Kiro hook, so this is applied by
emitting `rtk`-prefixed commands directly — no interception, no behavior change
beyond compacter output.

## Rules

1. When a common command has an `rtk` subcommand, invoke it through `rtk`
   instead of the raw binary (e.g. `rtk ls /` instead of `ls /`).
2. Map to the specific `rtk` subcommand when one exists; only fall back to
   `rtk run "<cmd>"` (raw, untracked) or `rtk proxy <cmd>` (untracked but
   measured) when there is no specialized subcommand.
3. Do NOT wrap commands whose exact/complete output is required for
   correctness (e.g. reading a file byte-for-byte, hashing, generating a
   diff to apply). Compression is lossy — use raw output when fidelity
   matters.
4. Never let `rtk` change command semantics. If wrapping would alter exit
   codes, flags, or interactive behavior, run the command raw.
5. Prefer dedicated Kiro tools over shell where the system prompt already
   requires it (reading files, editing, searching, listing). Use `rtk` for
   the terminal operations that still go through the shell.

## Command mapping (this project's stack)

| Raw command | Use instead |
|---|---|
| `ls` | `rtk ls` |
| `tree` | `rtk tree` |
| `find …` | `rtk find …` (accepts native flags like `-name`, `-type`) |
| `grep …` | `rtk grep …` |
| `rg …` | `rtk rg …` |
| `git <sub>` | `rtk git <sub>` |
| `gh <sub>` | `rtk gh <sub>` |
| `glab <sub>` | `rtk glab <sub>` |
| `diff …` | `rtk diff …` |
| `cargo <sub>` | `rtk cargo <sub>` |
| `pnpm <sub>` | `rtk pnpm <sub>` |
| `npm run <script>` | `rtk npm <script>` |
| `npx <tool>` | `rtk npx <tool>` (routes tsc/eslint/prisma to filters) |
| `tsc` | `rtk tsc` |
| `vitest` | `rtk vitest` |
| ESLint | `rtk lint` |
| prettier check | `rtk prettier` / `rtk format` |
| word/line count | `rtk wc` |
| environment vars | `rtk env` (filtered) |
| project deps | `rtk deps` |
| show JSON | `rtk json` (add `--keys-only` for schema) |
| tail/filter logs | `rtk log` |
| run + only errors | `rtk err <cmd>` |
| run tests + only failures | `rtk test <cmd>` |

### Tauri v2 (Rust backend)
- Build / check / test / clippy: `rtk cargo build`, `rtk cargo test`,
  `rtk cargo clippy`, `rtk cargo check`.
- Full Tauri build via cargo passthrough or `rtk run "cargo tauri build"`
  when no specialized subcommand fits.

### Svelte + TypeScript (frontend)
- Type check: `rtk tsc` (or `rtk npx tsc`).
- Unit tests (Vitest): `rtk vitest --run`.
- Lint / format: `rtk lint`, `rtk format`.

## Exceptions (run raw, do NOT wrap)
- Long-running processes: dev servers, watchers (`tauri dev`,
  `vite`, `--watch`). Compression offers nothing and can buffer output.
- Commands piped into an apply/parse step that needs exact bytes.
- Anything where a non-zero exit or specific stderr must be preserved
  verbatim for debugging.

## Measuring
- `rtk gain` shows cumulative token-savings history when you want a report.
