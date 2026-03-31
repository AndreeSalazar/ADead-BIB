# Arquitectura v3.0: PE/DLL vs ELF/SO + Preparación Mods

> **ADead-BIB v9.0** — Análisis de arquitecturas binarias para Linker Especial DLL/SO  
> Fecha: Marzo 2026 | Objetivo: Dominar exportación de bibliotecas nativas

---

## Resumen Ejecutivo

Este documento analiza las arquitecturas **PE (Windows)** y **ELF (Linux)** desde la perspectiva de ADead-BIB, enfocado específicamente en la generación de **DLLs** y **SOs** exportables. Incluye preparación para el sistema `mods/` que facilitará la integración externa.

---

## 1. Arquitectura PE (Portable Executable) — Windows

### 1.1 Estructura General

```
┌─────────────────────────────────────┐
│           DOS Header (64 bytes)     │
│  - Magic: "MZ" (0x4D 0x5A)          │
│  - e_lfanew: offset a PE header     │
├─────────────────────────────────────┤
│           DOS Stub (opcional)       │
│  "This program cannot be run..."    │
├─────────────────────────────────────┤
│  PE Signature: "PE\0\0" (4 bytes)   │
├─────────────────────────────────────┤
│      COFF File Header (20 bytes)    │
│  - Machine: 0x8664 (x64)            │
│  - NumberOfSections                 │
│  - SizeOfOptionalHeader             │
│  - Characteristics: EXECUTABLE_IMAGE│
│  - Characteristics: DLL (0x2000)    │  ← CRÍTICO para DLL
├─────────────────────────────────────┤
│   Optional Header (PE32+: 240 bytes)│
│  - Magic: 0x20B (PE32+)             │
│  - AddressOfEntryPoint              │
│  - ImageBase: 0x140000000 (typical) │
│  - SectionAlignment: 0x1000         │
│  - FileAlignment: 0x200             │
│  - MajorSubsystemVersion: 6         │
│  - SizeOfImage                      │
│  - Subsystem: 1 (console) / 2 (GUI) │
│  - DllCharacteristics               │
│  - DataDirectory[16]                │  ← Export Table aquí
├─────────────────────────────────────┤
│      Section Headers (40 bytes c/u) │
│  - .text (code)                     │
│  - .rdata (const data)              │
│  - .data (initialized data)         │
│  - .idata (imports)                 │
│  - .edata (exports) ← NUEVO para DLL│
│  - .reloc (relocation)              │
└─────────────────────────────────────┘
```

### 1.2 Export Address Table (EAT) — El Corazón de la DLL

**DataDirectory[0]: Export Table**

```
Export Directory Table (40 bytes):
┌──────────────────────────────────────┐
│ Export Flags              (4 bytes)  │  ← Reservado, 0
│ Time/Date Stamp           (4 bytes)  │
│ Major/Minor Version       (4 bytes)  │
│ Name RVA                  (4 bytes)  │  → "mylib.dll"\0
│ Ordinal Base              (4 bytes)  │  ← 1 (usualmente)
│ Address Table Entries     (4 bytes)  │  ← Número de funciones
│ Number of Name Pointers   (4 bytes)  │  ← Número de nombres exportados
│ Export Address Table RVA  (4 bytes)  │  → Array de RVAs
│ Name Pointer RVA          (4 bytes)  │  → Array de punteros a nombres
│ Ordinal Table RVA         (4 bytes)  │  → Array de ordinales
└──────────────────────────────────────┘
```

**Estructura de Exportación:**

