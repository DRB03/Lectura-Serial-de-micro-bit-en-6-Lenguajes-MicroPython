## 2\. 🟢 Node.js (`node/reader.js`)

Node.js es común en aplicaciones web/IoT por su naturaleza asíncrona.

### 🛠️ Configuración (Terminal)

```bash
# Crea un proyecto Node.js y navega a la carpeta
mkdir node && cd node
npm init -y

# Instala la librería para comunicación serial
npm install serialport @serialport/parser-readline
```

### 💻 Código (`reader.js`)

```javascript
const { SerialPort } = require('serialport')
const { ReadlineParser } = require('@serialport/parser-readline')

// --- CONFIGURACIÓN ---
const PORT_NAME = "COM3"; // <--- REEMPLAZA CON TU PUERTO
const BAUD_RATE = 115200;
// ---------------------

/**
 * Aplica las reglas de interpretación y devuelve una lista de alertas.
 * @param {object} data - Objeto JSON con los datos del micro:bit.
 */
function checkAlerts(data) {
    const alerts = [];
    
    // Cálculo de la aceleración total: √(ax²+ay²+az²)
    const accelMag = Math.sqrt(data.ax**2 + data.ay**2 + data.az**2);
    
    if (accelMag > 1.5) {
        alerts.push("🚨 Movimiento brusco");
    }
    if (data.tempC > 30) {
        alerts.push("🌡️ Alta temperatura");
    }
    if (data.light < 20) {
        alerts.push("🌑 Baja luz");
    }
    if (data.bat < 3.0) {
        alerts.push("🔋 Batería baja");
    }
    
    return alerts;
}

console.log(`Iniciando lector Node.js en ${PORT_NAME}...`);
try {
    // 1. Abrir puerto serial
    const port = new SerialPort({ path: PORT_NAME, baudRate: BAUD_RATE });
    
    // 2. Usar un parser para leer líneas
    const parser = port.pipe(new ReadlineParser({ delimiter: '\n' }));

    // Manejo de error al abrir el puerto
    port.on('error', function(err) {
        console.error(`ERROR: No se pudo abrir el puerto ${PORT_NAME}. Asegúrate de que el puerto es correcto y no está en uso.`);
        console.error(`Detalle: ${err.message}`);
        process.exit(1);
    });

    // 3. Leer datos del parser
    parser.on('data', (line) => {
        const trimmedLine = line.trim();
        
        if (trimmedLine.length === 0) return;

        try {
            // 4. Parsear JSON
            const data = JSON.parse(trimmedLine);
            
            // 5. Aplicar alertas
            const alerts = checkAlerts(data);
            
            // 6. Formato de salida
            let output = `[${data.id}] Temp: ${data.tempC.toFixed(1)}°C, Light: ${data.light}, Bat: ${data.bat.toFixed(2)}V`;
            if (alerts.length > 0) {
                output += " | ALERTA: " + alerts.join(", ");
            }
            
            console.log(output);

        } catch (e) {
            // Manejo de error: línea no es JSON válida
            console.log(`Error JSON en línea: ${trimmedLine}`);
        }
    });

} catch (e) {
    // Captura errores de instanciación
    console.error("Error general al iniciar SerialPort:", e.message);
    process.exit(1);
}

// Detener con Ctrl+C
process.on('SIGINT', () => {
    console.log('\nLector detenido.');
    process.exit();
});
```

### 🚀 Ejecución (Terminal)

```bash
node reader.js
```


-----

# Ejecución del Codigo en el editor de Micro:bit de Python
<img width="1916" height="1078" alt="image" src="https://github.com/user-attachments/assets/871b9e67-d488-44e3-bf74-0bc4f48f4045" />


# Evidencia de Recepción de datos en Visual Code.
<img width="1919" height="1079" alt="image" src="https://github.com/user-attachments/assets/3ea7a280-7c1d-4c94-bc6e-ba825e8eb0a0" />
