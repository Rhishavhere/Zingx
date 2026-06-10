<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { Terminal } from "@xterm/xterm";
  import { FitAddon } from "@xterm/addon-fit";
  import "@xterm/xterm/css/xterm.css";
  import type { AppMode } from "./lib/types";
  import ChatsPane from "./lib/ChatsPane.svelte";
  import UtilsPane from "./lib/UtilsPane.svelte";
  import HostPane from "./lib/HostPane.svelte";

  let mode: AppMode = $state("shell");
  let browserUrl = $state("https://duckduckgo.com");
  let urlInput = $state(browserUrl);
  let focusBrowser = $state(false);
  let termEl: HTMLDivElement;
  let term: Terminal;
  let fitAddon: FitAddon;

  const modes: { id: AppMode; label: string; key: string }[] = [
    { id: "shell", label: "shell", key: "1" },
    { id: "chats", label: "chats", key: "2" },
    { id: "utils", label: "utils", key: "3" },
    { id: "host", label: "host", key: "4" },
  ];

  function setMode(m: AppMode) {
    mode = m;
  }

  function navigate() {
    browserUrl = urlInput.trim() || "about:blank";
  }

  onMount(() => {
    term = new Terminal({
      cursorBlink: true,
      theme: {
        background: "#0a0a0a",
        foreground: "#c8ffc8",
        cursor: "#00ff66",
      },
      fontFamily: "IBM Plex Mono, monospace",
      fontSize: 14,
    });
    fitAddon = new FitAddon();
    term.loadAddon(fitAddon);
    term.open(termEl);
    fitAddon.fit();

    term.onData((data) => {
      invoke("pty_write", { data }).catch(console.error);
    });

    invoke("pty_spawn").catch(() => {
      term.writeln("\x1b[33m[dev] PTY unavailable\x1b[0m");
    });

    const unlisten = listen<string>("pty-output", (ev) => {
      term.write(ev.payload);
    });

    const onKey = (e: KeyboardEvent) => {
      if (e.altKey && e.key >= "1" && e.key <= "4") {
        e.preventDefault();
        setMode(modes[Number(e.key) - 1].id);
      }
      if (e.key === "Escape") setMode("shell");
      if (mode === "shell" && e.altKey && e.key === "ArrowRight") {
        e.preventDefault();
        focusBrowser = true;
      }
      if (mode === "shell" && e.altKey && e.key === "ArrowLeft") {
        e.preventDefault();
        focusBrowser = false;
        term.focus();
      }
    };
    window.addEventListener("keydown", onKey);
    window.addEventListener("resize", () => fitAddon.fit());

    return () => {
      unlisten.then((fn) => fn());
      window.removeEventListener("keydown", onKey);
      term.dispose();
    };
  });
</script>

<div class="zingx">
  <header class="bar">
    <span class="logo">ZINGX</span>
    <span class="status">vault ● wifi ●</span>
  </header>

  <main class="body">
    {#if mode === "shell"}
      <section class="split" class:focus-right={focusBrowser}>
        <div class="pane terminal-pane">
          <div class="pane-label">terminal</div>
          <div class="term" bind:this={termEl}></div>
        </div>
        <div class="pane browser-pane">
          <div class="pane-label">internet</div>
          <form class="urlbar" onsubmit={(e) => { e.preventDefault(); navigate(); }}>
            <input bind:value={urlInput} spellcheck="false" />
            <button type="submit">go</button>
          </form>
          <iframe title="browser" src={browserUrl} sandbox="allow-scripts allow-same-origin allow-forms allow-popups"></iframe>
        </div>
      </section>
    {:else if mode === "chats"}
      <ChatsPane />
    {:else if mode === "utils"}
      <UtilsPane />
    {:else}
      <HostPane />
    {/if}
  </main>

  <footer class="dock">
    {#each modes as m}
      <button class:active={mode === m.id} onclick={() => setMode(m.id)}>
        [{m.key}] {m.label}
      </button>
    {/each}
    <span class="hint">Alt+1–4 · Esc → shell · Alt+←/→ focus</span>
  </footer>
</div>

<style>
  .zingx {
    display: grid;
    grid-template-rows: auto 1fr auto;
    height: 100%;
  }

  .bar {
    display: flex;
    justify-content: space-between;
    padding: 0.5rem 1rem;
    border-bottom: 1px solid var(--border);
    letter-spacing: 0.2em;
    font-size: 0.75rem;
  }

  .logo {
    color: var(--accent);
  }

  .status {
    color: var(--dim);
  }

  .body {
    min-height: 0;
  }

  .split {
    display: grid;
    grid-template-columns: 1fr 1fr;
    height: 100%;
  }

  .pane {
    display: grid;
    grid-template-rows: auto auto 1fr;
    min-height: 0;
    border-right: 1px solid var(--border);
  }

  .browser-pane {
    border-right: none;
  }

  .pane-label {
    padding: 0.35rem 0.75rem;
    font-size: 0.7rem;
    color: var(--dim);
    text-transform: uppercase;
    border-bottom: 1px solid var(--border);
  }

  .term {
    padding: 0.25rem;
    min-height: 0;
    height: 100%;
  }

  .urlbar {
    display: flex;
    gap: 0.5rem;
    padding: 0.35rem;
    border-bottom: 1px solid var(--border);
  }

  .urlbar input {
    flex: 1;
    padding: 0.25rem 0.5rem;
  }

  iframe {
    width: 100%;
    height: 100%;
    border: 0;
    background: #000;
  }

  .focus-right .terminal-pane {
    opacity: 0.65;
  }

  .dock {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    padding: 0.5rem;
    border-top: 1px solid var(--border);
  }

  .dock button {
    padding: 0.25rem 0.75rem;
    font-size: 0.75rem;
  }

  .hint {
    margin-left: auto;
    color: var(--dim);
    font-size: 0.65rem;
  }
</style>
