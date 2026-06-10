<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let ollamaOk = $state(false);
  let dataPath = $state("");
  let vaultPath = $state("");

  onMount(async () => {
    const status = await invoke<{ ollama: boolean; data: string; vault: string }>("utils_status");
    ollamaOk = status.ollama;
    dataPath = status.data;
    vaultPath = status.vault;
  });
</script>

<div class="utils">
  <h2>utils</h2>
  <dl>
    <dt>Ollama</dt>
    <dd>{ollamaOk ? "online" : "offline"}</dd>
    <dt>Data</dt>
    <dd>{dataPath}</dd>
    <dt>Vault</dt>
    <dd>{vaultPath}</dd>
  </dl>
  <p class="hint">Notes live in <code>{dataPath}/notes</code></p>
</div>

<style>
  .utils {
    padding: 1.5rem;
  }

  h2 {
    font-size: 0.85rem;
    color: var(--dim);
    text-transform: uppercase;
  }

  dl {
    display: grid;
    grid-template-columns: 120px 1fr;
    gap: 0.5rem 1rem;
  }

  dt {
    color: var(--dim);
  }

  .hint {
    margin-top: 2rem;
    color: var(--dim);
    font-size: 0.85rem;
  }
</style>
