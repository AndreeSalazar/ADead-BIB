# 🧪 Examples-New: Tests de Mejoras ADead-BIB

> **Objetivo:** Testear paso a paso las mejoras del compilador para asegurar **determinismo** y **estabilidad**.

---

## 📁 Estructura de Tests

```
examples-new/
├── fase1_syscalls/      # ✅ Syscalls directos
├── fase2_stack/         # ✅ Stack dinámico
├── fase3_functions/     # ✅ Múltiples funciones
├── fase4_targets/       # ✅ Multi-target (PE, ELF, Raw)
└── fase5_optimize/      # 🔄 Optimizaciones (pendiente)
```

## 🎯 Criterios de Éxito

Cada test debe cumplir:

1. **Determinismo**: Mismo input → Mismo output (bytes idénticos) ✅
2. **Compilación**: Sin errores ni warnings ✅
3. **Ejecución**: Output correcto ✅
4. **Tamaño**: Binario mínimo posible ✅

## 📊 Registro de Tests - RESULTADOS FINALES

| Fase | Test | Estado | Tests | Determinista |
|------|------|--------|-------|--------------|
| 1 | syscalls | ✅ PASSED | 4/4 | 100% |
| 2 | stack | ✅ PASSED | 5/5 | 100% |
| 3 | functions | ✅ PASSED | 5/5 | 100% |
| 4 | targets | ✅ PASSED | 5/5 | 100% |

**Total: 19/19 tests pasados - 100% DETERMINISTA**

---

## 📈 Métricas Clave

### Fase 1: Syscalls
- `sys_write`: 33 bytes, hash consistente
- `sys_exit`: 12 bytes, hash consistente
- Hello World Linux: 45 bytes de código

### Fase 2: Stack Dinámico
- 10 vars: 88 bytes stack
- 50 vars: 408 bytes stack (supera límite anterior de 256)
- 100 vars: 808 bytes stack
- 500 vars: 4008 bytes stack ✅
- Alineación 16 bytes: 100%

### Fase 3: Múltiples Funciones
- 2 funciones: 32 bytes
- Llamadas resueltas: ✅
- Cadena A→B→C: ✅
- Recursión (factorial): 46 bytes
- 50 funciones: 800 bytes

### Fase 4: Multi-Target
- ELF Hello World: 179 bytes
- PE mínimo: 1536 bytes
- Raw binary: 7 bytes (overhead ELF: 120 bytes)
- Código preservado en ambos formatos: ✅

### Fase 5: Auto-Detección CPU
- Detección básica: ✅
- Determinismo: ✅
- SSE2 disponible: ✅
- SIMD avanzado: ✅
- Backend selection: ✅

### Tu Hardware Detectado:
- **CPU:** AMD Ryzen 5 5600X 6-Core Processor
- **Cores:** 12 threads
- **Best SIMD:** AVX2 (256-bit, 8 floats/vector)
- **FMA:** ✓ (Fused Multiply-Add)

---

## 🚀 Ejecutar Tests

```powershell
# Fase 1
cd fase1_syscalls
rustc test_syscalls.rs -o test.exe && .\test.exe

# Fase 2
cd fase2_stack
rustc test_stack.rs -o test.exe && .\test.exe

# Fase 3
cd fase3_functions
rustc test_functions.rs -o test.exe && .\test.exe

# Fase 4
cd fase4_targets
rustc test_targets.rs -o test.exe && .\test.exe

# Fase 5
cd fase5_detect
rustc test_detect.rs -o test.exe && .\test.exe
```

---

**Autor:** Eddi Andreé Salazar Matos  
**Fecha:** 2024-12-20  
**Estado:** ✅ BASE DETERMINISTA + AUTO-DETECCIÓN CPU COMPLETADA
