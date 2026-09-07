//! Typed, serializable errors surfaced to the frontend (steering rule 2).
//! Never leak a raw internal error string across IPC.

use serde::Serialize;

/// Errors from loading/parsing/rendering cheat sheets.
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("cheat sheet not found: {0}")]
    NotFound(String),

    #[error("invalid cheat sheet id")]
    InvalidId,

    #[error("failed to read cheat sheet")]
    Read,

    #[error("malformed front matter")]
    FrontMatter,
}

/// Wire representation sent to the frontend: a stable machine-readable `code`
/// plus a human-readable `message`. No internal paths or raw error dumps.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WireError {
    pub code: String,
    pub message: String,
}

impl From<CoreError> for WireError {
    fn from(e: CoreError) -> Self {
        let code = match &e {
            CoreError::NotFound(_) => "not_found",
            CoreError::InvalidId => "invalid_id",
            CoreError::Read => "read_error",
            CoreError::FrontMatter => "front_matter_error",
        };
        WireError {
            code: code.to_string(),
            message: e.to_string(),
        }
    }
}
