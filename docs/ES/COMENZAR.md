# 🚀 Comenzar con ADead-BIB

> **Autor:** Eddi Andreé Salazar Matos | **Hecho con ❤️ en Perú** 🇵🇪

---

## Requisitos Previos

### 1. Instalar Rust

```bash
# Windows (PowerShell)
winget install Rustlang.Rust.MSVC

# O descargar desde https://rustup.rs
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Instalar Python 3.8+

```bash
# Windows
winget install Python.Python.3.11

# O descargar desde https://python.org
```

### 3. Instalar Dependencias de Python

```bash
pip install numpy
```

### 4. (Opcional) Instalar Ollama para Demos de IA

```bash
# Instalar Ollama
winget install Ollama.Ollama

# Descargar un modelo pequeño
ollama pull tinyllama

# Iniciar servidor Ollama (en otra terminal)
ollama serve
```

---

## Inicio Rápido

### Paso 1: Compilar el Compilador

```powershell
cd ADead-BIB
cargo build --release
```

### Paso 2: Escribir tu Primer Programa

Crea un archivo `examples/mi_primero.adB`:

```python
def main():
    print("Hola desde ADead-BIB!")
    x = 10
    y = 20
    print(x + y)
```

### Paso 3: Compilar y Ejecutar

```powershell
cargo run --release examples/mi_primero.adB
.\mi_primero.exe
```

**Salida:**
```
Hola desde ADead-BIB!
30
```

---

## Integración con IA

### Demo de IA Básica (0.19 MB RAM)

```powershell
cd python
python ai_complete.py
```

### IA Escalable con BPE (0.82 MB RAM)

```powershell
python ai_scalable.py
```

### Con Ollama (Requiere Ollama Ejecutándose)

```powershell
# En terminal 1: Iniciar Ollama
ollama serve

# En terminal 2: Ejecutar demo
python ollama_integration.py
```

---

## Estructura del Proyecto

```
ADead-BIB/
├── src/rust/           # Código fuente del compilador
├── examples/           # Archivos de ejemplo .adB
├── python/             # Integración Python + IA
├── build/              # Ejecutables compilados
└── docs/               # Documentación
    ├── EN/             # Documentación en inglés
    └── ES/             # Documentación en español
```

---

## Próximos Pasos

1. **Aprende la sintaxis:** Ver `SINTAXIS.md`
2. **Explora ejemplos:** Revisa la carpeta `examples/`
3. **Prueba las funciones de IA:** Ejecuta los demos de Python
4. **Lee el roadmap:** Ver `docs/IDEAS/`

---

**Creado por Eddi Andreé Salazar Matos** | eddi.salazar.dev@gmail.com | 🇵🇪