```
Export Address Table (EAT):
┌─────────┬────────────────────────────────────┐
│ Ordinal │ RVA de la función en .text        │
├─────────┼────────────────────────────────────┤
│    1    │ 0x1060  → add()                   │
│    2    │ 0x1080  → factorial()             │
│    3    │ 0x10A0  → process_data()          │
└─────────┴────────────────────────────────────┘

Name Pointer Table:
┌─────────┬────────────────────────────────────┐
│  Index  │ RVA del nombre (en .edata)         │
├─────────┼────────────────────────────────────┤
│    0    │ → "add"\0                         │
│    1    │ → "factorial"\0                   │
│    2    │ → "process_data"\0                │
└─────────┴────────────────────────────────────┘

Ordinal Table:
┌─────────┬────────────────────────────────────┐
│  Index  │ Ordinal (índice en EAT)            │
├─────────┼────────────────────────────────────┤
│    0    │ 1                                  │
│    1    │ 2                                  │
│    2    │ 3                                  │
└─────────┴────────────────────────────────────┘
```

### 1.3 Entry Point DllMain (vs main en EXE)

```cpp
// EXE entry point
int main(int argc, char *argv[]);

// DLL entry point
BOOL APIENTRY DllMain(
    HINSTANCE hinstDLL,   // Handle a la DLL
    DWORD fdwReason,      // DLL_PROCESS_ATTACH/DETACH/THREAD_ATTACH/DETACH
    LPVOID lpvReserved    // Reservado
) {
    switch (fdwReason) {
        case DLL_PROCESS_ATTACH:
            // Inicialización al cargar
            break;
        case DLL_PROCESS_DETACH:
            // Limpieza al descargar
            break;
    }
    return TRUE;
}
```

**En ADead-BIB:** Tu encoder actual genera entry point para EXE. Para DLL necesitas:
1. Cambiar `Characteristics` para incluir flag DLL (0x2000)
2. Entry point apunta a `DllMain` en vez de `main`
3. Agregar sección `.edata` con EAT

### 1.4 Código de Exportación (Syntax)

```cpp
// Forma MSVC
__declspec(dllexport) int add(int a, int b) {
    return a + b;
}

// Forma portable (ADead-BIB)
#ifdef _WIN32
    #define ADB_EXPORT __declspec(dllexport)
#else
    #define ADB_EXPORT __attribute__((visibility("default")))
#endif

extern "C" {
    ADB_EXPORT int add(int a, int b);
    ADB_EXPORT void process_buffer(char *buf, int len);
}
```

---

## 2. Arquitectura ELF (Executable and Linkable Format) — Linux

### 2.1 Estructura General

```
┌─────────────────────────────────────┐
│        ELF Header (64 bytes)          │
│  - Magic: 0x7F 'E' 'L' 'F'           │
│  - Class: 2 (64-bit)                │
│  - Data: 1 (little endian)          │
│  - OS/ABI: 0 (System V) / 3 (Linux)   │
│  - Type: 3 (shared object) ← .SO    │
│  - Machine: 62 (x86-64)             │
│  - Entry point address                │
│  - Program header offset              │
│  - Section header offset              │
│  - e_flags, e_ehsize                 │
├─────────────────────────────────────┤
│     Program Headers (arrays)          │
│  Describen segmentos en memoria       │
├─────────────────────────────────────┤
│     Section Headers (arrays)        │
│  Describen secciones en disco         │
├─────────────────────────────────────┤
│  Sections:                            │
│  - .text (PROGBITS, AX)               │
│  - .rodata (PROGBITS, A)              │
│  - .data (PROGBITS, WA)               │
│  - .bss (NOBITS, WA)                  │
│  - .symtab (SYMTAB)                   │
│  - .strtab (STRTAB)                   │
│  - .dynsym (DYNSYM) ← Exports SO      │
│  - .dynstr (STRTAB)                   │
│  - .hash / .gnu.hash                  │
│  - .dynamic (DYNAMIC)                 │
│  - .init / .fini                      │
│  - .init_array / .fini_array          │
└─────────────────────────────────────┘
```

### 2.2 Dynamic Symbol Table (.dynsym) — Exportación SO

**Estructura Elf64_Sym (24 bytes):**

