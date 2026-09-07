//! Markdown -> sanitized HTML. The single trust boundary for cheat sheet
//! bodies: whatever this returns is considered safe for the frontend to
//! render with `{@html}` (security steering rule 4).

use pulldown_cmark::{html, Options, Parser};

/// Render markdown to sanitized HTML.
///
/// Tables and other GFM extensions are enabled so cheat sheets can use the
/// two-column `| shortcut | action |` layout. Output is passed through
/// `ammonia` to strip any scripts/handlers before it can reach the webview.
pub fn markdown_to_safe_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(markdown, options);
    let mut raw_html = String::new();
    html::push_html(&mut raw_html, parser);

    // ammonia's default policy removes scripts, event handlers, and unsafe
    // URL schemes while keeping structural tags (tables, headings, code).
    ammonia::clean(&raw_html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_table_from_markdown() {
        let md = "| Shortcut | Action |\n|---|---|\n| Ctrl+P | Quick Open |";
        let html = markdown_to_safe_html(md);
        assert!(html.contains("<table>"));
        assert!(html.contains("Ctrl+P"));
        assert!(html.contains("Quick Open"));
    }

    #[test]
    fn strips_script_tags() {
        let md = "Hello <script>alert('xss')</script> world";
        let html = markdown_to_safe_html(md);
        assert!(!html.contains("<script>"));
        assert!(!html.contains("alert"));
    }

    #[test]
    fn strips_event_handler_attributes() {
        let md = "<img src=x onerror=alert(1)>";
        let html = markdown_to_safe_html(md);
        assert!(!html.contains("onerror"));
    }

    #[test]
    fn renders_headings() {
        let html = markdown_to_safe_html("## General");
        assert!(html.contains("<h2>"));
        assert!(html.contains("General"));
    }
}
