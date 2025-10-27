## 4\. 🐹 Go (`go/main.go`)

Go destaca por su compilación rápida a binario nativo y su excelente manejo de errores explícito.

### 🛠️ Configuración (Terminal)

```bash
# Navega a la carpeta 'go'
cd go

# Crea el módulo Go
go mod init microbit-reader-go

# Descarga e instala la librería serial
go get go.bug.st/serial
```

### 💻 Código (`main.go`)

```go
package main

import (
    "bufio"
    "fmt"
    "log"
    "math"
    "encoding/json"

    "go.bug.st/serial" 
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
const PortName = "COM3" // <--- REEMPLAZA CON TU PUERTO
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

    // 1. Configurar y abrir puerto serial
    mode := &serial.Mode{BaudRate: BaudRate}
    port, err := serial.Open(PortName, mode)
    
    if err != nil {
        log.Fatalf("ERROR: No se pudo abrir el puerto %s. Detalle: %v", PortName, err)
    }
    defer port.Close()

    // 2. Usar un Scanner para leer línea por línea
    reader := bufio.NewScanner(port)
    reader.Split(bufio.ScanLines)

    fmt.Println("Conectado. Esperando datos...")

    for reader.Scan() {
        line := reader.Text()
        if len(line) == 0 {
            continue
        }

        var data MicrobitData
        
        // 3. Parsear JSON
        err := json.Unmarshal([]byte(line), &data)
        
        if err != nil {
            // Manejo de error: línea no es JSON válida (mensajes de inicio)
            fmt.Printf("Error JSON/Ignorado: %s\n", line)
            continue
        }

        // 4. Aplicar alertas
        alerts := checkAlerts(data)
        
        // 5. Formato de salida
        output := fmt.Sprintf("[%s] Temp: %.1f°C, Light: %d, Bat: %.2fV", 
                              data.ID, data.TempC, data.Light, data.Bat)

        if len(alerts) > 0 {
            output += " | ALERTA: " + alerts[0]
            if len(alerts) > 1 {
                for _, alert := range alerts[1:] {
                    output += ", " + alert
                }
            }
        }
        fmt.Println(output)
    }

    if err := reader.Err(); err != nil {
        log.Println("Error de lectura serial:", err)
    }
}
```

### 🚀 Ejecución (Terminal)

```bash
# Asegúrate de estar en la carpeta 'go'
go run main.go
```

-----

### 🚀 Ejecución (Visual Studio)

Presionar **F5** (Ejecutar) en Visual Studio. El IDE compilará el proyecto, enlazando las librerías a través de vcpkg, y ejecutará el binario.
# Ejecución del Codigo en el editor de Micro:bit de Python
<img width="1916" height="1078" alt="image" src="https://github.com/user-attachments/assets/871b9e67-d488-44e3-bf74-0bc4f48f4045" />

# Evidencia de recepción de datos en Visual Code
<img width="1919" height="1079" alt="image" src="https://github.com/user-attachments/assets/40dd453f-0146-4967-95de-5b0a00f5a35e" />