```
┌─────────────────────────────────────┐
│ st_name: 4 bytes    │ Index en .dynstr
│ st_info: 1 byte     │ Tipo + Binding
│ st_other: 1 byte    │ Visibility
│ st_shndx: 2 bytes   │ Sección
│ st_value: 8 bytes   │ Dirección virtual
│ st_size: 8 bytes    │ Tamaño del símbolo
└─────────────────────────────────────┘

Binding:
  - STB_LOCAL (0): No exportado
  - STB_GLOBAL (1): Exportado globalmente ← TU FUNCIÓN
  - STB_WEAK (2): Weak symbol

Type:
  - STT_NOTYPE (0)
  - STT_OBJECT (1): Datos
  - STT_FUNC (2): Función ← TU FUNCIÓN
  - STT_SECTION (3)
```

### 2.3 Sección .dynamic — Metadatos del SO

```
Tag           │ Value                    │ Significado
──────────────┼──────────────────────────┼─────────────────────────
DT_HASH       │ address of .hash        │ Tabla hash símbolos
DT_GNU_HASH   │ address of .gnu.hash    │ Hash GNU (faster)
DT_STRTAB     │ address of .dynstr      │ String table
DT_SYMTAB     │ address of .dynsym      │ Symbol table
DT_STRSZ      │ size of string table    │
DT_SYMENT     │ size of symbol entry    │ 24 para Elf64_Sym
DT_INIT       │ address of _init        │ Constructor
DT_FINI       │ address of _fini        │ Destructor
DT_INIT_ARRAY │ address of init array   │ Array constructores
```

### 2.4 Entry Points en SO

```cpp
// Funciones especiales llamadas por dynamic loader
void _init(void) __attribute__((constructor));
void _fini(void) __attribute__((destructor));

// O mejor (moderno):
__attribute__((constructor)) void my_init(void) {
    // Se ejecuta al cargar la SO
}

__attribute__((destructor)) void my_fini(void) {
    // Se ejecuta al descargar la SO
}

// Funciones exportadas
__attribute__((visibility("default"))) int add(int a, int b) {
    return a + b;
}
```

### 2.5 Visibility Attributes

```cpp
// Por defecto: símbolos son globales (visibles)
// Para ocultar: __attribute__((visibility("hidden")))

// Macro portable ADead-BIB
#if defined(__linux__) || defined(__unix__)
    #define ADB_EXPORT __attribute__((visibility("default")))
    #define ADB_HIDDEN __attribute__((visibility("hidden")))
#elif defined(_WIN32)
    #define ADB_EXPORT __declspec(dllexport)
    #define ADB_HIDDEN
#endif
```

---

## 3. Comparativa PE vs ELF — Perspectiva ADead-BIB

| Aspecto | PE (Windows DLL) | ELF (Linux SO) | Implicación ADead-BIB |
|---------|------------------|----------------|----------------------|
| **Formato** | Binary, little-endian | Binary, little-endian | ✅ Tu encoder ya maneja binario |
| **Header size** | ~512 bytes (DOS+PE+Optional) | 64 bytes (ELF64) | ✅ Manejable |
| **Export mechanism** | EAT (Export Address Table) | .dynsym (Dynamic Symbols) | ⚠️ Dos formatos diferentes |
| **Export count** | Counted arrays | Linked list via hash | ⚠️ PE más simple para pocos exports |
| **Entry point** | DllMain | _init/_fini / constructor | ⚠️ Diferente semántica |
| **String table** | RVA-based in .edata | Offset-based in .dynstr | ⚠️ Formato diferente |
| **Base address** | Preferida: 0x140000000 | 0 (PIC/PIE) | ✅ Tu PE usa base fija |
| **Relocation** | .reloc section | RELA entries | ⚠️ Necesitas ambos |
| **Imports** | IAT in .idata | PLT/GOT in .got.plt | ✅ Ya tienes IAT PE |

