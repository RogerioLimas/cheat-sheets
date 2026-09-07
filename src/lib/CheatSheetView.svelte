<script lang="ts">
  // Presentational: renders a cheat sheet. Defense in depth for XSS
  // (security steering rules 4 & 5): the body is already sanitized in the
  // Rust backend (ammonia), and we sanitize AGAIN here with DOMPurify before
  // raw-HTML insertion. Two independent layers so a gap in one doesn't reach
  // the webview.
  import DOMPurify from "dompurify";
  import type { CheatSheet } from "./types";

  interface Props {
    sheet: CheatSheet;
    onBack: () => void;
  }

  let { sheet, onBack }: Props = $props();

  // Re-sanitize on the client before rendering.
  let safeBody = $derived(DOMPurify.sanitize(sheet.bodyHtml));

  function onKeydown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      event.preventDefault();
      onBack();
    }
  }
</script>

<svelte:window onkeydown={onKeydown} />

<article class="sheet">
  <header>
    <button type="button" class="back" onclick={onBack}>← Back (Esc)</button>
    <h1>{sheet.app}</h1>
    <span class="version">v{sheet.version}</span>
  </header>

  <!-- Sanitized in Rust (ammonia) AND here (DOMPurify); see comment above. -->
  <!-- nosemgrep: svelte-html-injection -- input double-sanitized (ammonia + DOMPurify) -->
  <div class="body">{@html safeBody}</div>

  {#if sheet.source}
    <footer class="attribution">
      Source: <a href={sheet.source} target="_blank" rel="noreferrer noopener"
        >{sheet.source}</a
      >
      {#if sheet.license}· License: {sheet.license}{/if}
    </footer>
  {/if}
</article>

<style>
  .sheet {
    padding: 1rem;
  }
  header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 1rem;
  }
  h1 {
    font-size: 1.3rem;
    margin: 0;
  }
  .version {
    opacity: 0.6;
    font-size: 0.85rem;
  }
  .back {
    background: transparent;
    border: 1px solid var(--border, #3a3a4a);
    color: inherit;
    border-radius: 6px;
    padding: 0.3rem 0.6rem;
    cursor: pointer;
  }
  /* Multi-column layout like a printed cheat sheet. */
  .body {
    column-width: 22rem;
    column-gap: 2rem;
  }
  .body :global(h2) {
    font-size: 0.95rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    opacity: 0.8;
    break-after: avoid;
  }
  .body :global(table) {
    width: 100%;
    border-collapse: collapse;
    break-inside: avoid;
    margin-bottom: 1rem;
  }
  .body :global(td) {
    padding: 0.15rem 0.4rem;
    vertical-align: top;
  }
  .body :global(td:first-child) {
    font-family: ui-monospace, monospace;
    white-space: nowrap;
    opacity: 0.9;
  }
  .attribution {
    margin-top: 1.5rem;
    font-size: 0.75rem;
    opacity: 0.6;
  }
</style>
