#  🐍 Código Fuente del Micro:bit (MicroPython)

Esta sección detalla el código que debe ejecutarse en el micro:bit para generar los datos seriales en formato JSON y el proceso para cargarlo usando el Editor de Python de micro:bit.

##  💻 Código (`main.py`)

Este código lee los sensores de temperatura, luz y acelerómetro del micro:bit, calcula el voltaje de la batería y lo empaqueta todo en una línea JSON que se envía por el puerto serial a **115200 baudios**.

```python
# main.py - Código para el micro:bit
import microbit
import time
import json
import math

# Constante de identificación del dispositivo
DEVICE_ID = "M1"

# Factor de conversión para la batería (asumiendo 3V máx y un divisor resistivo adecuado)
# Nota: La lectura es de 0-1023. Se calibra para obtener un valor aproximado de voltaje.
BAT_VOLTAGE_CONV = 3.3 / 1023.0 
BAT_PIN = microbit.pin1.read_analog

def get_battery_voltage():
    """Lee el pin analógico y lo convierte a un valor aproximado de voltaje."""
    # Usando el pin P1 para la lectura analógica de la batería (debe estar conectado)
    analog_value = BAT_PIN()
    # Retorna un valor aproximado en Volts
    return analog_value * BAT_VOLTAGE_CONV

def read_data():
    """Compila las lecturas de los sensores en un objeto JSON."""
    
    # Lectura del acelerómetro (ejes x, y, z)
    ax = microbit.accelerometer.get_x() / 1000.0 # Convertir a Gs aproximados
    ay = microbit.accelerometer.get_y() / 1000.0
    az = microbit.accelerometer.get_z() / 1000.0
    
    # Compilación del diccionario de datos
    data = {
        "id": DEVICE_ID,
        # Tiempo en milisegundos desde el inicio (para evitar problemas de módulo 'time' en micro:bit)
        "ts": microbit.running_time() // 1000, 
        "tempC": microbit.temperature(),
        "ax": round(ax, 3),
        "ay": round(ay, 3),
        "az": round(az, 3),
        "light": microbit.display.get_light_level(),
        "bat": round(get_battery_voltage(), 2)
    }
    return data

# Bucle principal de ejecución
microbit.display.show(DEVICE_ID[1]) # Muestra el ID en la matriz
while True:
    try:
        sensor_data = read_data()
        
        # Serializa el objeto a una cadena JSON
        json_string = json.dumps(sensor_data)
        
        # Envía la cadena JSON por el puerto serial seguido de un salto de línea
        print(json_string) 
        
    except Exception as e:
        # Manejo simple de errores para evitar que el bucle se detenga
        print("ERROR: Fallo en lectura de sensor:", e)
        
    # Esperar 200ms antes de la próxima lectura
    microbit.sleep(200)

```

-----

##  🚀 Carga y Ejecución (Editor de Micro:bit)

La ejecución se realiza cargando el código directamente a la placa a través del **Editor de Python de micro:bit** y la tecnología **WebUSB**.

| Paso | Instrucción | Notas de IDE |
| :--- | :--- | :--- |
| **1. Conexión** | Abre el [**Editor de Python de micro:bit (V3)**](https://python.microbit.org/v/3) en tu navegador y haz clic en el botón **"Conectar"**. | Asegúrate de que el navegador detecte la placa. |
| **2. Copiar Código** | Pega el código MicroPython de la sección 7.1 en el área de código del editor. | Reemplaza cualquier código existente. |
| **3. Flashear** | Haz clic en el botón **"Flashear"** (ícono de descarga). | El código se transferirá a la placa. La luz de la placa parpadeará. |
| **4. Verificar Serial** | Haz clic en **"Abrir Serial"** (ícono de terminal) para confirmar que los datos JSON se están transmitiendo. | Deberías ver el flujo JSON constante. |
| **5. Liberar Puerto** | **¡IMPORTANTE\!** Haz clic en **"Cerrar Serial"** o desconecta el monitor serial del editor. | El puerto debe estar **libre** para que los programas de tu PC (Python, Rust, etc.) puedan usarlo. |

# Ejecución de codigo en el editor Micro:bit de Python
<img width="1916" height="1078" alt="image" src="https://github.com/user-attachments/assets/ab67a44c-2c8f-4e6c-99fe-9bbd75ecfec5" />
