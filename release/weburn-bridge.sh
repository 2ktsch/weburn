#!/bin/bash

# This program currently requires tty0tty or any null modem (virtual) cable, loaded as a kernel module (the latest versions of LightBurn don't see my /dev/tntX ports on Arch :c )

if lsmod | grep -wq "tty0tty"; then
  echo "tty0tty is loaded!"
else
  echo "need to load kernel modules tty0tty and v4l2loopback"
  sudo modprobe tty0tty
  # comment this out if no need for camera or using OBS Studio:
  sudo modprobe v4l2loopback
fi
sleep 1
echo "need to rename null modem device /dev/tnt0 -> /dev/ttyS30"

# this will move the virtual serial port (the end that you connect to LightBurn) to a port name that LightBurn can see
sudo mv /dev/tnt0 /dev/ttyS30
sleep 1

# WeCreat Vision ip address (or hostname) -- preferably set to a static address by your router
export LASER_IP=192.168.1.15

# comment this next block out if no need for camera (but leave it if using OBS Studio)
. ./camera/.venv/bin/activate
export LASER_PORT=8080
export FPS=15
export IMAGE_ENDPOINT=camera/take_photo
python3 camera/main.py &

# The other end of the null modem
export SERIAL_PORT=/dev/tnt1 
# export DEBUG=false
# export AUTO_CANCEL_BULK_END=false

./weburn
