ssh 100.100.204.26 'ssh tymo "parec --rate=44100 --format=s16le --channels=2 --device=@DEFAULT_MONITOR@" | pacat --rate=44100 --format=s16le --channels=2'
