# 🚀 Cómo Usar ADead-BIB

> **Autor:** Eddi Andreé Salazar Matos | **Hecho con ❤️ en Perú** 🇵🇪

---

## 📋 Requisitos

### 1. Instalar Rust
```powershell
winget install Rustlang.Rust.MSVC
```

### 2. Instalar Python + NumPy
```powershell
pip install numpy psutil
```

### 3. (Opcional) Instalar Ollama para IA avanzada
```powershell
winget install Ollama.Ollama
ollama pull tinyllama
ollama serve
```

---

## 🔧 Compilar el Proyecto

```powershell
cargo build --release
```

---

## ▶️ Ejecutar Hello World

```powershell
cargo run --release examples/hello_world.adB
.\build\hello_world.exe
```

**Salida esperada:** `Hello, World!`

---

## 🤖 Demos de IA

### IA Básica (0.19 MB RAM)
```powershell
cd python
python ai_complete.py
```

### IA Escalable con BPE (0.82 MB RAM)
```powershell
python ai_scalable.py
```

### Integración con Ollama (Modelo Real)
```powershell
python ollama_integration.py
```

### Demo Completa (Todo junto)
```powershell
python demo_full.py
```

---

## 📊 Resultados Esperados

| Componente | RAM | Velocidad |
|------------|-----|-----------|
| Compilador | ~5 MB | 10-20 ms |
| IA Básica | 0.19 MB | 15 ms/token |
| IA Escalable | 0.82 MB | 29 ms/token |
| Ollama | ~700 MB | 2-3 s/resp |

---

## 📁 Estructura del Proyecto

```
ADead-BIB/
├── src/rust/       # Compilador
├── examples/       # Ejemplos .adB
├── python/         # IA + FFI
├── build/          # Binarios compilados
├── docs/EN/        # Documentación inglés
├── docs/ES/        # Documentación español
└── README.md       # Documentación principal
```

---

**¿Problemas?** Contacta: eddi.salazar.dev@gmail.com