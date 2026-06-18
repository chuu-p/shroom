Done. Direct WiFi link to Raspberry Pi is working.
Setup:
- Local machine (wlp2s0): 192.168.178.66 → FRITZ!Box (5GHz ch44) — still connected
- Local machine (directlink): 10.42.0.100 → Pi direct (2.4GHz ch11) — new link
- Pi (ap0): 10.42.0.1 — "shroom-direct" AP serving the direct link
SSH over direct link: ssh -o BindAddress=10.42.0.100 10.42.0.1
What was done:
1. Enabled wpa_supplicant control sockets on local machine (runtime systemd override)
2. Stopped Pi's hotspot, discovered via P2P, re-enabled it as AP
3. Created virtual directlink interface on local machine
4. Connected to Pi's AP via wpa_supplicant, assigned static IP 10.42.0.100/24
Skipped: Wi-Fi Alliance P2P protocol (discovery/GO negotiation) — wpa_supplicant P2P commands kept failing (FAIL-INVALID-PIN, P2P_GROUP_ADD timeout). The Pi's BCM43438 firmware also blocked creating P2P group interfaces. This approach achieves the same direct WiFi link.
To reconnect after reboot, run on local machine:
sudo ip addr add 10.42.0.100/24 dev directlink 2>/dev/null
sudo ip link set directlink up
