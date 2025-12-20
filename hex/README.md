# 🔧 HEX - Opcodes GPU para ADead-BIB

> **Autor:** Eddi Andreé Salazar Matos | **Hecho con ❤️ en Perú** 🇵🇪

---

## 📁 Estructura

```
hex/
├── README.md           # Esta documentación
├── gpu_opcodes.py      # Generador de opcodes GPU
├── cuda_kernels.py     # Kernels CUDA pre-compilados
├── ptx_templates/      # Templates PTX para NVIDIA
└── binary_gpu.py       # Generador de binarios GPU
```

## 🎯 Propósito

Esta carpeta contiene los **opcodes hexadecimales** para operaciones GPU que ADead-BIB puede generar directamente, permitiendo:

1. **Binarios híbridos CPU+GPU** - Un solo ejecutable que usa ambos
2. **Llamadas CUDA nativas** - Sin overhead de Python
3. **Kernels pre-compilados** - Máximo rendimiento

## 📊 Opcodes Disponibles

| Opcode | Hex | Descripción |
|--------|-----|-------------|
| GPU_INIT | 0xC0DA0001 | Inicializar contexto CUDA |
| GPU_ALLOC | 0xC0DA0002 | Reservar memoria GPU |
| GPU_COPY_H2D | 0xC0DA0003 | Copiar Host → Device |
| GPU_COPY_D2H | 0xC0DA0004 | Copiar Device → Host |
| GPU_MATMUL | 0xC0DA0010 | Multiplicación de matrices |
| GPU_ADD | 0xC0DA0011 | Suma de tensores |
| GPU_RELU | 0xC0DA0012 | Activación ReLU |
| GPU_SOFTMAX | 0xC0DA0013 | Softmax |
| GPU_ATTENTION | 0xC0DA0020 | Atención multi-head |
| GPU_FFN | 0xC0DA0021 | Feed-forward network |
| GPU_SYNC | 0xC0DA00FF | Sincronizar GPU |
| GPU_FREE | 0xC0DA00FE | Liberar memoria |

## 🚀 Uso

```python
from hex.gpu_opcodes import GPUOpcodes

# Generar binario con soporte GPU
gpu = GPUOpcodes()
binary = gpu.generate_matmul_kernel(1024, 1024)
gpu.execute(binary)
```

## 📈 Rendimiento Esperado

| Operación | CPU | GPU (RTX 3060) | Speedup |
|-----------|-----|----------------|---------|
| MatMul 1024x1024 | 6.5 ms | 0.37 ms | **17.5x** |
| MatMul 4096x4096 | 290 ms | 20 ms | **14.7x** |
| Attention | 221 ms | 5 ms | **44.9x** |

---

**Creado por Eddi Andreé Salazar Matos** | eddi.salazar.dev@gmail.com | 🇵🇪
