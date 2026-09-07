//! Integration test: load real `.md` files from a temp dir via the catalog.
//! Runs in the regression gate, not on every save (TDD steering rule 5).

use std::fs;

use cheatkeys_core::catalog::Catalog;

fn write(dir: &std::path::Path, name: &str, contents: &str) {
    fs::write(dir.join(name), contents).expect("write temp cheat sheet");
}

const VIM: &str = "---\nid: vim\napp: Vim\ntags: [editor]\n---\n## Motions\n| Key | Action |\n|---|---|\n| w | word |\n";
const GIT: &str = "---\nid: git\napp: Git\ntags: [vcs]\n---\n## Basics\n| Cmd | Action |\n|---|---|\n| add | stage |\n";

#[test]
fn loads_and_searches_real_files() {
    let dir = tempfile::tempdir().expect("temp dir");
    write(dir.path(), "vim.md", VIM);
    write(dir.path(), "git.md", GIT);
    write(dir.path(), "ignore.txt", "not a cheat sheet");

    let catalog = Catalog::load_from_dir(dir.path()).expect("load");

    // Empty query returns both, sorted by app name (Git before Vim).
    let all = catalog.search("");
    assert_eq!(all.len(), 2);
    assert_eq!(all[0].app, "Git");
    assert_eq!(all[1].app, "Vim");

    // Query filters.
    let vim_only = catalog.search("vim");
    assert_eq!(vim_only.len(), 1);
    assert_eq!(vim_only[0].id, "vim");

    // Full fetch renders sanitized HTML.
    let sheet = catalog.get("vim").expect("get vim");
    assert!(sheet.body_html.contains("<table>"));

    // Missing id is an error, not a panic.
    assert!(catalog.get("nope").is_err());
}

#[test]
fn skips_malformed_files_without_failing() {
    let dir = tempfile::tempdir().expect("temp dir");
    write(dir.path(), "good.md", VIM);
    write(dir.path(), "bad.md", "no front matter at all");

    let catalog = Catalog::load_from_dir(dir.path()).expect("load");
    assert_eq!(catalog.search("").len(), 1);
}
