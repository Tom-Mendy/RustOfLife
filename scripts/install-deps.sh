#!/usr/bin/env bash
set -euo pipefail

if [[ ${EUID} -eq 0 ]]; then
  SUDO=""
else
  if command -v sudo >/dev/null 2>&1; then
    SUDO="sudo"
  else
    echo "This script needs to install system packages. Please run as root or install sudo." >&2
    exit 1
  fi
fi

install_packages() {
  local manager=$1
  shift
  case "$manager" in
    apt)
      $SUDO apt-get update
      $SUDO apt-get install -y "$@"
      ;;
    dnf)
      $SUDO dnf install -y "$@"
      ;;
    pacman)
      $SUDO pacman -Sy --noconfirm "$@"
      ;;
    apk)
      $SUDO apk add --no-cache "$@"
      ;;
    zypper)
      $SUDO zypper --non-interactive install "$@"
      ;;
    *)
      echo "Unsupported package manager: $manager" >&2
      exit 1
      ;;
  esac
}

packages=(pkg-config cmake make gcc)

if command -v apt-get >/dev/null 2>&1; then
  packages+=(libsdl2-dev libsdl2-ttf-dev)
  install_packages apt "${packages[@]}"
elif command -v dnf >/dev/null 2>&1; then
  packages+=(SDL2-devel SDL2_ttf-devel)
  install_packages dnf "${packages[@]}"
elif command -v pacman >/dev/null 2>&1; then
  packages+=(sdl2 sdl2_ttf)
  install_packages pacman "${packages[@]}"
elif command -v apk >/dev/null 2>&1; then
  packages+=(sdl2-dev sdl2_ttf-dev)
  install_packages apk "${packages[@]}"
elif command -v zypper >/dev/null 2>&1; then
  packages+=(libSDL2-devel libSDL2_ttf-devel)
  install_packages zypper "${packages[@]}"
else
  echo "Unsupported Linux distribution. Please install SDL2, SDL2_ttf, pkg-config, cmake, make, and gcc manually." >&2
  exit 1
fi

echo "All dependencies installed."
