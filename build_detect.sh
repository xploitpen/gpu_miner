#!/usr/bin/env bash
# build_detect.sh
# developed by bekmen_my
set -euo pipefail

echo "[banner] developed by bekmen_my"
OS="$(uname -s)"
FEATURE="cpu"
GPU_VENDOR="unknown"

is_cmd() { command -v "$1" >/dev/null 2>&1; }

detect_linux() {
  local has_nvidia=0 has_amd=0 has_vulkan=0 has_gles=0 has_opencl=0 qualcomm=0
  if is_cmd nvidia-smi; then has_nvidia=1; fi
  if is_cmd rocm-smi || lspci | grep -qi 'AMD'; then has_amd=1; fi
  if is_cmd vulkaninfo || ldconfig -p | grep -qi libvulkan; then has_vulkan=1; fi
  if is_cmd glxinfo || pkg-config --exists glesv2; then has_gles=1; fi
  if is_cmd clinfo || ldconfig -p | grep -qi 'libOpenCL'; then has_opencl=1; fi
  if is_cmd clinfo && clinfo | grep -qi 'QUALCOMM'; then qualcomm=1; GPU_VENDOR="qualcomm"; fi

  if [ $has_nvidia -eq 1 ]; then GPU_VENDOR="nvidia"
    if is_cmd nvcc || is_cmd nvidia-smi; then FEATURE="cuda"; else FEATURE="opencl"; fi
  elif [ $has_amd -eq 1 ]; then GPU_VENDOR="amd"
    if [ $has_opencl -eq 1 ]; then FEATURE="opencl"; elif [ $has_vulkan -eq 1 ]; then FEATURE="vulkan"; else FEATURE="cpu"; fi
  elif [ $qualcomm -eq 1 ]; then
    if [ $has_vulkan -eq 1 ]; then FEATURE="vulkan"; elif [ $has_opencl -eq 1 ]; then FEATURE="opencl"; elif [ $has_gles -eq 1 ]; then FEATURE="gles"; else FEATURE="cpu"; fi
  elif [ $has_vulkan -eq 1 ]; then GPU_VENDOR="generic"; FEATURE="vulkan"
  elif [ $has_opencl -eq 1 ]; then GPU_VENDOR="generic"; FEATURE="opencl"
  elif [ $has_gles -eq 1 ]; then GPU_VENDOR="generic"; FEATURE="gles"
  else GPU_VENDOR="none"; FEATURE="cpu"; fi
}

detect_android_termux() {
  local has_vulkan=0 has_opencl=0
  if is_cmd vulkaninfo; then has_vulkan=1; fi
  if is_cmd clinfo; then has_opencl=1; fi
  GPU_VENDOR="mobile"
  if [ $has_vulkan -eq 1 ]; then FEATURE="vulkan"
  elif [ $has_opencl -eq 1 ]; then FEATURE="opencl"
  else FEATURE="gles"; fi
}

detect_windows() {
  if command -v nvidia-smi.exe >/dev/null 2>&1; then GPU_VENDOR="nvidia"; FEATURE="cuda"
  elif wmic path win32_videocontroller get name | findstr /I "AMD" >/dev/null; then GPU_VENDOR="amd"; FEATURE="opencl"
  elif command -v vulkaninfo.exe >/dev/null 2>&1; then GPU_VENDOR="generic"; FEATURE="vulkan"
  else GPU_VENDOR="generic"; FEATURE="cpu"; fi
}

case "$OS" in
  Linux)
    if command -v getprop >/dev/null 2>&1; then detect_android_termux; else detect_linux; fi
    ;;
  MINGW*|MSYS*|CYGWIN*|Windows_NT)
    detect_windows
    ;;
  *)
    detect_linux
    ;;
esac

echo "[detect] GPU_VENDOR=$GPU_VENDOR | feature=$FEATURE"
echo "[build] cargo build --release --features $FEATURE"
cargo build --release --features "$FEATURE"

echo "[run] launching miner..."
./target/release/bekmen_miner "$@"
