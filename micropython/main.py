from microbit import *
import utime, json
DEVICE_ID = "M1"
while True:
    ax = accelerometer.get_x() / 1024
    ay = accelerometer.get_y() / 1024
    az = accelerometer.get_z() / 1024
    tempC = temperature()
    light = display.read_light_level()
    bat = 3.1  # Voltaje simulado

    data = {
        "id": DEVICE_ID,
        "ts": utime.time(),
        "tempC": tempC,
        "ax": ax,
        "ay": ay,
        "az": az,
        "light": light,
        "bat": bat
    }

    print(json.dumps(data))
    sleep(500)
