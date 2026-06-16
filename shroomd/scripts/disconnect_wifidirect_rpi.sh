#!/usr/bin/env bash
set -euo pipefail

RPI_SSH="${1:-raspberrypi}"
VIF="${2:-wfd0}"

IW() {
  if command -v iw &>/dev/null; then
    sudo iw "$@"
  else
    nix-shell -p iw --run "sudo iw $*"
  fi
}

echo "=== Tearing down G14 direct link ==="
sudo pkill -f "wpa_supplicant.*$VIF" 2>/dev/null || true
sudo dhcpcd -k "$VIF" 2>/dev/null || true
IW dev "$VIF" del 2>/dev/null || true
echo "G14 side cleaned up"

echo "=== Tearing down RPi hotspot ==="
timeout 15 ssh -tt "$RPI_SSH" "
  set -e
  sudo nmcli connection down 'Hotspot' 2>/dev/null || true
  sudo nmcli connection delete 'Hotspot' 2>/dev/null || true
  sudo iw dev ap0 del 2>/dev/null || true
  echo 'RPi side cleaned up'
" 2>&1 | grep -v '^Connection to.*closed' || {
  echo "Failed to tear down RPi hotspot via SSH."
  echo "Run these commands manually on the RPi:"
  echo "  sudo nmcli connection down 'Hotspot'; sudo nmcli connection delete 'Hotspot'; sudo iw dev ap0 del"
}

echo "=== Direct link disconnected ==="
