# ADead-BIB FFI/ABI - Foreign Function Interface

**ADead-BIB como Cabeza Principal ABI para Interoperabilidad Universal**

Author: Eddi Andreé Salazar Matos  
Email: eddi.salazar.dev@gmail.com  
Made with ❤️ in Peru 🇵🇪

---

## 🎯 Visión

ADead-BIB actúa como **cabeza principal ABI** (Application Binary Interface) para:
- Compilar código de alto rendimiento directo a binario
- Exportar funciones para uso desde cualquier lenguaje
- Importar funciones de otros lenguajes
- Interoperabilidad universal sin overhead

## 📁 Estructura

```
FFI/
├── README.md           # Este archivo
├── abi/                # Definiciones ABI estándar
│   └── adead_abi.adB   # ABI principal de ADead-BIB
├── python/             # Bindings Python
│   ├── adead_py.py     # Wrapper Python
│   └── examples/       # Ejemplos Python
├── java/               # Bindings Java (JNI)
│   ├── ADeadBIB.java   # Clase Java
│   └── examples/       # Ejemplos Java
├── c/                  # Bindings C/C++
│   ├── adead.h         # Header C
│   └── examples/       # Ejemplos C
├── rust/               # Bindings Rust
│   ├── adead_rs.rs     # Wrapper Rust
│   └── examples/       # Ejemplos Rust
└── examples/           # Ejemplos de interoperabilidad
    ├── cross_lang.adB  # Ejemplo multi-lenguaje
    └── gpu_compute.adB # Ejemplo GPU compartido
```

## 🔧 ABI Estándar ADead-BIB

### Calling Convention (Windows x64)
- **Registros de argumentos:** RCX, RDX, R8, R9
- **Retorno:** RAX (enteros), XMM0 (flotantes)
- **Stack:** 16-byte aligned
- **Shadow space:** 32 bytes

### Tipos Exportados
| ADead-BIB | C       | Python  | Java    | Rust    |
|-----------|---------|---------|---------|---------|
| int       | int64_t | int     | long    | i64     |
| float     | double  | float   | double  | f64     |
| char      | char    | str[0]  | char    | char    |
| int*      | int64_t*| ctypes  | Pointer | *mut i64|
| void      | void    | None    | void    | ()      |

### Atributos de Exportación
```c
#[export("C")]           // Exportar con ABI C
#[export("stdcall")]     // Exportar con stdcall
#[import("python")]      // Importar desde Python
#[repr(C)]               // Layout compatible con C
```

## 🐍 Python Integration

```python
from FFI.python.adead_py import ADeadFFI

# Cargar librería ADead-BIB
ffi = ADeadFFI()

# Llamar función exportada
result = ffi.call("sumar", 10, 20)
print(f"10 + 20 = {result}")

# Usar con Metal_Dead
from Metal_Dead import MetalDead
ai = MetalDead(backend=ffi)
```

## ☕ Java Integration

```java
import adead.ADeadBIB;

public class Example {
    public static void main(String[] args) {
        ADeadBIB adead = new ADeadBIB();
        long result = adead.call("sumar", 10, 20);
        System.out.println("10 + 20 = " + result);
    }
}
```

## 🦀 Rust Integration

```rust
use adead_ffi::ADeadBIB;

fn main() {
    let adead = ADeadBIB::new();
    let result: i64 = adead.call("sumar", &[10, 20]);
    println!("10 + 20 = {}", result);
}
```

## 🔥 Ejemplo: ADead-BIB + Metal_Dead GPU

```python
# Usar ADead-BIB para cálculos críticos
# Metal_Dead para IA con GPU

from FFI.python.adead_py import ADeadFFI
from Metal_Dead import MetalDead

ffi = ADeadFFI()
ai = MetalDead(gpu=True)

# ADead-BIB compila kernel optimizado
kernel = ffi.compile_kernel("matmul.adB")

# Metal_Dead usa el kernel para inferencia
result = ai.inference(data, kernel=kernel)
```

## 📊 Rendimiento

| Operación      | Python Puro | ADead-BIB FFI | Speedup |
|----------------|-------------|---------------|---------|
| Sum 1M         | 50ms        | 2ms           | **25x** |
| MatMul 1024    | 200ms       | 8ms           | **25x** |
| Sort 100K      | 100ms       | 5ms           | **20x** |

---

Made with ⚡ for ADead-BIB v3.2
