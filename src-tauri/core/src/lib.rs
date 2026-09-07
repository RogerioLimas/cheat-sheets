//! Framework-agnostic core: cheat sheet loading, rendering, search.
//! No Tauri types here — unit-testable without a Tauri runtime or system libs.

pub mod catalog;
pub mod error;
pub mod loader;
pub mod model;
pub mod render;
