---
inclusion: always
---
# TDD Policy (mandatory)

1. No production code is written before a failing test exists for it.
   Red -> Green -> Refactor, in that order, every time.
2. A task is not complete until: the new test passes, the full test suite
   for the affected crate/package still passes, and no existing test was
   modified to make it pass unless the requirement itself changed.
3. Test naming:
   - Rust: descriptive snake_case, e.g. `returns_err_when_cheat_sheet_missing`.
   - TypeScript/Svelte (Vitest): `describe('ComponentOrModule')` +
     `it('does X when Y')`.
4. Unit tests must not touch the filesystem, network, or a real Tauri
   runtime:
   - Rust: mock boundaries via traits + `mockall`, or hand-written test
     doubles.
   - Frontend: mock `@tauri-apps/api` `invoke` calls; never call the real
     backend in a unit test.
5. Integration tests are separate and run only in the regression gate, not
   on every save:
   - Rust: `tests/` directory, may use a temp dir for real filesystem
     access.
   - Frontend/E2E: drive the built Tauri app, not mocked IPC.
6. If a bug is found, the first action is a regression test that
   reproduces it, before touching the fix.