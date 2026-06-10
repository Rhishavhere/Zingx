<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { Webview } from "@tauri-apps/api/webview";
  import { LogicalPosition, LogicalSize } from "@tauri-apps/api/dpi";

  interface Props {
    url: string;
    visible?: boolean;
  }

  let { url, visible = true }: Props = $props();

  let hostEl: HTMLDivElement;
  let webview: Webview | null = null;
  let currentUrl = "";

  function normalizeUrl(raw: string): string {
    const trimmed = raw.trim();
    if (!trimmed) return "about:blank";
    if (/^https?:\/\//i.test(trimmed)) return trimmed;
    return `https://${trimmed}`;
  }

  async function bounds() {
    if (!hostEl) return null;
    const rect = hostEl.getBoundingClientRect();
    if (rect.width < 8 || rect.height < 8) return null;
    return {
      x: Math.round(rect.left),
      y: Math.round(rect.top),
      width: Math.round(rect.width),
      height: Math.round(rect.height),
    };
  }

  async function syncWebview(nextUrl?: string) {
    if (!visible || !hostEl) {
      await webview?.hide();
      return;
    }

    const b = await bounds();
    if (!b) return;

    const target = normalizeUrl(nextUrl ?? url);

    if (!webview) {
      const win = getCurrentWindow();
      webview = new Webview(win, "internet", {
        url: target,
        x: b.x,
        y: b.y,
        width: b.width,
        height: b.height,
        focus: false,
      });
      webview.once("tauri://error", (e) => {
        console.error("browser webview error", e);
      });
      currentUrl = target;
      return;
    }

    await webview.setPosition(new LogicalPosition(b.x, b.y));
    await webview.setSize(new LogicalSize(b.width, b.height));
    await webview.show();

    if (target !== currentUrl) {
      await webview.close();
      webview = null;
      currentUrl = "";
      await syncWebview(target);
    }
  }

  $effect(() => {
    if (url && visible) {
      syncWebview(url).catch(console.error);
    } else if (!visible) {
      webview?.hide().catch(console.error);
    }
  });

  onMount(() => {
    const ro = new ResizeObserver(() => {
      syncWebview().catch(console.error);
    });
    ro.observe(hostEl);
    window.addEventListener("resize", onResize);
    syncWebview().catch(console.error);

    return () => {
      ro.disconnect();
      window.removeEventListener("resize", onResize);
    };
  });

  function onResize() {
    syncWebview().catch(console.error);
  }

  onDestroy(() => {
    webview?.close().catch(console.error);
    webview = null;
  });
</script>

<div class="browser-host" bind:this={hostEl}></div>

<style>
  .browser-host {
    width: 100%;
    height: 100%;
    min-height: 0;
    background: #000;
  }
</style>
