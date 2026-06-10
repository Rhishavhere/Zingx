# Zingx

Personal homelab-in-a-pocket OS — boots from USB, runs in RAM, terminal + internet by default.

## Quick start (UI dev on Windows)

```powershell
cd app
npm install
npm run tauri:dev
```

Requires [Ollama](https://ollama.com) for `@zingx` and Chats mode:

```powershell
ollama pull gemma3:1b
```

Dev data dirs (`dev-data/`, `dev-vault/`) are created automatically.

## USB boot (fastest path)

Buildroot runs in **WSL2** (Ubuntu recommended):

```bash
# In WSL
sudo apt update
sudo apt install -y build-essential git unzip wget cpio rsync bc libncurses-dev \
  python3 python3-pip file patch bash sed tar gzip flex bison gawk \
  libssl-dev libelf-dev diffutils

cd /mnt/e/CodeRepo/Zingx   # adjust drive letter
bash scripts/build.sh
```

First build downloads Buildroot and takes **30–90 minutes**.

### Flash USB (WSL, destructive)

```bash
lsblk                        # find /dev/sdX (USB stick)
bash scripts/flash-usb.sh /dev/sdX
```

### Flash USB (Windows)

After `scripts/build.sh`, copy artifacts from WSL:

```powershell
# Artifacts at E:\CodeRepo\Zingx\artifacts\zingx-usb
.\scripts\prepare-usb.ps1 -DriveLetter E
```

### Boot

1. Insert USB, reboot
2. Enter UEFI boot menu → select USB
3. Disable Secure Boot if needed
4. You should see `Zingx booting...` then a login shell on tty1
5. Type `ls /mnt/data` after partitions mount

## Repo layout

```
app/        Tauri + Svelte UI (daily development)
os/         Buildroot external tree
scripts/    build.sh, flash-usb.sh, prepare-usb.ps1
docs/       product PRD
artifacts/  build output (gitignored)
```

## Roadmap

- [x] Buildroot boot chain (initramfs → squashfs from ZINGX_BOOT)
- [x] Tauri shell UI (terminal, browser, dock modes)
- [ ] Cage kiosk autostart with zingx-app
- [ ] Ollama in OS image + GPU
- [ ] LUKS vault partition setup script
- [ ] Host gallery (axum + QR)
