// Types mirror the Rust command input/output payloads exactly.
// Keep in sync with src-tauri/src/core/model.rs.

/** Lightweight entry returned by search — no rendered body. */
export interface CheatSheetSummary {
  id: string;
  app: string;
  tags: string[];
}

/** Full cheat sheet with pre-sanitized HTML body (sanitized in Rust). */
export interface CheatSheet {
  id: string;
  app: string;
  version: string;
  tags: string[];
  source: string | null;
  license: string | null;
  /** HTML already sanitized by the backend. Safe to render. */
  bodyHtml: string;
}
