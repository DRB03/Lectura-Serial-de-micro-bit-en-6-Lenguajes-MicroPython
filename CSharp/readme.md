## 3\. 💻 C\# (`csharp/Program.cs`)

C\# es el estándar para aplicaciones Windows, utilizando la librería `System.IO.Ports` del .NET SDK.

### 🛠️ Configuración (Terminal)

```bash
# Crea un proyecto de consola .NET y navega a la carpeta
mkdir csharp && cd csharp
dotnet new console

# No es necesaria librería JSON externa para .NET 6+
```

### 💻 Código (`Program.cs`)

```csharp
using System;
using System.IO.Ports;
using System.Text.Json;
using System.Threading.Tasks;
using System.Collections.Generic;
using System.Linq;

namespace MicrobitReader
{
    // Modelo de datos para deserialización
    public class MicrobitData
    {
        public string id { get; set; }
        public long ts { get; set; }
        public double tempC { get; set; }
        public double ax { get; set; }
        public double ay { get; set; }
        public double az { get; set; }
        public int light { get; set; }
        public double bat { get; set; }
    }

    class Program
    {
        // --- CONFIGURACIÓN ---
        private const string PortName = "COM3"; // <--- REEMPLAZA CON TU PUERTO
        private const int BaudRate = 115200;
        // ---------------------

        static async Task Main(string[] args)
        {
            Console.WriteLine($"Iniciando lector C# en {PortName}...");

            using (var serialPort = new SerialPort(PortName, BaudRate))
            {
                try
                {
                    // 1. Abrir puerto serial
                    serialPort.Open();
                    serialPort.ReadTimeout = 1000; // 1 segundo
                    
                    while (true)
                    {
                        // 2. Leer línea
                        string line = serialPort.ReadLine().Trim();

                        if (string.IsNullOrEmpty(line)) continue;

                        try
                        {
                            // 3. Parsear JSON
                            var data = JsonSerializer.Deserialize<MicrobitData>(line);
                            
                            if (data != null)
                            {
                                // 4. Aplicar alertas
                                var alerts = CheckAlerts(data);

                                // 5. Formato de salida
                                string output = $"[{data.id}] Temp: {data.tempC:F1}°C, Light: {data.light}, Bat: {data.bat:F2}V";
                                if (alerts.Count > 0)
                                {
                                    output += " | ALERTA: " + string.Join(", ", alerts);
                                }

                                Console.WriteLine(output);
                            }
                        }
                        catch (JsonException)
                        {
                            // Manejo de error: línea no es JSON válida
                            Console.WriteLine($"Error JSON en línea: {line}");
                        }
                        catch (TimeoutException)
                        {
                            // Ignorar si solo es un timeout de lectura ocasional
                        }
                        await Task.Delay(1); 
                    }
                }
                catch (System.IO.IOException e)
                {
                    // Manejo de error: Puerto incorrecto/desconectado (el error más común al iniciar)
                    Console.WriteLine($"ERROR: No se pudo encontrar o abrir el puerto {PortName}. Detalle: {e.Message}");
                }
                catch (Exception e)
                {
                    Console.WriteLine($"Ocurrió un error inesperado: {e.Message}");
                }
                finally
                {
                    if (serialPort.IsOpen)
                        serialPort.Close();
                }
            }
        }

        private static List<string> CheckAlerts(MicrobitData data)
        {
            var alerts = new List<string>();

            // Cálculo de la aceleración total: √(ax²+ay²+az²)
            double accelMag = Math.Sqrt(Math.Pow(data.ax, 2) + Math.Pow(data.ay, 2) + Math.Pow(data.az, 2));

            if (accelMag > 1.5)
            {
                alerts.Add("🚨 Movimiento brusco");
            }
            if (data.tempC > 30)
            {
                alerts.Add("🌡️ Alta temperatura");
            }
            if (data.light < 20)
            {
                alerts.Add("🌑 Baja luz");
            }
            if (data.bat < 3.0)
            {
                alerts.Add("🔋 Batería baja");
            }

            return alerts;
        }
    }
}
```

### 🚀 Ejecución (Terminal)

```bash
dotnet run
```

-----

### 🚀 Ejecución (Visual Studio)

Presionar **F5** (Ejecutar) en Visual Studio. El IDE compilará el proyecto, enlazando las librerías a través de vcpkg, y ejecutará el binario.
# Ejecución del Codigo en el editor de Micro:bit de Python
<img width="1916" height="1078" alt="image" src="https://github.com/user-attachments/assets/871b9e67-d488-44e3-bf74-0bc4f48f4045" />

# Visualización de recepcion de datos en Visual Studio
<img width="1919" height="1079" alt="image" src="https://github.com/user-attachments/assets/64d9e65e-dc52-445e-9490-2bf0ee6816fc" />