### 3.1 Complejidad de Implementación

```
PE Export Table (EAT):
┌─────────────────────────────────────┐
│ Fácil: Array de RVAs simple         │
│ Nombres: Tabla paralela de strings  │
│ Ordinales: Tabla de índices         │
│ Búsqueda: Lineal O(n)               │
│ Tamaño estimado: 100-500 bytes      │
└─────────────────────────────────────┘
        vs
ELF .dynsym:
┌─────────────────────────────────────┐
│ Medio: Estructura fija por símbolo  │
│ Hash table: .hash o .gnu.hash       │
│ Secciones múltiples coordinadas     │
│ Búsqueda: Hash O(1) promedio        │
│ Tamaño estimado: 200-1000 bytes     │
└─────────────────────────────────────┘
```

**Conclusión:** PE/EAT es más simple para pocas funciones exportadas. ELF requiere más coordinación entre secciones pero es más eficiente en runtime.

---

## 4. Qué Tiene ADead-BIB Actualmente

### 4.1 Componentes Existentes (v8.0 → v9.0)

| Componente | Estado | Usado para | Para DLL/SO necesitas |
|------------|--------|-----------|----------------------|
| `encoder.rs` PE | ✅ Funcional | .exe con IAT | Extender para EAT |
| `encoder.rs` ELF | ✅ Funcional | .bin Linux | Extender para .dynsym |
| `isa_compiler.rs` | ✅ Genera x86-64 | Código nativo | Igual |
| `frontend C/C++` | ✅ IR completo | Parsing, types | Agregar atributos export |
| `IAT generation` | ✅ Import table | Llamar a DLLs externas | Nada, nuevo feature |
| `Stack frame` | ✅ RBP-based | Funciones locales | Igual |
| `Class layout` | ✅ Vtable + fields | OOP | Exportar vtables? |
| `Call ABI` | ✅ Windows x64 | Llamadas de funciones | Igual para exports |

### 4.2 Gap Analysis — Qué Falta

```
ADead-BIB v8.0:
├─ PE Encoder
│  ├─ DOS Header ✅
│  ├─ COFF Header ✅
│  ├─ Optional Header ✅
│  ├─ Section Headers ✅
│  ├─ .text ✅
│  ├─ .data ✅
│  ├─ .idata (IAT) ✅
│  └─ EAT (Export) ❌ ← AGREGAR
│
├─ ELF Encoder
│  ├─ ELF Header ✅
│  ├─ Program Headers ✅
│  ├─ Section Headers ✅
│  ├─ Sections (.text, .data) ✅
│  ├─ Symbol Table (local) ✅
│  └─ Dynamic Symbols (.dynsym) ❌ ← AGREGAR
│
└─ Frontend IR
   ├─ Functions ✅
   ├─ Classes ✅
   └─ Export attributes ❌ ← AGREGAR
```

### 4.3 Estructuras de Datos a Agregar

```rust
// PE Export Table
pub struct ExportTable {
    pub dll_name: String,           // "mylib.dll"
    pub ordinal_base: u32,          // 1
    pub exports: Vec<Export>,       // Funciones exportadas
}

pub struct Export {
    pub name: String,               // "add"
    pub ordinal: u16,               // 1, 2, 3...
    pub rva: u32,                   // RVA en .text
    pub is_ordinal_only: bool,    // Export por ordinal sin nombre
}

// ELF Dynamic Symbol
pub struct DynamicSymbol {
    pub name: String,
    pub value: u64,                 // Dirección virtual
    pub size: u64,
    pub binding: SymbolBinding,     // GLOBAL
    pub symbol_type: SymbolType,    // FUNC
    pub visibility: u8,             // DEFAULT
}

pub struct DynamicSection {
    pub symbols: Vec<DynamicSymbol>,
    pub string_table: Vec<u8>,      // .dynstr
    pub hash_table: Option<HashTable>,
}
```

---

