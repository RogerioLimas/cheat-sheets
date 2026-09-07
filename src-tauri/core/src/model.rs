//! Domain types shared across the core. Serde-serializable so commands can
//! return them across IPC. Field names mirror `src/lib/types.ts`.

use serde::Serialize;

/// Lightweight entry for search results — no rendered body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CheatSheetSummary {
    pub id: String,
    pub app: String,
    pub tags: Vec<String>,
}

/// Full cheat sheet, with body already sanitized to safe HTML.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatSheet {
    pub id: String,
    pub app: String,
    pub version: String,
    pub tags: Vec<String>,
    pub source: Option<String>,
    pub license: Option<String>,
    /// HTML already sanitized by [`crate::render`]. Safe to render.
    pub body_html: String,
}
