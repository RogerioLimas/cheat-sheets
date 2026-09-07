import { describe, it, expect, vi, beforeEach } from "vitest";

// Mock the Tauri IPC boundary — unit tests never call the real backend
// (TDD steering rule 4).
const invoke = vi.fn();
vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => invoke(...args),
  // Pretend we're inside the Tauri window for these unit tests.
  isTauri: () => true,
}));

import { listCheatSheets, getCheatSheet } from "./api";

describe("api", () => {
  beforeEach(() => {
    invoke.mockReset();
  });

  it("invokes list_cheat_sheets with the query", async () => {
    invoke.mockResolvedValueOnce([
      { id: "vim", app: "Vim", tags: ["editor"] },
    ]);

    const result = await listCheatSheets("vi");

    expect(invoke).toHaveBeenCalledWith("list_cheat_sheets", { query: "vi" });
    expect(result).toEqual([{ id: "vim", app: "Vim", tags: ["editor"] }]);
  });

  it("invokes get_cheat_sheet with the id", async () => {
    invoke.mockResolvedValueOnce({
      id: "vim",
      app: "Vim",
      version: "1.0",
      tags: [],
      source: null,
      license: null,
      bodyHtml: "<h2>Motions</h2>",
    });

    const sheet = await getCheatSheet("vim");

    expect(invoke).toHaveBeenCalledWith("get_cheat_sheet", { id: "vim" });
    expect(sheet.bodyHtml).toContain("Motions");
  });

  it("propagates a rejected invoke as an error", async () => {
    invoke.mockRejectedValueOnce({ code: "not_found", message: "nope" });
    await expect(getCheatSheet("missing")).rejects.toEqual({
      code: "not_found",
      message: "nope",
    });
  });
});
