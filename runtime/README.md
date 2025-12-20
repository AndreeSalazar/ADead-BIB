# 🚀 ADead-BIB Universal Runtime

> **Autor:** Eddi Andreé Salazar Matos | **Hecho con ❤️ en Perú** 🇵🇪

---

## 🎯 Descripción

Runtime universal ultra ligero para ADead-BIB que es:

- **Compatible con todos los lenguajes** - C, C++, Rust, Python, Zig
- **Determinista** - Comportamiento predecible y reproducible
- **Ultra ligero** - < 1 MB (vs 100+ MB de LLVM)
- **Sin dependencias** - Compila en cualquier sistema
- **GPU nativo** - CUDA y Vulkan sin capas intermedias

---

## 📁 Estructura

```
runtime/
├── core/                    # Núcleo del runtime
│   ├── types.h              # Tipos universales
│   ├── memory.h/.c          # Gestor de memoria determinista
│   └── runtime.h/.c         # API principal
│
├── backends/                # Backends de ejecución
│   ├── cpu/                 # Backend CPU (x86-64, ARM)
│   ├── gpu/                 # Backend GPU (CUDA)
│   └── vulkan/              # Backend Vulkan
│       └── shaders/         # Compute shaders GLSL
│
├── ffi/                     # Foreign Function Interfaces
│   ├── cpp/                 # FFI para C++
│   ├── rust/                # FFI para Rust
│   ├── python/              # FFI para Python
│   ├── c/                   # FFI para C puro
│   └── zig/                 # FFI para Zig
│
└── tests/                   # Tests del runtime
```

---

## 🔧 Compilar

### Linux/macOS

```bash
cd runtime
mkdir build && cd build
cmake ..
make -j$(nproc)
```

### Windows

```powershell
cd runtime
mkdir build; cd build
cmake ..
cmake --build . --config Release
```

---

## 📊 Uso

### C

```c
#include "runtime/core/runtime.h"

int main() {
    ADeadRuntime rt;
    adead_init(&rt, ADEAD_BACKEND_AUTO);
    
    // Crear tensores
    ADeadTensor a, b, c;
    u64 shape[] = {1024, 1024};
    adead_tensor_create(&rt, &a, shape, 2, ADEAD_DTYPE_F32);
    adead_tensor_create(&rt, &b, shape, 2, ADEAD_DTYPE_F32);
    adead_tensor_create(&rt, &c, shape, 2, ADEAD_DTYPE_F32);
    
    // MatMul
    adead_matmul(&rt, &a, &b, &c);
    adead_sync(&rt);
    
    // Cleanup
    adead_tensor_destroy(&rt, &a);
    adead_tensor_destroy(&rt, &b);
    adead_tensor_destroy(&rt, &c);
    adead_shutdown(&rt);
    
    return 0;
}
```

### C++

```cpp
#include "runtime/ffi/cpp/adead_cpp.h"

int main() {
    adead::Runtime rt(ADEAD_BACKEND_AUTO);
    
    auto a = rt.tensor({1024, 1024});
    auto b = rt.tensor({1024, 1024});
    auto c = rt.tensor({1024, 1024});
    
    rt.matmul(a, b, c);
    rt.sync();
    
    auto result = c.to_vector<float>();
    return 0;
}
```

### Python

```python
from runtime.ffi.python.adead_py import ADeadRuntime, Backend
import numpy as np

rt = ADeadRuntime(Backend.AUTO)

a = rt.tensor([1024, 1024])
b = rt.tensor([1024, 1024])
c = rt.tensor([1024, 1024])

a.copy_from(np.random.randn(1024, 1024).astype(np.float32))
b.copy_from(np.random.randn(1024, 1024).astype(np.float32))

rt.matmul(a, b, c)
result = c.to_numpy()
```

---

## 🎮 Backends

### CPU (Default)
- Implementación optimizada en C
- Soporte SIMD (AVX2/AVX-512) planificado
- Funciona en cualquier sistema

### CUDA
- Para GPUs NVIDIA
- Kernels optimizados
- Requiere CUDA Toolkit

### Vulkan
- Para cualquier GPU (NVIDIA, AMD, Intel)
- Compute shaders GLSL
- Requiere Vulkan SDK

---

## 📈 Operaciones Soportadas

| Operación | CPU | CUDA | Vulkan |
|-----------|-----|------|--------|
| MatMul | ✅ | 🔄 | 🔄 |
| Add | ✅ | 🔄 | 🔄 |
| ReLU | ✅ | 🔄 | 🔄 |
| Softmax | ✅ | 🔄 | ✅ |
| Attention | ✅ | 🔄 | ✅ |
| LayerNorm | ✅ | 🔄 | 🔄 |

✅ = Implementado | 🔄 = En desarrollo

---

## 📊 Comparación con LLVM

| Característica | LLVM | ADead-BIB Runtime |
|----------------|------|-------------------|
| **Tamaño** | ~100 MB | **< 1 MB** |
| **Dependencias** | Muchas | **Ninguna** |
| **Tiempo compilación** | Minutos | **Milisegundos** |
| **Determinismo** | No garantizado | **100%** |
| **GPU Support** | Limitado | **CUDA + Vulkan** |

---

## 🔗 Integración con Vulkan

Los compute shaders están en `backends/vulkan/shaders/`:

- `matmul.comp` - Multiplicación de matrices tiled
- `softmax.comp` - Softmax con reducción paralela
- `attention.comp` - Scaled dot-product attention

Compilar shaders:
```bash
glslangValidator -V matmul.comp -o matmul.spv
```

---

**Creado por:** Eddi Andreé Salazar Matos  
**Email:** eddi.salazar.dev@gmail.com  
**Hecho con ❤️ en Perú** 🇵🇪