## 5. Sistema `mods/` — Zero Friction para Modders

### 5.1 Filosofía: Sin Pelearse con Nada

```
Problema actual del modder:
├─ Linker flags incomprensibles (/EXPORT, --shared)
├─ Calling conventions (__cdecl, __stdcall, __fastcall)
├─ Headers complejos (Windows.h, dlfcn.h)
├─ Dependencias circulares
└─ DLL Hell

Solución ADead-BIB v9.0:
adB import opengl32.dll   → Listo, usable desde tu código
adB export mi_mod.c       → DLL limpia, lista para C#/Python/Unity
```

### 5.2 Estructura de Carpetas Nativa

```
ADead-BIB/                         ← Tu compilador (portable, 5MB)
├── mods/
│   ├── imports/                   ← DLLs/SOs externos analizados
│   │   ├── opengl32.dll.json      ← Metadata auto-generada
│   │   ├── kernel32.dll.json
│   │   ├── user32.dll.json
│   │   └── mi_engine.dll/         ← DLL de terceros que importaste
│   │       ├── symbols.json       ← Funciones detectadas
│   │       ├── headers.h          ← Headers auto-generados
│   │       └── mi_engine.dll.lnk  ← Link al original
│   │
│   ├── exports/                   ← Tus DLLs generadas (listas para usar)
│   │   ├── my_physics.dll         ← 8KB, sin deps
│   │   ├── my_ai.dll
│   │   ├── libmy_utils.so         ← Linux version
│   │   └── registry.json          ← Catálogo de tus exports
│   │
│   └── registry/                  ← Índice global disponible
│       ├── local.json             ← Tus mods locales
│       ├── cached.json            │
│       └── remote/                ← Descargas de repositorios
│
└── mi_proyecto/
    ├── main.cpp
    └── mods/                      ← Mods específicos del proyecto
        └── physics/               ← Código fuente del mod
            ├── mod.json
            └── src/
                └── physics.cpp
```

### 5.3 CLI v9.0 — Comandos Import/Export

```bash
# ═══════════════════════════════════════════════════════════════
# IMPORT: Traer DLLs externas al ecosistema ADead-BIB
# ═══════════════════════════════════════════════════════════════

# Importar DLL del sistema — análisis automático de exports
adB import opengl32.dll
# → Analiza exports EAT
# → Genera mods/imports/opengl32.dll.json
# → Crea headers adaptados
# → Listo para usar: #include <imports/opengl32.h>

# Importar DLL de terceros con análisis profundo
adB import ./mi_engine.dll --deep
# → Detecta: init_engine(), render(), cleanup()
# → Genera bindings C/C++ nativos
# → Crea wrapper de tipos automático

# Importar SO de Linux (desde Windows con cross-info)
adB import ./libcuda.so --platform linux
# → Analiza estructura ELF .dynsym
# → Prepara para cross-compilation

# Ver qué has importado
adB import list
# → kernel32.dll    [40 funciones]  ← system
# → opengl32.dll    [358 funciones] ← system  
# → mi_engine.dll   [12 funciones]  ← ./third_party/
#     ├─ init_engine()
#     ├─ render_frame(int width, int height)
#     └─ ...

# ═══════════════════════════════════════════════════════════════
# EXPORT: Generar DLLs/SOs limpios desde código fuente
# ═══════════════════════════════════════════════════════════════

# Exportar función simple — DLL Windows
adB export mi_modulo.c -o my_physics.dll
# → Compila C99 → x86-64
# → Genera PE con EAT
# → Coloca en mods/exports/my_physics.dll
# → Zero deps, 2-5KB típico

# Exportar C++ con clases — funciones mangled automáticas
adB export mi_engine.cpp --cpp -o game_engine.dll
# → Exporta clases como interface C (vtable wrapper)
# → Genera headers de consumo (.h)

# Exportar para Linux
adB export mi_modulo.c --so -o libmi_modulo.so
# → ELF64 con .dynsym
# → Visibility default para exports

# Exportar biblioteca dual (Windows + Linux)
adB export mi_modulo.c --dual -o mi_mod
# → Genera: mi_mod.dll + libmi_mod.so
# → Mismo código, dos formatos

# Ver exports generados
adB export list
# → my_physics.dll   [5 funciones]  ← mods/exports/
# → game_engine.dll  [23 funciones] ← mods/exports/
# → libmy_utils.so   [8 funciones] ← mods/exports/

# ═══════════════════════════════════════════════════════════════
# USO: Consumir imports y exports en tu código
# ═══════════════════════════════════════════════════════════════
```

