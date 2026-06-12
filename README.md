
## roadmap 

reimplementation of missing features

### bluetooth keyboard

bluetooth keyboard is now working flawlessly! :partying_face:

run ydotooold on hone as sudo

~~~
sudo ydotoold
~~~

receiver on phone

~~~
nc -l -p 5000 | sudo python3 /home/chuu/git/shroom/receiver.py
~~~

sender remote on rpi

~~~
ssh -R 9999:localhost:5000 100.100.204.26 "evtest /dev/input/event3 | nc localhost 9999"
~~~
