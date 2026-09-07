<script lang="ts">
  // Data-loading parent (steering rule 6): owns state + api calls, delegates
  // rendering to SearchOverlay / CheatSheetView.
  import SearchOverlay from "./lib/SearchOverlay.svelte";
  import CheatSheetView from "./lib/CheatSheetView.svelte";
  import { listCheatSheets, getCheatSheet } from "./lib/api";
  import type { CheatSheet, CheatSheetSummary } from "./lib/types";

  let query = $state("");
  let results = $state<CheatSheetSummary[]>([]);
  let selected = $state<CheatSheet | null>(null);
  let error = $state<string | null>(null);

  async function refresh(q: string) {
    query = q;
    try {
      results = await listCheatSheets(q);
      error = null;
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }

  async function select(id: string) {
    try {
      selected = await getCheatSheet(id);
      error = null;
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }

  function back() {
    selected = null;
  }

  // Initial unfiltered list.
  refresh("");
</script>

<main>
  {#if error}
    <p class="error" role="alert">{error}</p>
  {/if}

  {#if selected}
    <CheatSheetView sheet={selected} onBack={back} />
  {:else}
    <SearchOverlay
      {query}
      {results}
      onQueryChange={refresh}
      onSelect={select}
    />
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    font-family: system-ui, sans-serif;
    background: #16161e;
    color: #e6e6ee;
  }
  main {
    min-height: 100vh;
  }
  .error {
    margin: 0;
    padding: 0.6rem 1rem;
    background: #4a1e1e;
    color: #ffd9d9;
  }
</style>
