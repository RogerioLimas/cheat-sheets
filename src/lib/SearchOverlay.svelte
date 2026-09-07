<script lang="ts">
  // Presentational: owns no data fetching. Emits selection via callback prop.
  import type { CheatSheetSummary } from "./types";

  interface Props {
    query: string;
    results: CheatSheetSummary[];
    onQueryChange: (q: string) => void;
    onSelect: (id: string) => void;
  }

  let { query, results, onQueryChange, onSelect }: Props = $props();

  // Keyboard navigation state (steering rule 4: fully keyboard-navigable).
  let active = $state(0);

  // Focus the search box on mount without the `autofocus` attribute
  // (which Svelte flags as an a11y issue). Programmatic focus is the
  // accessible pattern here.
  let searchEl: HTMLInputElement | undefined = $state();
  $effect(() => {
    searchEl?.focus();
  });

  // Keep the active index within bounds when results change.
  $effect(() => {
    if (active >= results.length) active = Math.max(0, results.length - 1);
  });

  function onKeydown(event: KeyboardEvent) {
    if (event.key === "ArrowDown") {
      event.preventDefault();
      active = Math.min(active + 1, results.length - 1);
    } else if (event.key === "ArrowUp") {
      event.preventDefault();
      active = Math.max(active - 1, 0);
    } else if (event.key === "Enter") {
      event.preventDefault();
      const chosen = results[active];
      if (chosen) onSelect(chosen.id);
    }
  }
</script>

<div class="overlay">
  <input
    bind:this={searchEl}
    class="search"
    type="text"
    placeholder="Search apps… (Vim, awk, Git)"
    value={query}
    oninput={(e) => onQueryChange(e.currentTarget.value)}
    onkeydown={onKeydown}
    aria-label="Search cheat sheets"
  />
  <ul class="results" role="listbox" aria-label="Results">
    {#each results as r, i (r.id)}
      <li>
        <button
          type="button"
          class="result"
          class:active={i === active}
          role="option"
          aria-selected={i === active}
          onclick={() => onSelect(r.id)}
          onmouseenter={() => (active = i)}
        >
          <span class="app">{r.app}</span>
          <span class="tags">{r.tags.join(" · ")}</span>
        </button>
      </li>
    {:else}
      <li class="empty">No matches</li>
    {/each}
  </ul>
</div>

<style>
  .overlay {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding: 1rem;
  }
  .search {
    font-size: 1.1rem;
    padding: 0.6rem 0.8rem;
    border-radius: 8px;
    border: 1px solid var(--border, #3a3a4a);
    background: var(--input-bg, #1e1e2a);
    color: inherit;
    outline: none;
  }
  .search:focus {
    border-color: var(--accent, #5b8def);
  }
  .results {
    list-style: none;
    margin: 0;
    padding: 0;
    max-height: 60vh;
    overflow-y: auto;
  }
  .result {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    width: 100%;
    text-align: left;
    padding: 0.5rem 0.8rem;
    border: none;
    border-radius: 6px;
    background: transparent;
    color: inherit;
    cursor: pointer;
  }
  .result.active {
    background: var(--accent, #5b8def);
    color: #fff;
  }
  .tags {
    font-size: 0.8rem;
    opacity: 0.7;
  }
  .empty {
    padding: 0.5rem 0.8rem;
    opacity: 0.6;
  }
</style>
