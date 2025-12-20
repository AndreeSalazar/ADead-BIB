# Guía Completa: ADead-BIB + GPU + Ollama

> **Autor:** Eddi Andreé Salazar Matos | **Hecho con ❤️ en Perú** 🇵🇪

---

## Requisitos

### 1. Instalar Rust
```powershell
winget install Rustlang.Rust.MSVC
```

### 2. Instalar Python + Dependencias
```powershell
pip install numpy psutil GPUtil
```

### 3. Instalar PyTorch con CUDA (GPU)
```powershell
pip install torch --index-url https://download.pytorch.org/whl/cu121
```

### 4. Instalar Ollama para IA
```powershell
winget install Ollama.Ollama
ollama pull tinyllama
ollama serve
```

---

## Compilar el Proyecto

```powershell
cargo build --release
```

---

## Ejecutar Hello World

```powershell
cargo run --release examples/hello_world.adB
.\build\hello_world.exe
```

**Salida esperada:** `Hello, World!`

---

## Demos GPU

### Benchmark GPU vs CPU
```powershell
cd python
python benchmark_gpu.py
```

### Comparación CPU vs GPU vs Híbrido
```powershell
python demo_gpu_comparison.py
```

### Kernels CUDA
```powershell
cd ../hex
python cuda_kernels.py
```

---

## Demos de IA

### IA Básica (0.19 MB RAM)
```powershell
cd python
python ai_complete.py
```

### IA Escalable con BPE (0.82 MB RAM)
```powershell
python ai_scalable.py
```

### Integración con Ollama
```powershell
python ollama_integration.py
```

### Demo Completa
```powershell
python demo_full.py
```

### Benchmark Ollama: CPU vs GPU
```powershell
python ollama_gpu_benchmark.py
```

---

## Comparación de Modos con Ollama

| Modo | CPU | GPU | RAM | Velocidad | Uso |
|------|-----|-----|-----|-----------|-----|
| **CPU Solo** | 100% | 0% | ~2 GB | 5-10 s/resp | Sin GPU |
| **GPU Solo** | 10% | 90% | ~4 GB VRAM | 1-2 s/resp | Máximo rendimiento |
| **CPU + GPU** | 50% | 50% | ~2 GB + 2 GB | 2-3 s/resp | Balanceado |
| **Híbrido Óptimo** | 10% | 90% | ~1 GB + 3 GB | 1.5 s/resp | Recomendado |

### Detalle de Modos

#### CPU Solo (100% CPU, 0% GPU)
- **Cuándo usar:** Sin GPU o GPU ocupada
- **Ventajas:** Compatible con todo
- **Desventajas:** Lento para modelos grandes

#### GPU Solo (10% CPU, 90% GPU)
- **Cuándo usar:** Máximo rendimiento
- **Ventajas:** 5-10x más rápido
- **Desventajas:** Requiere VRAM suficiente

#### CPU + GPU (50% / 50%)
- **Cuándo usar:** VRAM limitada
- **Ventajas:** Balanceado
- **Desventajas:** Overhead de transferencia

#### Híbrido Óptimo (10% CPU, 90% GPU)
- **Cuándo usar:** Producción
- **Ventajas:** Mejor relación velocidad/recursos
- **Desventajas:** Requiere configuración

---

## Resultados Reales (RTX 3060 12GB)

### Multiplicación de Matrices

| Tamaño | CPU | GPU | Speedup |
|--------|-----|-----|---------|
| 512x512 | 1.04 ms | 0.10 ms | **10.1x** |
| 1024x1024 | 5.75 ms | 0.36 ms | **15.9x** |
| 2048x2048 | 38.22 ms | 2.38 ms | **16.1x** |
| 4096x4096 | 317 ms | 19 ms | **16.7x** |

### Atención Transformer

| Config | CPU | GPU | Speedup |
|--------|-----|-----|---------|
| seq=512, dim=128 | 92 ms | 1.3 ms | **73.6x** |
| seq=1024, dim=256 | 488 ms | 5.7 ms | **86.1x** |

### Ollama con GPU

| Modo | Tiempo/Respuesta | Tokens/s |
|------|------------------|----------|
| CPU Solo | 5-10 s | 10-20 |
| GPU Solo | 1-2 s | 50-100 |
| Híbrido | 1.5-3 s | 40-80 |

---

## Estructura del Proyecto

```
ADead-BIB/
├── src/rust/       # Compilador
├── examples/       # Ejemplos .adB
├── python/         # IA + FFI + Demos
│   ├── ai_complete.py
│   ├── ai_scalable.py
│   ├── benchmark_gpu.py
│   ├── demo_gpu_comparison.py
│   ├── ollama_integration.py
│   └── ollama_gpu_benchmark.py
├── hex/            # Opcodes GPU
│   ├── gpu_opcodes.py
│   ├── cuda_kernels.py
│   └── binary_gpu.py
├── build/          # Binarios compilados
├── docs/EN/        # Documentación inglés
├── docs/ES/        # Documentación español
└── README.md       # Documentación principal
```

---

## Recomendaciones

| Tarea | Modo Recomendado |
|-------|------------------|
| Tokenización | CPU |
| Embeddings | GPU |
| Atención | GPU (86x más rápido) |
| FFN | GPU |
| Generación Ollama | GPU (5x más rápido) |
| I/O, Preprocesamiento | CPU |

---

**¿Problemas?** Contacta: eddi.salazar.dev@gmail.com