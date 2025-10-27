# DANIEL ROMERO BRAVO

# 🔧 Práctica Integral — Lectura Serial de micro:bit en 6 Lenguajes + MicroPython

**Materia:** Taller de Cultura Digital / IoT
**Docente:** IoTeacher
**Modalidad:** Local (sin nube, sin MQTT)
**Duración:** 2 sesiones (4 h total)

---

## 🎯 Objetivo General

Comprender y mantener una solución IoT ya desplegada que lee datos en tiempo real enviados por un **micro:bit** conectado vía **USB serial**, mediante programas equivalentes en **6 lenguajes distintos**.

---

## 🧠 Contexto

Cada estudiante posee un micro:bit ejecutando un programa que transmite lecturas de sensores (temperatura, acelerómetro, luz y voltaje simulado).
El equipo de mantenimiento (los alumnos) debe **probar, comparar y depurar** lectores desarrollados previamente en:

* 🐍 Python
* 💻 C#
* ⚙️ C++
* 🟢 Node.js
* 🐹 Go
* 🦀 Rust

El objetivo es validar que todos interpreten los mismos datos, detecten alertas y sean mantenibles.

---

# 📊 Análisis Comparativo Técnico Extendido de Lectores Seriales (6 Lenguajes)


| Característica Técnica | 🐍 Python | 🟢 Node.js | 💻 C# | 🐹 Go | 🦀 Rust | ⚙️ C++ |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Biblioteca Serial** | `pyserial` (externa) | `serialport` (externa) | `System.IO.Ports` (NuGet) | `go.bug.st/serial` | `serialport` (Crate) | Win32 API / Boost.Asio |
| **Modelo de Lectura** | Síncrono (`ser.readline()`) | **Asíncrono** (Event Loop) | Síncrono (`ReadLine()`) | Síncrono (`bufio.Scanner`) | Síncrono (`BufReader::read_line`) | Síncrono/Asíncrono (Manual) |
| **Manejo de JSON** | `json` (Nativo) | `JSON` (Nativo) | `System.Text.Json` (Nativo) | `encoding/json` (Nativo) | `serde`/`serde_json` (Crates) | nlohmann/json (Externa) |
| **Rendimiento** | Interpretado (Lento) | JIT (Rápido) | JIT (Rápido) | **Compilado (Muy Rápido)** | **Compilado (Máx. Velocidad)** | Compilado (Máx. Velocidad) |
| **Manejo de Errores** | `try...except` | Callbacks/Excepciones | `try...catch` (Robusto) | Manejo de `error` explícito | **`Result<T, E>` (Seguro)** | Códigos de error Win32/Excepciones |
| **Complejidad de Código** | **Baja** | Baja-Media | Media | Media | Media-Alta | **Muy Alta** |
| **Observaciones Técnicas** | El **más simple**. Abstracción total del SO. Ideal para **prototipado rápido** y scripts de monitoreo. La dependencia es fácil de instalar (`pip`). | Utiliza un **bucle de eventos** (non-blocking I/O), lo que lo hace muy eficiente para tareas concurridas (red y serial a la vez). Requiere un *parser* para manejar el `\n`. | **Ambiente .NET.** La clase `SerialPort` es oficial y estable, ideal para aplicaciones de escritorio o **servicios robustos en Windows**. Requiere definir un *modelo de datos* (Clase) para el JSON. | Famoso por su **concurrencia nativa** (Goroutines). El manejo de errores `if err != nil` es obligatorio, lo que garantiza un **código muy robusto**. Compila muy rápido. | **Máxima seguridad en memoria** (sin GC). El compilador obliga a gestionar todos los posibles fallos seriales y de JSON, resultando en binarios extremadamente rápidos y confiables, perfectos para *gateways*. | Mayor control sobre el *hardware*. Requiere la implementación manual de la comunicación y el buffer (`ReadFile` en Windows). La **mayor curva de aprendizaje** y el código menos portable entre sistemas operativos. |

---

## 🔎 Reflexión Final de la Práctica

### 1. ¿Qué lenguaje resultó más sencillo para manejar el puerto serial?

 Python. Gracias a `pyserial`, la lectura es una sola línea de código síncrono que abstrae completamente las complejidades del sistema operativo. Es la solución más rápida para poner en marcha el lector.

### 2. ¿Qué lenguaje ofrece mejor manejo de errores?

 Rust. Su sistema basado en el tipo `Result<T, E>` fuerza al desarrollador a considerar y manejar explícitamente cada posible escenario de fallo (puerto no encontrado, línea malformada, error de JSON) en tiempo de **compilación**, minimizando la posibilidad de fallos inesperados en producción.

### 3. ¿Cuál sería más adecuado para un sistema IoT en producción local?

 C# o  Go son las mejores opciones balanceadas. C# es excelente para un **servicio robusto** en Windows, aprovechando el ecosistema .NET para estabilidad y fácil depuración. Go es superior si la aplicación necesita **alta velocidad y concurrencia** (manejar muchos *threads* o conexiones) fuera del ecosistema de Microsoft.
