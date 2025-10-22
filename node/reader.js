// Archivo: node/reader.js
const { SerialPort } = require('serialport');
const { ReadlineParser } = require('@serialport/parser-readline');

// !!! CAMBIA "COM3" por tu puerto serial !!!
const SERIAL_PORT = 'COM5'; 
const BAUD_RATE = 115200;

function checkAlerts(data) {
    let alerts = [];
    try {
        const mag = Math.sqrt(data.ax**2 + data.ay**2 + data.az**2);
        if (mag > 1.5) alerts.push(`🚨 Movimiento brusco (${mag.toFixed(2)}g)`);
        if (data.tempC > 30) alerts.push(`🌡️ Alta temp (${data.tempC.toFixed(1)}°C)`);
        if (data.light < 20) alerts.push(`🌑 Baja luz (${data.light})`);
        if (data.bat < 3.0) alerts.push(`🔋 Batería baja (${data.bat.toFixed(2)}V)`);
        return alerts.length > 0 ? alerts.join(' | ') : '✅ Todo OK';
    } catch (e) { return `Error: Falta clave en JSON`; }
}

console.log(`Intentando conectar a ${SERIAL_PORT}...`);
const port = new SerialPort({ path: SERIAL_PORT, baudRate: BAUD_RATE });
const parser = port.pipe(new ReadlineParser({ delimiter: '\n' }));

port.on('open', () => console.log('Conectado. Esperando datos... (Presiona Ctrl+C para salir)'));

parser.on('data', (line) => {
    try {
        // --- Corrección de error (Paso 7) ---
        if (!line.trim()) { 
            return; // Ignorar líneas vacías
        }
        // -------------------------------------

        const data = JSON.parse(line);
        const alertsMsg = checkAlerts(data);
        console.log(`[${data.id}] Temp: ${data.tempC}°C | ${alertsMsg}`);
    } catch (e) {
        console.warn(`Error JSON en línea: ${line}`);
    }
});

port.on('error', (err) => {
    console.error(`Error en el puerto ${SERIAL_PORT}: ${err.message}`);
    console.log('Verifica el puerto y que Mu esté cerrado.');
});
