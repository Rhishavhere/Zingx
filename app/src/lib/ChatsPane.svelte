<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let threads = $state<{ id: string; title: string }[]>([]);
  let activeId = $state<string | null>(null);
  let input = $state("");
  let messages = $state<{ role: string; content: string }[]>([]);
  let streaming = $state("");

  onMount(async () => {
    threads = await invoke("chat_list");
  });

  async function openThread(id: string) {
    activeId = id;
    messages = await invoke("chat_load", { id });
    streaming = "";
  }

  async function newThread() {
    const id = await invoke<string>("chat_create", { title: "new chat" });
    threads = await invoke("chat_list");
    await openThread(id);
  }

  async function send() {
    if (!activeId || !input.trim()) return;
    const prompt = input.trim();
    input = "";
    messages = [...messages, { role: "user", content: prompt }];
    const reply = await invoke<string>("chat_send", { id: activeId, prompt });
    messages = [...messages, { role: "assistant", content: reply }];
    threads = await invoke("chat_list");
  }
</script>

<div class="chats">
  <aside>
    <button onclick={newThread}>+ new</button>
    <ul>
      {#each threads as t}
        <li><button class:active={activeId === t.id} onclick={() => openThread(t.id)}>{t.title}</button></li>
      {/each}
    </ul>
  </aside>
  <section>
    {#if !activeId}
      <p class="empty">Select or create a chat</p>
    {:else}
      <div class="msgs">
        {#each messages as m}
          <div class="msg {m.role}">{m.content}</div>
        {/each}
        {#if streaming}
          <div class="msg assistant">{streaming}</div>
        {/if}
      </div>
      <form class="composer" onsubmit={(e) => { e.preventDefault(); send(); }}>
        <input bind:value={input} placeholder="message..." />
        <button type="submit">send</button>
      </form>
    {/if}
  </section>
</div>

<style>
  .chats {
    display: grid;
    grid-template-columns: 220px 1fr;
    height: 100%;
  }

  aside {
    border-right: 1px solid var(--border);
    padding: 0.5rem;
  }

  aside ul {
    list-style: none;
    padding: 0;
    margin: 0.5rem 0 0;
  }

  aside li button {
    width: 100%;
    text-align: left;
    padding: 0.35rem;
    margin-bottom: 0.25rem;
    background: transparent;
  }

  aside li button.active {
    color: var(--accent);
  }

  section {
    display: grid;
    grid-template-rows: 1fr auto;
    min-height: 0;
  }

  .msgs {
    overflow: auto;
    padding: 1rem;
  }

  .msg {
    margin-bottom: 0.75rem;
    white-space: pre-wrap;
  }

  .msg.user {
    color: var(--accent);
  }

  .msg.assistant {
    color: var(--fg);
  }

  .composer {
    display: flex;
    gap: 0.5rem;
    padding: 0.5rem;
    border-top: 1px solid var(--border);
  }

  .composer input {
    flex: 1;
    padding: 0.35rem;
  }

  .empty {
    padding: 2rem;
    color: var(--dim);
  }
</style>