### 5.4 Sintaxis de Código — Import/Export Nativo

```cpp
// ═══════════════════════════════════════════════════════════════
// CÓDIGO DEL MODDER: Exportar funciones
// ═══════════════════════════════════════════════════════════════

// mi_physics.cpp
#pragma adB export                    ← Declaración de exportación

// Funciones exportadas automáticamente
export vec3 gravity(0.0f, -9.81f, 0.0f);

export float calculate_velocity(float t) {
    return gravity.y * t;
}

export struct PhysicsBody {
    vec3 position;
    vec3 velocity;
    float mass;
    
    void apply_force(vec3 force) {
        velocity = velocity + (force / mass);
    }
};

// ═══════════════════════════════════════════════════════════════
// CÓDIGO DEL CONSUMIDOR: Usar imports
// ═══════════════════════════════════════════════════════════════

// main.cpp
#pragma adB import opengl32           ← Import de sistema
#pragma adB import mi_physics           ← Import de tu mod

int main() {
    // Usar función importada directamente
    opengl32::glClear(0x00004000);     // GL_COLOR_BUFFER_BIT
    
    // Usar tu mod
    mi_physics::PhysicsBody player;
    player.mass = 75.0f;
    player.apply_force({0, 500, 0});   // Salto
    
    return 0;
}
```

### 5.5 Zero Configuration Workflow

```bash
# Paso 1: Modder crea funciones en C/C++
cat > mi_mod.cpp << 'EOF'
export int add(int a, int b) { return a + b; }
EOF

# Paso 2: Exporta como DLL
adB export mi_mod.cpp -o mi_mod.dll
# Output: mods/exports/mi_mod.dll (2.3KB, 1 función)

# Paso 3: Cualquier programa la usa
# Python:
python -c "from ctypes import CDLL; print(CDLL('./mi_mod.dll').add(5,3))"
# → 8

# C#:
dotnet run  // [DllImport("mi_mod.dll")] static extern int add(int,int);
# → 8

# Unity:
[DllImport("mi_mod.dll")] private static extern int add(int, int);
Debug.Log(add(5, 3));  // → 8
```

### 5.6 Sistema Registry — Catálogo Automático

```json
// mods/registry/local.json
{
  "version": "1.0",
  "updated": "2026-03-30T22:00:00Z",
  "imports": {
    "system": [
      {"name": "kernel32.dll", "functions": 40, "size": "auto-analyzed"},
      {"name": "opengl32.dll", "functions": 358, "size": "auto-analyzed"},
      {"name": "user32.dll", "functions": 89, "size": "auto-analyzed"}
    ],
    "third_party": [
      {"name": "mi_engine.dll", "path": "./third_party/", "functions": 12}
    ]
  },
  "exports": {
    "mi_physics.dll": {
      "version": "1.2.0",
      "functions": ["calculate_velocity", "gravity", "PhysicsBody::apply_force"],
      "size_bytes": 4864,
      "platforms": ["windows-x64"],
      "checksum": "sha256:abc123..."
    },
    "game_engine.dll": {
      "version": "0.9.1",
      "functions": ["init_engine", "render_frame", "cleanup"],
      "size_bytes": 12288,
      "platforms": ["windows-x64", "linux-x64"],
      "checksum": "sha256:def456..."
    }
  }
}
```

