# 🎮 TEST-G: Tests de GPU y Vulkan para ADead-BIB

> **Objetivo:** Testear y validar la integración GPU/Vulkan para el runtime determinista.

---

## 📁 Estructura

```
TEST-G/
├── vulkan_detect/       # ✅ Detección de Vulkan disponible
├── cpu_gpu_dispatch/    # ✅ Auto-dispatch CPU vs GPU
├── benchmark/           # ✅ Benchmarks CPU vs GPU
└── README.md
```

## 📊 Resultados de Tests

| Test | Descripción | Tests | Estado |
|------|-------------|-------|--------|
| vulkan_detect | Detectar Vulkan/CUDA | 6/6 | ✅ PASSED |
| cpu_gpu_dispatch | Auto-dispatch | 7/7 | ✅ PASSED |
| benchmark | CPU vs GPU | 5/5 | ✅ PASSED |

**Total: 18/18 tests - 100% DETERMINISTA**

---

## 🖥️ Tu Sistema Detectado

| Componente | Valor |
|------------|-------|
| **CPU** | AMD Ryzen 5 5600X |
| **SIMD** | AVX2 (256-bit) + FMA |
| **GPU Vendor** | NVIDIA |
| **Vulkan** | ✓ Disponible |
| **CUDA** | ✓ Disponible |

---

## 📈 Métricas de Rendimiento

| Métrica | Valor |
|---------|-------|
| Dispatch overhead | **9.48 ns** |
| Dispatches/segundo | **106 M** |
| AVX2 speedup | **1.9x** vs scalar |
| Determinismo | **100%** |

---

## 🎯 Auto-Dispatch en Acción

```
100 elementos     → CpuAvx2
500K elementos    → CpuAvx2
2M elementos      → GpuCuda
100M elementos    → GpuCuda

MatMul 32x32      → CpuAvx2
MatMul 256x256    → GpuCuda
MatMul 1024x1024  → GpuCuda
```

---

## 🚀 Ejecutar Tests

```powershell
# Vulkan Detection
cd vulkan_detect
rustc test_vulkan.rs -o test.exe && .\test.exe

# CPU+GPU Dispatch
cd cpu_gpu_dispatch
rustc test_dispatch.rs -o test.exe && .\test.exe

# Benchmark
cd benchmark
rustc test_benchmark.rs -o test.exe -C target-feature=+avx2,+fma && .\test.exe
```

---

## 🔧 Requisitos

- Windows 10/11 o Linux
- GPU compatible con Vulkan (NVIDIA, AMD, Intel)
- Driver actualizado

---

**Autor:** Eddi Andreé Salazar Matos  
**Estado:** ✅ COMPLETADO - Runtime GPU DETERMINISTA
