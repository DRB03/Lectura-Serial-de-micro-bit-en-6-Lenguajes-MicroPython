

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
* 🦀 Rust
* 🟢 Node.js
* 🐹 Go

El objetivo es validar que todos interpreten los mismos datos, detecten alertas y sean mantenibles.

---

## Tabla Comparativa de Lectores Seriales (Windows)

| Característica | 🐍 Python | 🟢 Node.js | 💻 C# | ⚙️ C++ |
| :--- | :--- | :--- | :--- | :--- |
| **Biblioteca Serial** | `pyserial` (externa) | `serialport` (externa) | `System.IO.Ports` (NuGet) | **Win32 API** (Nativa de OS) |
| **Instalación** | `pip install pyserial` | `npm install serialport` | `dotnet add package ...` | Ninguna (incluida en Windows SDK) |
| **Biblioteca JSON** | `json` (nativa) | `JSON` (nativo) | `System.Text.Json` (nativa) | **nlohmann/json** (externa) |
| **Manejo de Lectura** | Síncrono (`ser.readline()`) | Asíncrono (eventos) | Síncrono (`_serialPort.ReadLine()`) | Síncrono (`ReadFile()`) + buffer manual |
| **Complejidad de código** | Baja | Baja-Media | Media | **Muy Alta** |
| **Manejo de Errores** | `try...except` | Callbacks/`try...catch` | `try...catch` | Códigos de error de Win32 + `try...catch` |

---

### 1. ¿Qué lenguaje resultó más sencillo para manejar el puerto serial?

Python es mas compatible para esto, ya que con una línea (pip install pyserial), ya tienes todo, con otra línea (ser.readline()), lees un dato, es súper directo y te ahorra todos los pasos de configurar detalles técnicos que C++ te obliga a hacer.

### 2. ¿Qué lenguaje ofrece mejor manejo de errores?

C# ya que es un lenguaje que te obliga a ser ordenado, al ser de "tipado estático" (tienes que declarar si algo es un número o un texto), el programa te dice si rompiste algo antes de que lo ejecutes y su forma de manejar excepciones (try...catch) es robusta.

### 3. ¿Cuál sería más adecuado para un sistema IoT en producción local?

Si el sistema es un servicio que debe funcionar 24/7 en una PC con Windows y no debe fallar nunca: C# es ideal. Es robusto, rápido (por el entorno .NET) y está diseñado para hacer aplicaciones serias y estables.

Si el sistema es un script que solo necesita tomar el dato, procesarlo y enviarlo rápido (sin GUI ni servicio complejo): Python es el campeón. Es rápido de escribir, fácil de mantener (porque es muy legible) y si falla, es más fácil de depurar que C++ o Node.js.
