# shroomd

shroom daemon

use nix develop --command cargo check to check if it works

## tasks

- [ ] add tokio async
- [ ] spawn 3 long running tasks, and restart them when they end and capture their output streamed
- [ ] add the first job that has to be running - sudo ydotoold
- [ ] add the second job that has to be running - ../userscripts/receiver.sh (reimplement the functionality in rust, the shell script and python script)
- [ ] add the third job that has to be running - ../userscripts/remote.sh
- [ ] add systemd service definition file for this binary
-
