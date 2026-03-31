# ADead-BIB v9.0 — Arquitectura DLL/SO + Mods 💀🦈
> Eddi Andreé Salazar Matos | Lima, Perú 🇵🇪 | Marzo 2026 | Techne v1.0

---

## 1. Estado Actual — Qué existe vs qué falta

### PE (Windows DLL)
| Componente | Estado | Nota |
|---|---|---|
| DOS + COFF + Optional Header | ✅ | Base sólida |
| `.text` `.data` `.idata` (IAT) | ✅ | 63 slots, 5 DLLs |
| idata_rva dinámico | ✅ | Sin límite .text |
| COFF DLL flag `0x2000` | ❌ | 1 línea — CRÍTICO |
| `.edata` Export Table (EAT) | ❌ | Núcleo de la DLL |
| `DllMain` auto-emission | ❌ | Entry point distinto a main |
| `.reloc` section | ❌ | Reubicación base |

### ELF (Linux SO)
| Componente | Estado | Nota |
|---|---|---|
| ELF Header, Program/Section Headers | ✅ | Base sólida |
| `.text` `.data` `.bss` | ✅ | OK |
| `e_type = ET_DYN` | ❌ | Actualmente ET_EXEC |
| `.dynsym` + `.dynstr` | ❌ | Exports del SO |
| `.dynamic` section | ❌ | Metadata requerida |
| `.gnu.hash` | ❌ | Búsqueda O(1) |
| `.init` / `.fini` | ❌ | Constructor/destructor SO |

### Frontend + Mods
| Componente | Estado | Nota |
|---|---|---|
| `export` keyword en parser | ❌ | C/C++ no lo reconoce |
| `ExportMeta` en IR/AST | ❌ | Falta campo en `Function` |
| `adB export` / `adB import` CLI | ❌ | Comandos nuevos |
| EAT Analyzer (leer DLLs externas) | ❌ | Import pipeline |
| Header auto-generator | ❌ | `.h` desde symbols |
| `mod.json` manifest | ❌ | Descriptor de mod |
| `mods/` folder registry | ❌ | Catálogo local |

---

## 2. Piezas Críticas — En orden de implementación

```
1. COFF flag 0x2000          → pe.rs          — 1 línea
2. EAT builder (.edata)      → pe.rs          — núcleo DLL
3. DllMain auto-emission     → isa_compiler.rs — entry point
4. export keyword            → parser.rs + ast.rs
5. CLI --dll / --so / --dual → main.rs
6. ELF ET_DYN + .dynsym      → elf.rs
7. EAT Analyzer (import)     → nuevo: eat_reader.rs
8. Header Generator          → nuevo: header_gen.rs
9. mods/ + mod.json          → nuevo: mods/manifest.rs
```

---

## 3. Estructuras Rust — Lo que hay que agregar

```rust
// adeb-core/src/ast.rs — Export metadata en IR
pub struct ExportMeta {
    pub exported: bool,
    pub extern_c: bool,
    pub ordinal: Option<u16>,
    pub alias: Option<String>,
}

pub struct Function {
    // ... campos existentes ...
    pub export_meta: Option<ExportMeta>, // ← NUEVO
}

pub struct Program {
    // ... campos existentes ...
    pub exports: Vec<String>,        // ← NUEVO
    pub dll_name: Option<String>,    // ← NUEVO
}

// adeb-platform/src/pe.rs — Export Table
pub struct Export {
    pub name: String,
    pub ordinal: u16,
    pub rva: u32,
}

pub struct ExportTable {
    pub dll_name: String,
    pub ordinal_base: u32,  // siempre 1
    pub exports: Vec<Export>,
}

// adeb-platform/src/elf.rs — Dynamic Symbol
pub struct DynamicSymbol {
    pub name: String,
    pub value: u64,   // dirección virtual
    pub size: u64,
    pub binding: SymbolBinding,  // GLOBAL
    pub sym_type: SymbolType,    // FUNC
}
```

---

## 4. Pipeline Completo