### 5.7 Beneficios para Modders

| Sin ADead-BIB | Con ADead-BIB v9.0 |
|---------------|-------------------|
| Aprender `__declspec(dllexport)` | `export` keyword nativo |
| Manejar `.def` files | Auto-generación de EAT |
| Linker flags `/EXPORT` | `adB export archivo.c` |
| Headers de Windows | `adB import kernel32.dll` |
| Dependency Walker | `adB import list` con análisis |
| DLL Hell manual | Registry + checksums |
| Cada lenguaje diferente | Un formato, todos lo usan |

### 5.8 Arquitectura Interna Import/Export

```
Import Pipeline:
┌────────────────────────────────────────────────────────────┐
│  adB import opengl32.dll                                   │
│        ↓                                                   │
│  ┌─────────────────┐    ┌─────────────────┐                │
│  │ PE Parser       │ →  │ EAT Extractor   │                │
│  │ - DOS Header    │    │ - Export names  │                │
│  │ - COFF Header   │    │ - RVAs          │                │
│  │ - Optional Hdr  │    │ - Ordinals      │                │
│  └─────────────────┘    └─────────────────┘                │
│        ↓                        ↓                          │
│  ┌─────────────────┐    ┌─────────────────┐                │
│  │ Type Inference  │ ←  │ Symbol Table    │                │
│  │ (heuristic)     │    │                 │                │
│  └─────────────────┘    └─────────────────┘                │
│        ↓                                                   │
│  ┌─────────────────┐                                       │
│  │ Header Gen      │ → opengl32.h con:                     │
│  │                 │    extern "C" {                       │
│  │                 │      void glClear(unsigned int mask); │
│  │                 │    }                                  │
│  └─────────────────┘                                       │
│        ↓                                                   │
│  mods/imports/opengl32.dll.json                            │
└────────────────────────────────────────────────────────────┘

Export Pipeline:
┌──────────────────────────────────────────────────────────────┐
│  adB export mi_mod.c -o mi_mod.dll                           │
│        ↓                                                     │
│  ┌─────────────────┐                                         │
│  │ Frontend C/C++  │ → AST con nodos 'export'                │
│  │                 │   (no diferencia con funciones normales)│
│  └─────────────────┘                                         │
│        ↓                                                     │
│  ┌─────────────────┐                                         │
│  │ IR Gen          │ → Funciones marcadas como 'exported'    │
│  │                 │   en metadata del módulo                │
│  └─────────────────┘                                         │
│        ↓                                                     │
│  ┌─────────────────┐                                         │
│  │ ISA Compiler    │ → x86-64 bytes (igual que .exe)         │
│  │                 │                                         │
│  └─────────────────┘                                         │
│        ↓                                                     │
│  ┌─────────────────┐    ┌─────────────────┐                  │
│  │ PE Encoder      │ ←  │ EAT Builder     │                  │
│  │ - COFF: DLL flag│    │ - Export names  │                  │
│  │ - Entry: DllMain│    │ - Function RVAs │                  │
│  │ - Sections      │    │ - Ordinal table │                  │
│  └─────────────────┘    └─────────────────┘                  │
│        ↓                                                     │
│  mods/exports/mi_mod.dll (2.5KB)                             │
└──────────────────────────────────────────────────────────────┘
```

---

## 6. Roadmap de Implementación

### Fase 1: Fundamentos DLL/SO (v9.0 core)

| Tarea | Complejidad | Archivos |
|-------|-------------|----------|
| PE EAT generation | Media | `encoder.rs` |
| ELF .dynsym generation | Media-Alta | `encoder.rs` |
| Export attribute parsing | Baja | `frontend/cpp/cpp_to_ir.rs` |
| `export` keyword handling | Media | `ast.rs`, `to_ir.rs` |
| CLI `--dll`, `--so` flags | Baja | `main.rs`, `cli/mod.rs` |

