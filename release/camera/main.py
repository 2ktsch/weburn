from PIL import Image
import urllib.request as req
import pyvirtualcam
import numpy as np
import os

laser_ip = os.environ.get("LASER_IP", "192.168.1.15")
laser_port = os.environ.get("LASER_PORT", "8080")
image_endpoint = os.environ.get("IMAGE_ENDPOINT", "camera/take_photo")
fps = os.environ.get("FPS", "15") # this will just get them as fast as the connection and camera can provide... real will be around 3-4

''' This is kind of useless... Better to just use LightBurn's built-in calibration and alignment tools. '''
adjust_image = os.environ.get("ADJUST_IMAGE", "false")
if adjust_image == "true":
    calibration_endpoint = os.environ.get("CALIBRATION_ENDPOINT", "device/camera/download")
    curl = f"http://{laser_ip}:{laser_port}/{calibration_endpoint}"
    print(f"Using calibration from url: {curl}")
    calibration = req.urlopen(curl)
    calibration = calibration.read().decode("utf-8")
    print(calibration)

def adjust(image):
    return image

def check_module():
    module_loaded = len(os.popen("lsmod | grep v4l2loopback").readlines())
    return module_loaded > 0

def main(laser_ip, laser_port, image_endpoint, adjust_image, fps):
    print(f"Initializing cam at {laser_ip} at {fps} fps.")
    if check_module():
        with pyvirtualcam.Camera(width=4032, height=3024, fps=float(fps)) as cam:
            print(f'Using virtual camera: {cam.device}')
            url = f"http://{laser_ip}:{laser_port}/{image_endpoint}"
            print(f'Getting frames from {url}')
            frame = np.zeros((cam.height, cam.width, 3), np.uint8)  # RGB
            count = 0
            while True:
                try:
                    with req.urlopen(url) as im:
                        image = Image.open(im)
                        # image.show()
                        if adjust_image == "true":
                            print("Using the calibration isn't implemented yet! Sending raw frame")
                            frame = adjust(np.array(image))
                        else:
                            # print('.', end='')
                            frame = np.array(image)
                        print(str(count)+"                                         ", end="\r")
                except:
                    print(f"couldn't get frame #{count}, sending blank                       ", end="\r") if count%int(fps) == 0 else None
                    pass
                cam.send(frame)
                count += 1
                cam.sleep_until_next_frame()
    else:
        print("please load v4l2loopback module!")


if __name__ == '__main__':
    main(laser_ip, laser_port, image_endpoint, adjust_image, fps)