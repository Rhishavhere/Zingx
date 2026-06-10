#!/bin/bash
# Build Zingx OS image (run inside WSL2 or Linux)
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BUILDROOT_VERSION="${BUILDROOT_VERSION:-2024.02.6}"
BUILDROOT_DIR="${ROOT}/buildroot"
OUTPUT="${ROOT}/artifacts"
JOBS="${JOBS:-$(nproc)}"

echo "==> Zingx OS build"
echo "    root: ${ROOT}"
echo "    buildroot: ${BUILDROOT_VERSION}"

if [ ! -d "${BUILDROOT_DIR}" ]; then
	echo "==> Cloning Buildroot ${BUILDROOT_VERSION}..."
	git clone --depth 1 --branch "${BUILDROOT_VERSION}" \
		https://github.com/buildroot/buildroot.git "${BUILDROOT_DIR}"
fi

export BR2_EXTERNAL="${ROOT}/os"
cd "${BUILDROOT_DIR}"

make BR2_EXTERNAL="${BR2_EXTERNAL}" zingx_x86_64_defconfig
make BR2_EXTERNAL="${BR2_EXTERNAL}" -j"${JOBS}"

mkdir -p "${OUTPUT}"
rm -rf "${OUTPUT}/zingx-usb"
cp -a "${BUILDROOT_DIR}/output/images/zingx-usb" "${OUTPUT}/"

echo ""
echo "==> Done. USB payload: ${OUTPUT}/zingx-usb"
ls -la "${OUTPUT}/zingx-usb"