### Export: Source → DLL/SO
```
mi_mod.cpp
  → CppPreprocessor → CppLexer → CppParser
  → CppToIR         (detecta 'export' → ExportMeta)
  → UB Detector     (estricto implícito — 0 UB o error)
  → ISA Compiler    (x86-64, registra RVAs de exports)
  → PE Encoder      → DllMain + EAT + COFF 0x2000 → mi_mod.dll
  → ELF Encoder     → ET_DYN + .dynsym + .dynamic  → libmi_mod.so
  → Header Gen      → mi_mod.h auto-generado
```

### Import: DLL externa → headers
```
opengl32.dll
  → PE Parser       (lee EAT)
  → EAT Extractor   (358 funciones: glClear, wglCreateContext...)
  → Type Inference  (heurística por nombre)
  → Header Gen      → opengl32.h
  → Registry        → mods/imports/opengl32.dll.json
```

---

## 5. CLI — Comandos nuevos

```bash
# Export
adB export mi_mod.c -o mi_mod.dll        # C → DLL
adB export mi_mod.c --so -o libmi.so     # C → SO Linux
adB export mi_mod.c --dual -o mi_mod     # DLL + SO simultáneo
adB export mi_mod.cpp --cpp -o mi_mod.dll

# Import
adB import opengl32.dll                  # analiza EAT → .h + .json
adB import ./libcuda.so --platform linux # analiza .dynsym ELF
adB import list

# Mods
adB mod new my_physics
adB mod build
adB mod pack
adB mod list
```

---

## 6. Sintaxis — export keyword

```cpp
// Forma nativa ADead-BIB
#pragma adB export

export int add(int a, int b) { return a + b; }
export float velocity(float t) { return -9.81f * t; }

// Forma portable (también soportada)
#define ADB_EXPORT __declspec(dllexport)   // Windows
#define ADB_EXPORT __attribute__((visibility("default"))) // Linux

extern "C" {
    ADB_EXPORT int add(int a, int b);
}
```

---

## 7. Consumo — Cualquier lenguaje

```python
# Python
import ctypes
lib = ctypes.CDLL("./mi_mod.dll")
lib.add.restype = ctypes.c_int
print(lib.add(5, 3))  # → 8
```
```csharp
// C# / Unity
[DllImport("mi_mod.dll")]
static extern int add(int a, int b);
Debug.Log(add(5, 3));  // → 8
```
```javascript
// Node.js
const lib = koffi.load("./mi_mod.dll");
const add = lib.func("int add(int, int)");
console.log(add(5, 3));  // → 8
```

---

## 8. mods/ — Estructura de carpetas

```
ADead-BIB/
└── mods/
    ├── imports/
    │   ├── opengl32.dll.json   ← metadata auto
    │   └── opengl32.h          ← header auto
    ├── exports/
    │   ├── mi_mod.dll          ← 2-8KB, 0 UB, 0 deps
    │   ├── libmi_mod.so        ← Linux version
    │   ├── mi_mod.h            ← header auto
    │   └── registry.json       ← catálogo
    └── registry/
        └── local.json
```

---

## 9. Roadmap

| Fase | Qué | Cuándo |
|---|---|---|
| **v9.1** | EAT + DLL flag + DllMain + export keyword + CLI | Próximo |
| **v9.2** | ELF .dynsym + .dynamic + import analyzer + mods/ | Después |
| **v9.3** | mod pack/install + remote registry + versioning | Futuro |

---

## 10. Impacto final

```
Sin ADead-BIB         → linker flags + .def + .lib + dumpbin + Visual Studio
Con ADead-BIB         → adB export mi_mod.c -o mi_mod.dll → listo

DLL generada          → 2-8KB, sin CRT, sin deps, 0 UB garantizado
Cualquier empresa     → integra sin migrar nada
FastOS                → hereda todo — DLL nativas del OS propio
Techne License        → 10-20% royalties sobre lo atribuible
```

> *"Bits respetados. Sin UB. Sin linker hell. Solo machine code puro. 💀🦈"*
