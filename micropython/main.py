from microbit import *
import utime # Solo necesitamos el módulo utime

DEVICE_ID = "M1"

while True:
    ax = accelerometer.get_x() / 1024
    ay = accelerometer.get_y() / 1024
    az = accelerometer.get_z() / 1024
    tempC = temperature()
    light = display.read_light_level()
    bat = 3.1  # Voltaje simulado
    
    # SOLUCIÓN: Usar utime.ticks_ms() y dividir por 1000 para obtener segundos.
    # Esto reemplaza al problemático utime.time() y genera un timestamp incremental.
    ts = utime.ticks_ms() // 1000 
        
    # Construcción manual de la cadena JSON (evita el módulo "json")
    json_string = (
        '{"id":"' + DEVICE_ID + '",' +
        '"ts":' + str(ts) + ',' +
        '"tempC":' + str(tempC) + ',' +
        '"ax":' + "{:.2f}".format(ax) + ',' + # Formateo simple para evitar demasiados decimales
        '"ay":' + "{:.2f}".format(ay) + ',' +
        '"az":' + "{:.2f}".format(az) + ',' +
        '"light":' + str(light) + ',' +
        '"bat":' + "{:.2f}".format(bat) +
        '}'
    )
    
    print(json_string)
    sleep(500)
