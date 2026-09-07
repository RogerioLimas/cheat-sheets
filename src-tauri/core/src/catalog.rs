//! In-memory catalog of cheat sheets loaded from a directory of `.md` files.
//! Filesystem access is isolated here so the rest of core stays pure and
//! unit-testable. Integration tests (tests/) exercise the real filesystem.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::error::CoreError;
use super::loader::{matches_query, parse_cheat_sheet};
use super::model::{CheatSheet, CheatSheetSummary};

/// Holds parsed cheat sheets keyed by id, plus their search summaries.
#[derive(Debug, Default)]
pub struct Catalog {
    sheets: HashMap<String, CheatSheet>,
}

impl Catalog {
    /// Load every `*.md` file directly inside `dir` into the catalog.
    /// A single malformed file is skipped (logged) rather than failing the
    /// whole load, so one bad contribution can't break the app.
    pub fn load_from_dir(dir: &Path) -> Result<Self, CoreError> {
        let mut sheets = HashMap::new();
        let entries = std::fs::read_dir(dir).map_err(|_| CoreError::Read)?;

        for entry in entries.flatten() {
            let path: PathBuf = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            match std::fs::read_to_string(&path) {
                Ok(raw) => match parse_cheat_sheet(&raw) {
                    Ok(sheet) => {
                        sheets.insert(sheet.id.clone(), sheet);
                    }
                    Err(_) => {
                        tracing::warn!("skipping malformed cheat sheet file");
                    }
                },
                Err(_) => tracing::warn!("could not read cheat sheet file"),
            }
        }

        Ok(Self { sheets })
    }

    /// Search summaries matching `query` (empty query returns all), sorted by
    /// app name for stable ordering.
    pub fn search(&self, query: &str) -> Vec<CheatSheetSummary> {
        let mut out: Vec<CheatSheetSummary> = self
            .sheets
            .values()
            .map(CheatSheetSummary::from)
            .filter(|s| matches_query(s, query))
            .collect();
        out.sort_by(|a, b| a.app.to_lowercase().cmp(&b.app.to_lowercase()));
        out
    }

    /// Fetch a full cheat sheet by id.
    pub fn get(&self, id: &str) -> Result<CheatSheet, CoreError> {
        self.sheets
            .get(id)
            .cloned()
            .ok_or_else(|| CoreError::NotFound(id.to_string()))
    }
}
