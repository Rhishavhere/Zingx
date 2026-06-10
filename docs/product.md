# Product Requirements Document (PRD): Zingx

## 1. Executive Summary
Zingx is an open-source, hyper-lean, single-purpose "Appliance OS" designed to run entirely out of RAM from a standard USB 3.0 drive or external SSD. It bypasses traditional desktop metaphors to provide an immediate, distraction-free, keyboard-driven workspace: a real terminal plus internet on boot, with optional modes for persistent chats, utilities, and host file access. It leaves no unnecessary trace on the host computer.

## 2. Architecture & File System Topology
The USB drive (Target: 64GB) uses three partitions:

* **Partition 1 (Boot & Core OS - ~1GB, FAT32):** GRUB2, Linux kernel, compressed `rootfs.squashfs`.
* **Partition 2 (Data Dropzone - ~55GB, exFAT):** Visible on Windows/macOS. LLM models, markdown notes, saved chat threads.
* **Partition 3 (Secure Enclave - ~8GB, LUKS ext4):** Browser cookies, session state, cryptographic keys.

### Execution Lifecycle
1. Cold boot via BIOS/UEFI → core OS loads from squashfs into RAM (`quiet loglevel=0`).
2. OverlayFS merges writable changes from Partition 2 into the RAM session.
3. LUKS unlock maps Partition 3 to the browser profile path.
4. Cage (Wayland kiosk) launches the Tauri UI fullscreen.

## 3. UI & Interaction Model

### Shell Mode (default on boot)
Split view after vault unlock:

| Left | Right |
|------|-------|
| Real terminal (PTY shell: `ls`, `cd`, `nano`, …) | Embedded browser (sessions in vault) |

* **`@zingx <prompt>`** in the terminal → one-shot Ollama response streamed inline. No history saved.
* **Chats mode** (dock) → persistent streaming chat threads saved to `/mnt/data/chats/`.

### Dock / Mode Panel
Keyboard-first strip (Alt+1–4):

| Key | Mode | Purpose |
|-----|------|---------|
| 1 | **Shell** | Terminal + Internet (default) |
| 2 | **Chats** | Saved LLM threads |
| 3 | **Utils** | Notes, Ollama status, Wi-Fi/vault info |
| 4 | **Host** | Gallery viewer, read-only host mount, QR file server |

Additional bindings: Alt+←/→ focus panes (Shell), Ctrl+` toggle terminal in other modes, Esc → Shell.

### Boot Experience
Black background, retro typography: `Welcome Rhishav. Decrypting vault...`. No kernel log spam.

## 4. Core Services
* **Ollama:** headless on `localhost:11434`, models on Partition 2 (`gemma3:1b` default).
* **Mobile Gallery (Host mode):** Rust/axum local server + QR code for phone access over Wi-Fi.
* **Tech stack:** Buildroot (x86_64, musl), Wayland + Cage, Tauri v2 + Svelte, Ollama.

## 5. Development vs Production Paths
| Path | Dev (Windows/Linux) | Production (Zingx USB) |
|------|---------------------|------------------------|
| `ZINGX_DATA` | `./dev-data` | `/mnt/data` |
| `ZINGX_VAULT` | `./dev-vault` | `/mnt/vault` |
| `OLLAMA_HOST` | `http://127.0.0.1:11434` | `http://127.0.0.1:11434` |

Daily UI work: `npm run tauri dev` in `app/`. OS integration: Buildroot in WSL2, QEMU or USB flash via `scripts/`.
