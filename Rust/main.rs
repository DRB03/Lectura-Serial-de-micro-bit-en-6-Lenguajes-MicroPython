use serialport::{SerialPort, self};
use std::{time::Duration, io::{self, BufReader, BufRead}, ffi::OsString};
use serde::Deserialize;
use std::f64; // Para operaciones matemáticas con punto flotante (aceleración)

// --- CONFIGURACIÓN ---
const PORT_NAME: &str = "COM5"; // <--- REEMPLAZA CON TU PUERTO
const BAUD_RATE: u32 = 115200;
// ---------------------

// Estructura para mapear el JSON (requiere #[derive(Deserialize)])
#[derive(Debug, Deserialize)]
struct MicrobitData {
    id: String,
    ts: i64,
    tempC: f64,
    ax: f64,
    ay: f64,
    az: f64,
    light: u8, // light es [0-255]
    bat: f64,
}

/// Aplica las reglas de interpretación y devuelve un vector de alertas.
fn check_alerts(data: &MicrobitData) -> Vec<&'static str> {
    let mut alerts = Vec::new();

    // Cálculo de la aceleración total: √(ax²+ay²+az²)
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
        .timeout(Duration::from_millis(100)) // Tiempo de espera para la lectura
        .open();

    // Manejo de errores de apertura de puerto
    let port: Box<dyn SerialPort> = match port_result {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "ERROR: No se pudo abrir el puerto {}. Asegúrate de que el puerto es correcto y no está en uso.",
                PORT_NAME
            );
            // Mostrar puertos disponibles para depuración
            if let Ok(ports) = serialport::available_ports() {
                 eprintln!("Puertos disponibles:");
                 for p in ports {
                     eprintln!("- {}", p.port_name);
                 }
            } else {
                 eprintln!("No se pudo listar los puertos disponibles.");
            }
            // Devolver un error io::Error para terminar el programa
            return Err(io::Error::new(io::ErrorKind::Other, format!("Error al abrir puerto: {}", e)));
        }
    };
    
    // 2. Usar BufReader para leer línea por línea
    let mut reader = BufReader::new(port);
    let mut line = String::new();

    println!("Conectado. Esperando datos. (Presiona Ctrl+C para salir)");

    loop {
        // 3. Leer línea (Rust requiere que el buffer sea pasado y vaciado)
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => {
                // Si read_line retorna 0, significa que no hay más datos o el puerto está cerrado.
                // Rust es muy explícito en el manejo de EOF (End of File).
                std::thread::sleep(Duration::from_millis(50));
                continue;
            }
            Ok(_) => {
                let trimmed_line = line.trim();

                // Manejo de error: línea vacía
                if trimmed_line.is_empty() {
                    continue;
                }
                
                // 4. Parsear JSON
                match serde_json::from_str::<MicrobitData>(trimmed_line) {
                    Ok(data) => {
                        // 5. Aplicar alertas
                        let alerts = check_alerts(&data);
                        
                        // 6. Formato de salida
                        let mut output = format!(
                            "[{}] Temp: {:.1}°C, Light: {}, Bat: {:.2}V",
                            data.id, data.tempC, data.light, data.bat
                        );

                        if !alerts.is_empty() {
                            let alerts_str = alerts.join(", ");
                            output.push_str(&format!(" | ALERTA: {}", alerts_str));
                        }
                        
                        println!("{}", output);
                    }
                    Err(_) => {
                        // Manejo de error: línea no es JSON válida (mensajes de inicio de MicroPython)
                        eprintln!("Error JSON/Ignorado en línea: {}", trimmed_line);
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                // Ignorar el timeout (esperamos datos)
                continue;
            }
            Err(e) => {
                // Otro error de lectura (ej. puerto desconectado)
                eprintln!("Error de lectura serial: {}", e);
                break;
            }
        }
    }

    Ok(())
}
