<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let url = $state("");
  let qr = $state("");
  let running = $state(false);

  async function startGallery() {
    const res = await invoke<{ url: string; qr: string }>("host_gallery_start");
    url = res.url;
    qr = res.qr;
    running = true;
  }

  async function stopGallery() {
    await invoke("host_gallery_stop");
    running = false;
    url = "";
    qr = "";
  }
</script>

<div class="host">
  <h2>host gallery</h2>
  <p>Read-only local file server for phone access over Wi‑Fi.</p>
  {#if !running}
    <button onclick={startGallery}>start gallery server</button>
  {:else}
    <p class="url">{url}</p>
    <pre class="qr">{qr}</pre>
    <button onclick={stopGallery}>stop</button>
  {/if}
</div>

<style>
  .host {
    padding: 1.5rem;
  }

  h2 {
    font-size: 0.85rem;
    color: var(--dim);
    text-transform: uppercase;
  }

  .url {
    color: var(--accent);
    word-break: break-all;
  }

  .qr {
    color: var(--fg);
    font-size: 0.65rem;
    line-height: 1;
  }
</style>
