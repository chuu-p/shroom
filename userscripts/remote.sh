ssh -R 9999:localhost:5000 100.100.204.26 "evtest /dev/input/event3 | nc localhost 9999"
