package main

import (
    "bufio"
    "fmt"
    "log"
    "math"
    "encoding/json"
    "time"

    "go.bug.st/serial" // Librería serial
)

// Estructura para mapear el JSON
type MicrobitData struct {
    ID    string  `json:"id"`
    TS    int64   `json:"ts"`
    TempC float64 `json:"tempC"`
    Ax    float64 `json:"ax"`
    Ay    float64 `json:"ay"`
    Az    float64 `json:"az"`
    Light int     `json:"light"`
    Bat   float64 `json:"bat"`
}

// --- CONFIGURACIÓN ---
const PortName = "COM5" // <--- REEMPLAZA CON TU PUERTO
const BaudRate = 115200
// ---------------------

func checkAlerts(data MicrobitData) []string {
    var alerts []string
    
    // Cálculo de la aceleración total: √(ax²+ay²+az²)
    accelMag := math.Sqrt(data.Ax*data.Ax + data.Ay*data.Ay + data.Az*data.Az)
    
    if accelMag > 1.5 {
        alerts = append(alerts, "🚨 Movimiento brusco")
    }
    if data.TempC > 30.0 {
        alerts = append(alerts, "🌡️ Alta temperatura")
    }
    if data.Light < 20 {
        alerts = append(alerts, "🌑 Baja luz")
    }
    if data.Bat < 3.0 {
        alerts = append(alerts, "🔋 Batería baja")
    }
    
    return alerts
}

func main() {
    fmt.Printf("Iniciando lector Go en %s...\n", PortName)

    // 1. Configurar el puerto
    mode := &serial.Mode{
        BaudRate: BaudRate,
        DataBits: 8,
        Parity: serial.NoParity,
        StopBits: serial.OneStopBit,
    }

    // 2. Abrir puerto serial
    port, err := serial.Open(PortName, mode)
    
    // Manejo de error: puerto incorrecto/ocupado (Típico manejo de errores en Go)
    if err != nil {
        log.Fatalf("ERROR: No se pudo abrir el puerto %s. Asegúrate de que el puerto es correcto y no está en uso. Detalle: %v", PortName, err)
    }
    defer port.Close() // Asegura que el puerto se cierre al salir

    // 3. Usar un Scanner para leer línea por línea
    reader := bufio.NewScanner(port)
    reader.Split(bufio.ScanLines)

    fmt.Println("Conectado. Esperando datos...")

    for reader.Scan() {
        line := reader.Text()
        line = time.Now().Format("[15:04:05] ") + line // Agrega un timestamp local de la PC

        // Manejo de error: línea vacía (puede ocurrir)
        if len(line) == 0 {
            continue
        }

        var data MicrobitData
        // 4. Parsear JSON. Go requiere una referencia (&data)
        err := json.Unmarshal([]byte(line), &data)
        
        if err != nil {
            // Manejo de error: línea no es JSON válida (ej: mensajes de inicio de MicroPython)
            fmt.Printf("Error JSON/Ignorado: %s\n", line)
            continue
        }

        // 5. Aplicar alertas
        alerts := checkAlerts(data)
        
        // 6. Formato de salida
        output := fmt.Sprintf("[%s] Temp: %.1f°C, Light: %d, Bat: %.2fV", 
                              data.ID, data.TempC, data.Light, data.Bat)

        if len(alerts) > 0 {
            output += " | ALERTA: "
            for i, alert := range alerts {
                if i > 0 {
                    output += ", "
                }
                output += alert
            }
        }
        fmt.Println(output)
    }

    // Verifica si el bucle terminó por un error de lectura
    if err := reader.Err(); err != nil {
        log.Println("Error de lectura serial:", err)
    }
}
