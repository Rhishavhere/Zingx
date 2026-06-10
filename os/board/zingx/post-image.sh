#!/bin/sh
set -e

BOARD_DIR="$(dirname "$0")"
ARTIFACTS="${BINARIES_DIR}/zingx-usb"
GRUB_CFG="${BOARD_DIR}/grub.cfg.in"

rm -rf "${ARTIFACTS}"
mkdir -p "${ARTIFACTS}/EFI/BOOT"

cp -f "${BINARIES_DIR}/bzImage" "${ARTIFACTS}/bzImage"
cp -f "${BINARIES_DIR}/rootfs.squashfs" "${ARTIFACTS}/rootfs.squashfs"

# UEFI GRUB
if [ -f "${BINARIES_DIR}/efi-part/bootx64.efi" ]; then
	cp -f "${BINARIES_DIR}/efi-part/bootx64.efi" "${ARTIFACTS}/EFI/BOOT/BOOTX64.EFI"
elif [ -f "${HOST_DIR}/share/grub/x86_64-efi/bootx64.efi" ]; then
	cp -f "${HOST_DIR}/share/grub/x86_64-efi/bootx64.efi" "${ARTIFACTS}/EFI/BOOT/BOOTX64.EFI"
fi

sed 's|@KERNEL@|/bzImage|g; s|@INITRD@||g; s|@CMDLINE@|quiet loglevel=0 rdinit=/init|g' \
	"${GRUB_CFG}" > "${ARTIFACTS}/EFI/BOOT/grub.cfg"

# Legacy BIOS GRUB (optional USB PCs)
mkdir -p "${ARTIFACTS}/boot/grub"
cp -f "${ARTIFACTS}/EFI/BOOT/grub.cfg" "${ARTIFACTS}/boot/grub/grub.cfg"

echo "Zingx USB artifacts -> ${ARTIFACTS}"
ls -la "${ARTIFACTS}"
