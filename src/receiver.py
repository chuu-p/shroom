#!/usr/bin/env python3

import re
import subprocess
import sys

for line in sys.stdin:
    m = re.search(r'code ([0-9]+).*value ([012])', line)
    if not m:
        continue

    code = m.group(1)
    value = m.group(2)

    if value == "1":
        subprocess.run(["/home/chuu/.nix-profile/bin/ydotool", "key", f"{code}:1"])
    elif value == "0":
        subprocess.run(["/home/chuu/.nix-profile/bin/ydotool", "key", f"{code}:0"])