### Fase 2: Sistema mods/ (v9.1)

| Tarea | Complejidad | Archivos |
|-------|-------------|----------|
| `import` statement parsing | Media | `cpp_to_ir.rs` |
| Mod resolution algorithm | Media | `driver/mod.rs` |
| mod.json parser | Baja | Nuevo: `mods/manifest.rs` |
| Auto-compilation de dependencias | Media | `driver/mod.rs` |
| CLI `mod` subcommand | Baja | `main.rs` |

### Fase 3: Ecosistema (v9.2+)

| Tarea | Complejidad |
|-------|-------------|
| Registry local de mods | Media |
| `adB mod install` (git/curl) | Media |
| Semantic versioning resolver | Alta |
| Mod testing framework | Media |

---

## 7. Ejemplo Completo: De Source a DLL

### Input

```cpp
// mylib.cpp
export int add(int a, int b) {
    return a + b;
}

export int factorial(int n) {
    return n <= 1 ? 1 : n * factorial(n - 1);
}
```

### CLI

```bash
adB cxx mylib.cpp --dll -o mylib.dll
# o
adB cxx mylib.cpp --so -o libmylib.so
```

### Output PE (mylib.dll)

```
mylib.dll (2.5KB):
├── DOS Header (64 bytes)
├── PE Header + COFF (24 bytes)
├── Optional Header (240 bytes)
├── Section Headers (5 × 40 = 200 bytes)
├── .text (512 bytes)
│   ├── DllMain (prologo)
│   ├── add() @ 0x1060
│   └── factorial() @ 0x1080
├── .edata (128 bytes)
│   ├── Export Directory Table
│   ├── Export Address Table [0x1060, 0x1080]
│   ├── Name Pointer Table ["add", "factorial"]
│   └── Ordinal Table [1, 2]
└── .reloc (64 bytes)
```

### Output ELF (libmylib.so)

```
libmylib.so (3.2KB):
├── ELF Header (64 bytes)
├── Program Headers (7 × 56 = 392 bytes)
├── .text (512 bytes)
├── .rodata (strings)
├── .dynsym (2 × 24 = 48 bytes)
│   ├── add: GLOBAL, FUNC, @0x1060
│   └── factorial: GLOBAL, FUNC, @0x1080
├── .dynstr ("add\0factorial\0")
├── .hash (hash table)
├── .dynamic (tags de metadata)
└── Section Headers (12 × 64 = 768 bytes)
```

---

## 8. Consumo desde Otros Lenguajes

### C#

```csharp
using System.Runtime.InteropServices;

class NativeLib {
    [DllImport("mylib.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int add(int a, int b);
    
    [DllImport("mylib.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int factorial(int n);
}

// Uso
int result = NativeLib.add(5, 3);  // 8
```

### Python

```python
import ctypes

# Windows
mylib = ctypes.CDLL("./mylib.dll")
# Linux
# mylib = ctypes.CDLL("./libmylib.so")

mylib.add.argtypes = [ctypes.c_int, ctypes.c_int]
mylib.add.restype = ctypes.c_int

result = mylib.add(5, 3)  # 8
```

### Node.js

```javascript
const koffi = require('koffi');

const mylib = koffi.load('./mylib.dll');
const add = mylib.func('int add(int, int)');

console.log(add(5, 3));  // 8
```

---

## Referencias

- **PE Format**: Microsoft PE and COFF Specification (Revision 11.0)
- **ELF Format**: System V ABI, AMD64 Architecture Processor Supplement
- **Windows x64 ABI**: Microsoft x64 Calling Convention
- **Linux x64 ABI**: System V AMD64 ABI (version 1.0)

---

> **Documento v3.0** | ADead-BIB Architecture Reference  
> Para implementación de Linker Especial DLL/SO v9.0
