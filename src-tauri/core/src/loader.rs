//! Parse a cheat sheet document (YAML front matter + markdown body) into the
//! domain model. Pure string-in / model-out so it is unit-testable without
//! touching the filesystem (TDD steering rule 4). The filesystem-backed
//! catalog lives in `catalog.rs`.

use serde::Deserialize;

use super::error::CoreError;
use super::model::{CheatSheet, CheatSheetSummary};
use super::render::markdown_to_safe_html;

/// Front matter fields parsed from the `---` YAML block at the top of a doc.
#[derive(Debug, Deserialize)]
struct FrontMatter {
    id: String,
    app: String,
    #[serde(default = "default_version")]
    version: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    source: Option<String>,
    #[serde(default)]
    license: Option<String>,
}

fn default_version() -> String {
    "1.0".to_string()
}

/// Split a raw document into (yaml_front_matter, markdown_body).
/// Documents must start with a `---` line, contain a closing `---`, then body.
fn split_front_matter(raw: &str) -> Result<(&str, &str), CoreError> {
    let rest = raw
        .strip_prefix("---\n")
        .or_else(|| raw.strip_prefix("---\r\n"))
        .ok_or(CoreError::FrontMatter)?;

    // Find the closing delimiter on its own line.
    let end = rest.find("\n---").ok_or(CoreError::FrontMatter)?;
    let yaml = &rest[..end];
    // Skip past the closing "---" line to the body.
    let after = &rest[end + 1..]; // at the "---" line
    let body = after
        .find('\n')
        .map(|nl| &after[nl + 1..])
        .unwrap_or("");
    Ok((yaml, body))
}

/// Parse a full cheat sheet document into the rendered domain model.
pub fn parse_cheat_sheet(raw: &str) -> Result<CheatSheet, CoreError> {
    let (yaml, body) = split_front_matter(raw)?;
    let fm: FrontMatter =
        serde_yaml::from_str(yaml).map_err(|_| CoreError::FrontMatter)?;

    if fm.id.trim().is_empty() {
        return Err(CoreError::InvalidId);
    }

    Ok(CheatSheet {
        id: fm.id,
        app: fm.app,
        version: fm.version,
        tags: fm.tags,
        source: fm.source,
        license: fm.license,
        body_html: markdown_to_safe_html(body),
    })
}

/// Case-insensitive substring match over app name and tags.
pub fn matches_query(summary: &CheatSheetSummary, query: &str) -> bool {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return true;
    }
    if summary.app.to_lowercase().contains(&q) {
        return true;
    }
    summary.tags.iter().any(|t| t.to_lowercase().contains(&q))
}

impl From<&CheatSheet> for CheatSheetSummary {
    fn from(s: &CheatSheet) -> Self {
        CheatSheetSummary {
            id: s.id.clone(),
            app: s.app.clone(),
            tags: s.tags.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "---\nid: vscode\napp: Visual Studio Code\nversion: \"1.0\"\ntags: [editor, ide]\nsource: https://example.com\nlicense: MIT\n---\n## General\n| Shortcut | Action |\n|---|---|\n| Ctrl+P | Quick Open |\n";

    #[test]
    fn parses_front_matter_fields() {
        let sheet = parse_cheat_sheet(SAMPLE).expect("should parse");
        assert_eq!(sheet.id, "vscode");
        assert_eq!(sheet.app, "Visual Studio Code");
        assert_eq!(sheet.version, "1.0");
        assert_eq!(sheet.tags, vec!["editor", "ide"]);
        assert_eq!(sheet.source.as_deref(), Some("https://example.com"));
        assert_eq!(sheet.license.as_deref(), Some("MIT"));
    }

    #[test]
    fn renders_body_to_sanitized_html() {
        let sheet = parse_cheat_sheet(SAMPLE).expect("should parse");
        assert!(sheet.body_html.contains("<table>"));
        assert!(sheet.body_html.contains("Quick Open"));
    }

    #[test]
    fn returns_err_when_front_matter_missing() {
        let err = parse_cheat_sheet("no front matter here").unwrap_err();
        assert!(matches!(err, CoreError::FrontMatter));
    }

    #[test]
    fn returns_err_when_front_matter_unterminated() {
        let err = parse_cheat_sheet("---\nid: x\napp: Y\n").unwrap_err();
        assert!(matches!(err, CoreError::FrontMatter));
    }

    #[test]
    fn returns_err_when_id_empty() {
        let raw = "---\nid: \"\"\napp: Y\n---\nbody\n";
        let err = parse_cheat_sheet(raw).unwrap_err();
        assert!(matches!(err, CoreError::InvalidId));
    }

    #[test]
    fn empty_query_matches_everything() {
        let s = CheatSheetSummary {
            id: "vim".into(),
            app: "Vim".into(),
            tags: vec!["editor".into()],
        };
        assert!(matches_query(&s, ""));
        assert!(matches_query(&s, "   "));
    }

    #[test]
    fn query_matches_app_name_case_insensitively() {
        let s = CheatSheetSummary {
            id: "vim".into(),
            app: "Vim".into(),
            tags: vec![],
        };
        assert!(matches_query(&s, "vi"));
        assert!(matches_query(&s, "VIM"));
        assert!(!matches_query(&s, "emacs"));
    }

    #[test]
    fn query_matches_tag() {
        let s = CheatSheetSummary {
            id: "vim".into(),
            app: "Vim".into(),
            tags: vec!["editor".into()],
        };
        assert!(matches_query(&s, "edit"));
    }
}
