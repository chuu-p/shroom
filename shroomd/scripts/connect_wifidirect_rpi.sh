#!/usr/bin/env bash
set -euo pipefail

RPI_SSH="${1:-raspberrypi}"
SSID="${2:-shroom-direct}"
PASSWORD="${3:-shroomp2p123}"
CHANNEL="${4:-11}"
VIF="wfd0"
VIF_MAC="02:$(openssl rand -hex 5 2>/dev/null | sed 's/\(..\)/\1:/g; s/.$//' || echo '11:22:33:44:55')"

IW() {
  if command -v iw &>/dev/null; then
    sudo iw "$@"
  else
    nix-shell -p iw --run "sudo iw $*"
  fi
}

WPA_CLI() {
  if command -v wpa_cli &>/dev/null; then
    sudo wpa_cli "$@"
  else
    nix-shell -p wpa_supplicant --run "sudo wpa_cli $*"
  fi
}

DHCP_RENEW() {
  local iface="$1"
  if command -v dhcpcd &>/dev/null; then
    sudo dhcpcd -n "$iface" 2>/dev/null
  else
    nix-shell -p dhcpcd --run "sudo dhcpcd -n $iface" 2>/dev/null
  fi
}

echo "=== Setting up RPi hotspot ==="
timeout 30 ssh -tt "$RPI_SSH" "
  set -e
  sudo iw dev ap0 del 2>/dev/null || true
  sudo iw dev wlan0 interface add ap0 type __ap
  sudo ip link set ap0 up
  sudo nmcli connection delete 'Hotspot' 2>/dev/null || true
  sudo nmcli device wifi hotspot ifname ap0 ssid '$SSID' password '$PASSWORD' band bg channel '$CHANNEL'
" 2>&1 | grep -v '^Connection to.*closed' || {
  echo "Failed to configure RPi hotspot via SSH."
  echo "Run these commands manually on the RPi:"
  echo "  sudo iw dev wlan0 interface add ap0 type __ap"
  echo "  sudo ip link set ap0 up"
  echo "  sudo nmcli device wifi hotspot ifname ap0 ssid $SSID password $PASSWORD band bg channel $CHANNEL"
  exit 1
}
echo "RPi hotspot '$SSID' ready"

echo "=== Setting up G14 direct link ==="
IW dev "$VIF" del 2>/dev/null || true
IW dev wlp2s0 interface add "$VIF" type managed addr "$VIF_MAC"
sudo ip link set "$VIF" up
nmcli device set "$VIF" managed no

sudo tee "/tmp/wpa-$VIF.conf" > /dev/null << WEOF
ctrl_interface=/var/run/wpa_supplicant-$VIF
update_config=1
network={
    ssid="$SSID"
    psk="$PASSWORD"
    key_mgmt=WPA-PSK
}
WEOF

sudo wpa_supplicant -D nl80211 -i "$VIF" -c "/tmp/wpa-$VIF.conf" -B

echo -n "waiting for 802.11 association"
for i in $(seq 1 15); do
  if WPA_CLI -i "$VIF" -p "/var/run/wpa_supplicant-$VIF" status 2>/dev/null | grep -q 'wpa_state=COMPLETED'; then
    echo " complete"
    break
  fi
  echo -n "."
  sleep 1
done

DHCP_RENEW "$VIF"

echo -n "waiting for DHCP lease"
for i in $(seq 1 15); do
  if ip addr show "$VIF" | grep -q 'inet '; then
    echo " acquired"
    break
  fi
  echo -n "."
  sleep 1
done

RPI_IP=$(ip -4 -o addr show "$VIF" | awk '{print $4}' | cut -d/ -f1 | sed 's/\.[0-9]*$/.1/')
echo "=== Direct link ready ==="
echo "  RPi (ap0): $RPI_IP"
echo "  G14 ($VIF): $(ip -4 -o addr show "$VIF" | awk '{print $4}')"
echo "  ping: ping -I $VIF $RPI_IP"
echo "  ssh:  ssh chuu@$RPI_IP"
