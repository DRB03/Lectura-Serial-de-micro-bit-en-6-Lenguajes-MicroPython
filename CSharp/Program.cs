// Archivo: csharp/Program.cs
using System;
using System.IO.Ports;
using System.Text.Json;
using System.Threading;

// Clase para mapear el JSON (nombres deben coincidir)
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
    // !!! CAMBIA "COM3" por tu puerto serial !!!
    static string SERIAL_PORT = "COM5";
    static int BAUD_RATE = 115200;

    static string CheckAlerts(MicrobitData data)
    {
        var alerts = new System.Collections.Generic.List<string>();
        try
        {
            double mag = Math.Sqrt(Math.Pow(data.ax, 2) + Math.Pow(data.ay, 2) + Math.Pow(data.az, 2));
            if (mag > 1.5) alerts.Add($"🚨 Movimiento brusco ({mag:F2}g)");
            if (data.tempC > 30) alerts.Add($"🌡️ Alta temp ({data.tempC:F1}°C)");
            if (data.light < 20) alerts.Add($"🌑 Baja luz ({data.light})");
            if (data.bat < 3.0) alerts.Add($"🔋 Batería baja ({data.bat:F2}V)");
            return alerts.Count > 0 ? string.Join(" | ", alerts) : "✅ Todo OK";
        }
        catch (Exception) { return "Error en datos"; }
    }

    public static void Main(string[] args)
    {
        Console.WriteLine($"Intentando conectar a {SERIAL_PORT}...");
        using (SerialPort serialPort = new SerialPort(SERIAL_PORT, BAUD_RATE))
        {
            serialPort.ReadTimeout = 1000;

            try
            {
                serialPort.Open();
                Console.WriteLine("Conectado. Esperando datos... (Cierra la ventana para salir)");

                while (true)
                {
                    try
                    {
                        string line = serialPort.ReadLine();

                        // --- Corrección de error (Paso 7) ---
                        if (string.IsNullOrWhiteSpace(line))
                        {
                            continue; // Ignorar líneas vacías
                        }
                        // -------------------------------------

                        MicrobitData data = JsonSerializer.Deserialize<MicrobitData>(line);
                        if (data != null)
                        {
                            string alertsMsg = CheckAlerts(data);
                            Console.WriteLine($"[{data.id}] Temp: {data.tempC}°C | {alertsMsg}");
                        }
                    }
                    catch (TimeoutException) { } // Ignorar timeouts, es normal
                    catch (JsonException) { Console.WriteLine($"Error JSON en línea"); }
                }
            }
            catch (Exception e)
            {
                Console.WriteLine($"Error al abrir el puerto {SERIAL_PORT}: {e.Message}");
                Console.WriteLine("Verifica el puerto y que Mu esté cerrado.");
                Console.WriteLine("Presiona Enter para salir.");
                Console.ReadLine();
            }
        }
    }
}
