
-----

## 6\. ⚙️ C++ (`cpp/reader.cpp`)

C++ requiere configuraciones complejas con librerías externas (Boost.Asio y nlohmann/json) manejadas por **vcpkg** en **Visual Studio**.

### 🛠️ Configuración (Terminal + Visual Studio)

1.  **Instalar VCPKG** e integrar con Visual Studio.
2.  **Instalar Dependencias** (Desde la carpeta de vcpkg):
    ```bash
    .\vcpkg install boost-asio nlohmann-json
    ```
3.  **Crear Proyecto:** En **Visual Studio (IDE)**, crear una **Aplicación de Consola (C++)**.

### 💻 Código (`reader.cpp`)

```cpp
#include <iostream>
#include <string>
#include <cmath>
#include <vector>
#include <boost/asio.hpp> 
#include <nlohmann/json.hpp> 

using namespace std;
using namespace boost::asio;
using json = nlohmann::json;

// --- CONFIGURACIÓN ---
const string PORT_NAME = "COM3"; // <--- REEMPLAZA CON TU PUERTO
const unsigned int BAUD_RATE = 115200;
// ---------------------

vector<string> check_alerts(const json& data) {
    vector<string> alerts;

    // Se usa .at() para asegurar que la clave existe antes de obtener el valor
    double ax = data.at("ax").get<double>();
    double ay = data.at("ay").get<double>();
    double az = data.at("az").get<double>();
    double tempC = data.at("tempC").get<double>();
    int light = data.at("light").get<int>();
    double bat = data.at("bat").get<double>();
    
    double accel_mag = sqrt(pow(ax, 2) + pow(ay, 2) + pow(az, 2));

    if (accel_mag > 1.5) {
        alerts.push_back("🚨 Movimiento brusco");
    }
    if (tempC > 30.0) {
        alerts.push_back("🌡️ Alta temperatura");
    }
    if (light < 20) {
        alerts.push_back("🌑 Baja luz");
    }
    if (bat < 3.0) {
        alerts.push_back("🔋 Batería baja");
    }

    return alerts;
}

int main() {
    try {
        cout << "Iniciando lector C++ en " << PORT_NAME << "..." << endl;
        
        // 1. Inicializar Boost.Asio y el puerto serial
        io_context io;
        serial_port port(io);

        // 2. Abrir y configurar el puerto
        port.open(PORT_NAME);
        port.set_option(serial_port_base::baud_rate(BAUD_RATE));
        // ... otras configuraciones seriales (8N1)

        cout << "Conectado. Esperando datos..." << endl;

        boost::asio::streambuf buffer;

        while (true) {
            // 3. Leer hasta el carácter de nueva línea '\n'
            boost::asio::read_until(port, buffer, '\n');

            // Convertir el buffer a una string
            istream is(&buffer);
            string line;
            getline(is, line);

            if (line.empty()) continue;

            try {
                // 4. Parsear JSON
                json data = json::parse(line);
                
                // 5. Aplicar alertas
                vector<string> alerts = check_alerts(data);

                // 6. Formato de salida
                string output = "[" + data.at("id").get<string>() + 
                                "] Temp: " + to_string(data.at("tempC").get<double>()) + "°C" +
                                ", Light: " + to_string(data.at("light").get<int>()) +
                                ", Bat: " + to_string(data.at("bat").get<double>()) + "V";
                
                if (!alerts.empty()) {
                    output += " | ALERTA: " + alerts[0];
                    for (size_t i = 1; i < alerts.size(); ++i) {
                        output += ", " + alerts[i];
                    }
                }
                
                cout << output << endl;

            } catch (const json::parse_error& e) {
                // Manejo de error: línea no es JSON válida
                cerr << "Error JSON/Ignorado en línea: " << line << endl;
            }
        }

    } catch (const boost::system::system_error& e) {
        // Manejo de error: puerto incorrecto/ocupado
        cerr << "ERROR: Error del sistema o serial: " << e.what() << endl;
    } catch (const exception& e) {
        cerr << "Error inesperado: " << e.what() << endl;
    }

    return 0;
}
```
-----

### 🚀 Ejecución (Visual Studio)

Presionar **F5** (Ejecutar) en Visual Studio. El IDE compilará el proyecto, enlazando las librerías a través de vcpkg, y ejecutará el binario.
# Ejecución del Codigo en el editor de Micro:bit de Python
<img width="1916" height="1078" alt="image" src="https://github.com/user-attachments/assets/871b9e67-d488-44e3-bf74-0bc4f48f4045" />

# Visualización de recepcion de datos en Visual Studio
<img width="1919" height="1073" alt="image" src="https://github.com/user-attachments/assets/a6fa48ae-ba59-40c5-bc0c-4dd4d137b787" />
