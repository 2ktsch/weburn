#!/bin/bash

# This program currently requires tty0tty or any null modem (virtual) cable, loaded as a kernel module (the latest versions of LightBurn don't see my /dev/tntX ports on Arch :c )

if lsmod | grep -wq "tty0tty"; then
  echo "tty0tty is loaded!"
else
  echo "need to load kernel module tty0tty"
  sudo modprobe tty0tty
fi

echo "need to rename null modem device /dev/tnt0 -> /dev/ttyS30"
if [[ -f /dev/tnt0 ]] && [[ -f /dev/tnt1 ]]; then
  sudo mv /dev/tnt0 /dev/ttyS30
elif [[ ! -f /dev/tnt0 ]]; then
    echo "/dev/tnt0 seems to be renamed already, cool."
else
  echo "don't know what's up with the serial ports... exiting"
  exit 1
fi


# WeCreat Vision ip address (or hostname) -- preferably set to a static address by your router
export LASER_IP=192.168.1.15

# This is pretty much static but configurable just in case... 
# export LASER_PORT=8080 

# The other end of the null modem
export SERIAL_PORT=/dev/tnt1 
# export DEBUG=false
# export AUTO_CANCEL_BULK_END=false

./weburn
