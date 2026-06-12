# shroom

i am running sxmo postmarketos on a oneplus 5t

many features are working out of the box but some are not working.

two major features that are not working is bluetooth and usb otg. this means i cannot connect a keyboard to this device, which i need.

## roadmap 

reimplementation of missing features

### bluetooth keyboard

i am using a raspberry pi zero 2 w as a companion device, since it has working bluetooth. the pi connects to the phone via ssh and netcats the keypresses to the phone.

bluetooth keyboard is now working flawlessly! :partying_face:

1. run ydotooold on phone as sudo

~~~
sudo ydotoold
~~~

2. receiver on phone

~~~
nc -l -p 5000 | sudo python3 /home/chuu/git/shroom/receiver.py
~~~

3. sender remote on rpi

~~~
ssh -R 9999:localhost:5000 100.100.204.26 "evtest /dev/input/event3 | nc localhost 9999"
~~~

### audio

audio forwarding is also working! the pi sits between the phone and my desktop (or headphones) as a relay. audio from the phone is captured via pipewire's monitor source, forwarded over ssh to the pi, then forwarded again over ssh to the destination.

1. run `audio.sh` on the destination machine (desktop or headphones connected to pi):

~~~
./audio.sh
~~~

the script pipes `parec` from the phone's audio monitor source through two ssh hops into the local `pacat`.
