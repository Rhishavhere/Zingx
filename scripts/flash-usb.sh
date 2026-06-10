#!/bin/bash
# Flash Zingx boot payload to USB (Linux/WSL). DESTROYS target disk.
set -euo pipefail

if [ $# -lt 1 ]; then
	echo "Usage: $0 /dev/sdX"
	echo "  WARNING: This wipes the entire disk."
	exit 1
fi

DISK="$1"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PAYLOAD="${ROOT}/artifacts/zingx-usb"

if [ ! -d "${PAYLOAD}" ]; then
	echo "Missing ${PAYLOAD}. Run scripts/build.sh first."
	exit 1
fi

if [ ! -b "${DISK}" ]; then
	echo "Not a block device: ${DISK}"
	exit 1
fi

echo "WARNING: ALL DATA ON ${DISK} WILL BE DESTROYED."
read -r -p "Type YES to continue: " confirm
[ "$confirm" = "YES" ] || exit 1

echo "==> Partitioning ${DISK}..."
parted -s "${DISK}" mklabel gpt
parted -s "${DISK}" mkpart ZINGX_BOOT fat32 1MiB 1024MiB
parted -s "${DISK}" mkpart ZINGX_DATA exfat 1024MiB 100%
parted -s "${DISK}" set 1 esp on

sleep 2
partprobe "${DISK}" 2>/dev/null || true

BOOT="${DISK}1"
DATA="${DISK}2"
[ -b "${BOOT}1" ] && BOOT="${DISK}1" && DATA="${DISK}2"
[ -b "${DISK}p1" ] && BOOT="${DISK}p1" && DATA="${DISK}p2"

echo "==> Formatting boot (${BOOT})..."
mkfs.vfat -F 32 -n ZINGX_BOOT "${BOOT}"

echo "==> Formatting data (${DATA})..."
mkfs.exfat -n ZINGX_DATA "${DATA}"

echo "==> Copying boot payload..."
mount "${BOOT}" /mnt
mkdir -p /mnt/EFI/BOOT
cp -rv "${PAYLOAD}/"* /mnt/
sync
umount /mnt

mkdir -p /mnt
mount "${DATA}" /mnt
mkdir -p /mnt/models /mnt/chats /mnt/notes
sync
umount /mnt

echo "==> USB ready. Optional: create LUKS vault on a third partition later."
echo "    Reboot and select USB boot."
