### Hi everyone! 
## This is a bridge utility to use the WeCreat Vision 20W laser directly from LightBurn! :partying_face:
The "WeCreat MakeIt!" software is really lacking, and there's no linux version.  Instead of exporting gcode and uploading it through wecreat, unlock useful features like framing and just pressing the button in software (and then the physical one). :)
## [quick demo on youtube](https://youtu.be/16apgy4763g)
> ## TL;DR [github repo](https://github.com/2ktsch/weburn)
> Currently runs on Linux... should compile and work for Windows or Mac.
> You need a null-modem software like tty0tty or com0com
> 1. [Download weburn here (x86_64)](https://github.com/2ktsch/weburn/tree/main/release)
> 2. Run weburn-bridge.sh
> 3. Set up Lightburn with the gcode from WeCreat
> 4. Modify the start gcode by adding "M14S0" on the line before "M14S1"
> 5. Point LightBurn to /dev/ttyS30
> 6. Enjoy!

> **First, some words of caution:** put the device on a private net, sandbox it, whatever.  The security is ABYSMAL.  I had to set up a separate wifi network with downgraded security so that it would even connect...  then checked for open ports and there's 22, 8082 and 8080
> 
> :rotating_light: **It has all the red flags of early IoT devices**
> **22** is ssh (haven't tried/needed to connect)
> **8082** is a directory listing (Mongoose ?) of the entire filesystem... even /etc/shadow is visible, the ssh config is visible and allowing root password login... ancient kernel... etc.
> ****8080**** is where the magic happens :D

# Endpoints on port 8080:
Using our buddy Wireshark, I've found the following endpoints on port 8080:
(Obviously, replace the ip address with the correct one)
### Camera image
(WIP virtual webcam ... another day)
```
curl -X GET "http://192.168.1.15:8080/camera/take_photo"
```
Returns a full frame 4032px × 3024px wide-angle jpeg.
### Camera current calibration data
(There's probably one to run the calibration, I'll check later)
```
curl -X GET "http://192.168.1.15:8080/device/camera/download"
```
Returns JSON:
```
{
  "topData": {
    "points": [
      {
        "x": 823,
        "y": 817
      },
      {
        "x": 1870,
        "y": 813
      },
      {
        "x": 2968,
        "y": 813
      },
      {
        "x": 823,
        "y": 1546
      },
      {
        "x": 1868,
        "y": 1541
      },
      {
        "x": 2963,
        "y": 1537
      },
      {
        "x": 819,
        "y": 2275
      },
      {
        "x": 1867,
        "y": 2267
      },
      {
        "x": 2962,
        "y": 2259
      }
    ],
    "xy": 2145,
    "f": 1795.1331081081082
  },
  "bottomData": {
    "points": [
      {
        "x": 310,
        "y": 509
      },
      {
        "x": 1830,
        "y": 501
      },
      {
        "x": 3422,
        "y": 505
      },
      {
        "x": 307,
        "y": 1567
      },
      {
        "x": 1828,
        "y": 1561
      },
      {
        "x": 3418,
        "y": 1554
      },
      {
        "x": 306,
        "y": 2625
      },
      {
        "x": 1826,
        "y": 2616
      },
      {
        "x": 3413,
        "y": 2599
      }
    ],
    "xy": 3112,
    "f": 1788.558918918919
  },
  "middleData": {
    "points": [
      {
        "x": 611,
        "y": 690
      },
      {
        "x": 1853,
        "y": 686
      },
      {
        "x": 3154,
        "y": 686
      },
      {
        "x": 611,
        "y": 1554
      },
      {
        "x": 1852,
        "y": 1549
      },
      {
        "x": 3149,
        "y": 1544
      },
      {
        "x": 608,
        "y": 2419
      },
      {
        "x": 1850,
        "y": 2409
      },
      {
        "x": 3148,
        "y": 2399
      }
    ],
    "xy": 2543,
    "f": 1784.5674324324323
  },
  "customThickness": 0.35,
  "factoryParameter": {
    "f": 1755.4864864864865,
    "points": [
      {
        "x": 322,
        "y": 481
      },
      {
        "x": 1835,
        "y": 491
      },
      {
        "x": 3415,
        "y": 498
      },
      {
        "x": 313,
        "y": 1534
      },
      {
        "x": 1831,
        "y": 1537
      },
      {
        "x": 3414,
        "y": 1540
      },
      {
        "x": 315,
        "y": 2588
      },
      {
        "x": 1830,
        "y": 2586
      },
      {
        "x": 3409,
        "y": 2581
      }
    ],
    "topData": {
      "points": [
        {
          "x": 821,
          "y": 789
        },
        {
          "x": 1872,
          "y": 797
        },
        {
          "x": 2973,
          "y": 801
        },
        {
          "x": 818,
          "y": 1521
        },
        {
          "x": 1870,
          "y": 1524
        },
        {
          "x": 2970,
          "y": 1527
        },
        {
          "x": 815,
          "y": 2254
        },
        {
          "x": 1869,
          "y": 2253
        },
        {
          "x": 2969,
          "y": 2252
        }
      ],
      "offset": {
        "x": 0,
        "y": 0
      },
      "xy": 2152,
      "f": 1785.5783783783784
    },
    "bottomData": {
      "points": [
        {
          "x": 322,
          "y": 481
        },
        {
          "x": 1835,
          "y": 491
        },
        {
          "x": 3415,
          "y": 498
        },
        {
          "x": 313,
          "y": 1534
        },
        {
          "x": 1831,
          "y": 1537
        },
        {
          "x": 3414,
          "y": 1540
        },
        {
          "x": 315,
          "y": 2588
        },
        {
          "x": 1830,
          "y": 2586
        },
        {
          "x": 3409,
          "y": 2581
        }
      ],
      "offset": {
        "x": 0,
        "y": 0
      },
      "xy": 3093,
      "f": 1755.4864864864865
    },
    "middleData": {
      "points": [
        {
          "x": 608,
          "y": 657
        },
        {
          "x": 1857,
          "y": 667
        },
        {
          "x": 3162,
          "y": 671
        },
        {
          "x": 603,
          "y": 1526
        },
        {
          "x": 1854,
          "y": 1529
        },
        {
          "x": 3159,
          "y": 1532
        },
        {
          "x": 602,
          "y": 2396
        },
        {
          "x": 1852,
          "y": 2394
        },
        {
          "x": 3157,
          "y": 2392
        }
      ],
      "offset": {
        "x": 0,
        "y": 0
      },
      "xy": 2554,
      "f": 1773.9945945945947
    }
  }
}

```
### Auto-focus
The following focuses the laser at x=100mm, y=100mm (offset is for the red laser)
 ```
curl -X GET "http://192.168.1.15:8080/device/camera/measure_distance?x=112&y=118"
```
Returns JSON; "distance" is the value you want to feed LightBurn.  Errors measuring will return 0 for both.
```
{
  "distance": -83.73999786376953,
  "height-z": 16.260000228881836
}
```
### Pause
```
curl -X POST "http://192.168.1.15:8080/process/control?action=0"
```
Returns JSON. To Resume, press the button.
```
{ "code" : 0, "result" : "ok" }
```

### Cancel and return to 0,0,0
```
curl -X POST "http://192.168.1.15:8080/process/control?action=2"
```
Returns JSON
```
{ "code" : 0, "result" : "ok" }
```
### Get current status
```
curl -X POST "http://192.168.1.15:8080/process/status"
```
Returns JSON
```
{ "code": 0, "status": "", "result": 0 }
```
### Send arbitrary gcode to be executed immediately
> :warning: **WARNING** THIS GCODE WILL RUN REGARDLESS OF THE LID POSITION. Please remember to wear safety glasses. :sunglasses: 

```
#!/bin/bash

CONTENT=M27 # replace "M27" with $1 and use a command-line arg!
FILE=$CONTENT\x0a\x0d
SUM=($(echo -n $FILE | md5sum))

echo $FILE | curl --data-binary "@-" -X POST "http://192.168.1.15:8080/test/cmd/mcu?md5=${SUM[0]^^}"
```
Runs the gcode in content (not sure if the \n\r is necessary but whatevs)
If the checksum is wrong, the returned JSON will have a calc_sum field and it won't run.
The checksum has to be uppercase.
### Go to Z pos (instead of auto-focus every time)
Send this gcode using the previous example to focus for a surface at 16mm
```
G0Z-84 
```
Alternatively, just do this in LightBurn...

### Bulk send gcode
This would be the equivalent of "Saving gcode and uploading it through WeCreat MakeIt!"
```
#!/bin/bash

FILE=$1
SUM=($(md5sum $FILE))

cat $FILE | curl --data-binary "@-" -X POST "http://192.168.1.15:8080/process/upload?md5=${SUM[0]^^}"

```
After uploading, you need to press the button so the program will run on the laser.  The usual safety measures work as usual here.
### ...and more! (I'll be back)

# Since I don't wanna be running scripts like this *every time*...
I wrote a little program that acts as a bridge. 

I'll be uploading it to github later in the day.
> To Get it running on my Arch installation, I need to have the tty0tty kernel module loaded, and move /rename of the devices to /dev/ttySX (I'm using 30 but that doesn't really matter)

> The gist of it is: Passes any short commands from LightBurn using the **arbitrary gcode** endpoint, and I added **M14S0** (laser fan off) just before the M14S1 (laser fan on) in the **custom gcode / user start script**. As soon as "**M14S0**" is detected, the program switches into bulk mode, listens to all the gcode until **M2** , uploads it to the **bulk send** endpoint and waits for the machine to return to waiting status, upon which it goes back to passthrough mode.
