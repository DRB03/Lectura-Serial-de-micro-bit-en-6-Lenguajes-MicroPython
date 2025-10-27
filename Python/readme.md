## 1\. 🐍 Python (`python/reader.py`)

Python es ideal por su facilidad de uso con librerías seriales y JSON.

### 🛠️ Configuración (Terminal)

```bash
# Instala la librería para comunicación serial
pip install pyserial
```

### 💻 Código (`reader.py`)

```python
import serial
import json
import time

# --- CONFIGURACIÓN ---
PORT_NAME = "COM3"  # <--- REEMPLAZA CON TU PUERTO
BAUD_RATE = 115200
# ---------------------

def check_alerts(data):
    """Aplica las reglas de interpretación y devuelve una lista de alertas."""
    alerts = []
    
    # Cálculo de la aceleración total: √(ax²+ay²+az²)
    accel_mag = (data['ax']**2 + data['ay']**2 + data['az']**2)**0.5
    
    if accel_mag > 1.5:
        alerts.append("🚨 Movimiento brusco")
    if data['tempC'] > 30:
        alerts.append("🌡️ Alta temperatura")
    if data['light'] < 20:
        alerts.append("🌑 Baja luz")
    if data['bat'] < 3.0:
        alerts.append("🔋 Batería baja")
        
    return alerts

print(f"Iniciando lector Python en {PORT_NAME}...")
try:
    # 1. Abrir puerto serial
    ser = serial.Serial(PORT_NAME, BAUD_RATE, timeout=1)
    
    while True:
        # 2. Leer línea
        line = ser.readline().decode('utf-8').strip()
        
        # 3. Manejo de error: línea vacía (puede ocurrir al inicio)
        if not line:
            time.sleep(0.1)
            continue
            
        try:
            # 4. Parsear JSON
            data = json.loads(line)
            
            # 5. Aplicar alertas
            alerts = check_alerts(data)
            
            # 6. Formato de salida
            output = f"[{data['id']}] Temp: {data['tempC']:.1f}°C, Light: {data['light']}, Bat: {data['bat']:.2f}V"
            if alerts:
                output += " | ALERTA: " + ", ".join(alerts)
            
            print(output)

        except json.JSONDecodeError:
            # Manejo de error: línea no es JSON válida
            print(f"Error JSON en línea: {line}")
            
except serial.SerialException as e:
    # Manejo de error: puerto incorrecto/ocupado
    print(f"ERROR: No se pudo abrir el puerto {PORT_NAME}. Asegúrate de que el puerto es correcto y no está en uso. Detalle: {e}")
except KeyboardInterrupt:
    print("\nLector detenido.")
finally:
    if 'ser' in locals() and ser.is_open:
        ser.close()
```

### 🚀 Ejecución (Terminal)

```bash
python reader.py
```

-----

### 🚀 Ejecución (Visual Studio)

Presionar **F5** (Ejecutar) en Visual Studio. El IDE compilará el proyecto, enlazando las librerías a través de vcpkg, y ejecutará el binario.
# Ejecución del Codigo en el editor de Micro:bit de Python
<img width="1916" height="1078" alt="image" src="https://github.com/user-attachments/assets/871b9e67-d488-44e3-bf74-0bc4f48f4045" />

# Visualización de recepción de datos en Visual Code
<img width="1919" height="1079" alt="image" src="https://github.com/user-attachments/assets/e7e924b9-a49a-45ff-b7ff-b146e37ce517" />
