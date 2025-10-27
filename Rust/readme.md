## 5\. 🦀 Rust (`rust/src/main.rs`)

Rust ofrece la máxima velocidad y seguridad de memoria, gestionado por Cargo.

### 🛠️ Configuración (Terminal)

```bash
# Crea la estructura de proyecto
cargo new rust --bin 
cd rust 
```

Abre `rust/Cargo.toml` y añade las dependencias:

```toml
[dependencies]
serialport = "4.2" 
serde = { version = "1.0", features = ["derive"] } 
serde_json = "1.0" 
```

### 💻 Código (`src/main.rs`)

```rust
use serialport::{SerialPort, self};
use std::{time::Duration, io::{self, BufReader, BufRead}};
use serde::Deserialize;
use std::f64; 

// --- CONFIGURACIÓN ---
const PORT_NAME: &str = "COM3"; // <--- REEMPLAZA CON TU PUERTO
const BAUD_RATE: u32 = 115200;
// ---------------------

// Estructura para mapear el JSON
#[derive(Debug, Deserialize)]
struct MicrobitData {
    id: String,
    ts: i64,
    tempC: f64,
    ax: f64,
    ay: f64,
    az: f64,
    light: u8, 
    bat: f64,
}

fn check_alerts(data: &MicrobitData) -> Vec<&'static str> {
    let mut alerts = Vec::new();

    let accel_mag = f64::sqrt(data.ax.powi(2) + data.ay.powi(2) + data.az.powi(2));

    if accel_mag > 1.5 {
        alerts.push("🚨 Movimiento brusco");
    }
    if data.tempC > 30.0 {
        alerts.push("🌡️ Alta temperatura");
    }
    if data.light < 20 {
        alerts.push("🌑 Baja luz");
    }
    if data.bat < 3.0 {
        alerts.push("🔋 Batería baja");
    }
    alerts
}

fn main() -> io::Result<()> {
    println!("Iniciando lector Rust en {}...", PORT_NAME);

    // 1. Abrir puerto serial
    let port_result = serialport::new(PORT_NAME, BAUD_RATE)
        .timeout(Duration::from_millis(100))
        .open();

    let port: Box<dyn SerialPort> = match port_result {
        Ok(p) => p,
        Err(e) => {
            eprintln!("ERROR: No se pudo abrir el puerto {}. Detalle: {}", PORT_NAME, e);
            return Err(io::Error::new(io::ErrorKind::Other, "Error al abrir puerto."));
        }
    };
    
    // 2. Usar BufReader para leer línea por línea
    let mut reader = BufReader::new(port);
    let mut line = String::new();

    println!("Conectado. Esperando datos. (Presiona Ctrl+C para salir)");

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(_) => {
                let trimmed_line = line.trim();

                if trimmed_line.is_empty() {
                    continue;
                }
                
                // 3. Parsear JSON
                match serde_json::from_str::<MicrobitData>(trimmed_line) {
                    Ok(data) => {
                        // 4. Aplicar alertas
                        let alerts = check_alerts(&data);
                        
                        // 5. Formato de salida
                        let mut output = format!(
                            "[{}] Temp: {:.1}°C, Light: {}, Bat: {:.2}V",
                            data.id, data.tempC, data.light, data.bat
                        );

                        if !alerts.is_empty() {
                            output.push_str(&format!(" | ALERTA: {}", alerts.join(", ")));
                        }
                        
                        println!("{}", output);
                    }
                    Err(_) => {
                        // Manejo de error: línea no es JSON válida
                        eprintln!("Error JSON/Ignorado en línea: {}", trimmed_line);
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                continue;
            }
            Err(e) => {
                eprintln!("Error de lectura serial: {}", e);
                break;
            }
        }
    }

    Ok(())
}
```

### 🚀 Ejecución (Terminal)

```bash
# Asegúrate de estar en la carpeta 'rust'
cargo run --release
```

-----


### 🚀 Ejecución (Visual Studio)

# Ejecución del Codigo en el editor de Micro:bit de Python
<img width="1916" height="1078" alt="image" src="https://github.com/user-attachments/assets/871b9e67-d488-44e3-bf74-0bc4f48f4045" />
