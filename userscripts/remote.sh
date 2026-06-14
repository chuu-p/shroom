ssh -t -R 9999:localhost:5000 100.100.204.26 "evtest --grab /dev/input/event2 | nc localhost 9999"
