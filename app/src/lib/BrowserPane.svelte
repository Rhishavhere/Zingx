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
  let webviewReady = false;
  let syncing = false;

  function normalizeUrl(raw: string): string {
    const trimmed = raw.trim();
    if (!trimmed) return "about:blank";
    
    // Check if it's a URL
    const isUrl = /^(https?:\/\/)?([a-zA-Z0-9-]+\.)+[a-zA-Z]{2,}(:\d+)?(\/.*)?$/i.test(trimmed) || 
                  /^(https?:\/\/)?localhost(:\d+)?(\/.*)?$/i.test(trimmed) ||
                  /^(https?:\/\/)?\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}(:\d+)?(\/.*)?$/i.test(trimmed);
                  
    if (isUrl) {
      if (/^https?:\/\//i.test(trimmed)) return trimmed;
      return `https://${trimmed}`;
    }
    
    // Otherwise, treat as search query
    return `https://duckduckgo.com/?q=${encodeURIComponent(trimmed)}`;
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
    if (syncing) {
      console.log("[BrowserPane] Already syncing, skipping");
      return;
    }
    
    syncing = true;
    try {
      if (!visible || !hostEl) {
        if (webview && webviewReady) {
          await webview.hide();
        }
        return;
      }

      const b = await bounds();
      if (!b) {
        console.log("[BrowserPane] No bounds available yet");
        return;
      }

      const target = normalizeUrl(nextUrl ?? url);
      console.log("[BrowserPane] syncWebview target:", target, "bounds:", b);

      if (!webview) {
        const win = getCurrentWindow();
        const label = "internet-" + Math.random().toString(36).substring(2, 9);
        console.log("[BrowserPane] Creating webview with label:", label);
        
        webviewReady = false;
        const newWebview = new Webview(win, label, {
          url: target,
          x: b.x,
          y: b.y,
          width: b.width,
          height: b.height,
          focus: false,
        });
        
        // Wait for the webview to be ready before storing it
        await new Promise<void>((resolve, reject) => {
          const timeout = setTimeout(() => {
            reject(new Error("Webview creation timeout"));
          }, 5000);
          
          newWebview.once("tauri://created", () => {
            clearTimeout(timeout);
            console.log("[BrowserPane] Webview created successfully");
            webview = newWebview;
            webviewReady = true;
            currentUrl = target;
            resolve();
          });
          
          newWebview.once("tauri://error", (e) => {
            clearTimeout(timeout);
            console.error("[BrowserPane] Webview creation error:", e);
            reject(e);
          });
        });
        
        return;
      }

      if (!webviewReady) {
        console.log("[BrowserPane] Webview not ready yet");
        return;
      }

      if (target !== currentUrl) {
        console.log("[BrowserPane] URL changed, recreating webview");
        webviewReady = false;
        const oldWebview = webview;
        webview = null;
        currentUrl = "";
        oldWebview.close().catch(console.error);
        syncing = false; // Allow next call to proceed
        await syncWebview(target);
        return;
      }

      console.log("[BrowserPane] Updating existing webview position/size");
      await webview.setPosition(new LogicalPosition(b.x, b.y));
      await webview.setSize(new LogicalSize(b.width, b.height));
      await webview.show();
    } catch (error) {
      console.error("[BrowserPane] syncWebview error:", error);
      webview = null;
      webviewReady = false;
      currentUrl = "";
    } finally {
      syncing = false;
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
    console.log("[BrowserPane] onMount, initial url:", url);
    
    const ro = new ResizeObserver(() => {
      syncWebview().catch(console.error);
    });
    ro.observe(hostEl);
    window.addEventListener("resize", onResize);
    
    // Delay initial sync to ensure layout is ready
    setTimeout(() => {
      console.log("[BrowserPane] Delayed initial sync");
      syncWebview().catch(console.error);
    }, 200);

    return () => {
      ro.disconnect();
      window.removeEventListener("resize", onResize);
    };
  });

  function onResize() {
    syncWebview().catch(console.error);
  }

  onDestroy(() => {
    if (webview && webviewReady) {
      webview.close().catch(console.error);
    }
    webview = null;
    webviewReady = false;
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
