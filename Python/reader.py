# Archivo: python/reader.py
import serial
import json
import time
import math

# !!! CAMBIA "COM3" por tu puerto serial !!!
SERIAL_PORT = 'COM15'
BAUD_RATE = 115200 

def check_alerts(data):
    alerts = []
    try:
        mag = math.sqrt(data['ax']**2 + data['ay']**2 + data['az']**2)
        if mag > 1.5: alerts.append(f"🚨 Movimiento brusco (mag: {mag:.2f}g)")
        if data['tempC'] > 30: alerts.append(f"🌡️ Alta temperatura ({data['tempC']:.1f}°C)")
        if data['light'] < 20: alerts.append(f"🌑 Baja luz ({data['light']})")
        if data['bat'] < 3.0: alerts.append(f"🔋 Batería baja ({data['bat']:.2f}V)")
        return " | ".join(alerts) if alerts else "✅ Todo OK"
    except KeyError as e:
        return f"Error: Falta la clave {e} en JSON"

print(f"Intentando conectar a {SERIAL_PORT}...")
try:
    ser = serial.Serial(SERIAL_PORT, BAUD_RATE, timeout=1)
    print(f"Conectado. Esperando datos... (Presiona Ctrl+C para salir)")

    while True:
        line = ser.readline()
        if line:
            try:
                data_str = line.decode('utf-8').strip()
                # --- Corrección de error (Paso 7) ---
                if not data_str: 
                    continue # Ignorar líneas vacías
                # -------------------------------------

                data = json.loads(data_str)
                alerts_msg = check_alerts(data)
                print(f"[{data['id']}] Temp: {data['tempC']}°C | {alerts_msg}")

            except json.JSONDecodeError: print(f"Error JSON en línea: {line}")
            except UnicodeDecodeError: print(f"Error de codificación en línea: {line}")

except serial.SerialException as e:
    print(f"Error al abrir el puerto {SERIAL_PORT}: {e}")
    print("Verifica el nombre del puerto y que Mu Editor esté cerrado.")
except KeyboardInterrupt:
    print("\nDesconectando...")
    ser.close()
