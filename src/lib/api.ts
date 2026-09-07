// Single seam for all Tauri IPC. Components never call `invoke` directly.
// This is the one place to mock in unit tests and to update if a command
// signature changes (see frontend steering rule 2).
import { invoke, isTauri } from "@tauri-apps/api/core";
import type { CheatSheet, CheatSheetSummary } from "./types";

/**
 * The frontend only works inside the Tauri window (where the IPC bridge
 * exists). Opened in a plain browser at localhost:1420 there is no bridge,
 * so surface a clear message instead of a cryptic "invoke is undefined".
 */
function ensureTauri(): void {
  if (!isTauri()) {
    throw new Error(
      "Not running inside the CheatKeys window. Launch with `npm run tauri dev` — a browser tab at localhost:1420 has no Tauri backend.",
    );
  }
}

/** List all available cheat sheets, optionally filtered by a query. */
export function listCheatSheets(query: string): Promise<CheatSheetSummary[]> {
  ensureTauri();
  return invoke<CheatSheetSummary[]>("list_cheat_sheets", { query });
}

/** Load a single cheat sheet by id, with sanitized HTML body. */
export function getCheatSheet(id: string): Promise<CheatSheet> {
  ensureTauri();
  return invoke<CheatSheet>("get_cheat_sheet", { id });
}
