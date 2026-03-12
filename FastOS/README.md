# FastOS v2.0 💀🦈
**El OS Definitivo — Puro, Mínimo, Sin Muletas**

> *"Un OS es un OS. No una muleta."*  
> *"El CPU ya sabe todo — solo hay que dejarlo recordar gradualmente."*  
> *"Los drivers van en el disco, no en el OS."*

**Compilador:** ADead-BIB (GPLv2)  
**Formato nativo:** `.Po` (24 bytes header)  
**Seguridad:** Binary Guardian (matemática pura, sin heurística)  
**Licencia:** Apache 2.0  
**Autor:** Eddi Andreé Salazar Matos 🇵🇪

---

## ¿Por qué FastOS existe?

Linux tiene 40 millones de líneas. El 60% son drivers.  
Windows necesita 4 GB solo para arrancar.  
Ambos cargan drivers de hardware que nadie usa, de impresoras que no están conectadas, de arquitecturas que no existen en tu máquina.

**Eso no es un OS. Es una muleta.**

FastOS es la respuesta a una pregunta simple:

> *¿Qué es un OS si solo contiene lo que debe contener?*

La respuesta: **~150 KB.**

---

## Filosofía Central

### 1. El CPU ya sabe todo — solo hay que dejarlo recordar

Cualquier CPU x86-64 moderno ya tiene quemado en silicio:
- Modos 16/32/64-bit, SSE, AVX2
- Controladores de teclado, VGA, PCI, interrupciones
- Toda la historia x86 desde 1978 hasta hoy

El problema de todos los OS es que **reimplementan en software lo que el CPU ya tiene en hardware.**

FastOS no reimplementa — **conversa con lo que ya existe.** Detecta capabilities via CPUID en runtime y activa solo lo que el CPU soporta.

### 2. Boot gradual — el CPU se calienta, no se aturde

```
16-bit real mode  →  32-bit protected  →  64-bit long mode
     (ASM)               (ASM)               (ASM)
```

No es un salto brusco. Es un despertar gradual.

Cada paso **cierra el contexto anterior** antes de abrir el siguiente.  
Cuando el kernel arranca, el CPU está completamente orientado — sin estado fantasma, sin contexto perdido.

Linux pelea contra el CPU en cada boot porque saltó sin calentar.  
FastOS **respeta cómo el CPU realmente funciona.**

### 3. Los drivers NO pertenecen al OS

```
Linux kernel  → 24 millones de líneas de drivers
               drivers de impresoras que no tienes
               drivers de hardware de 1995
               drivers de arquitecturas que no usas

FastOS kernel → 0 drivers incluidos
               el CPU detecta hardware via PCI scan
               busca driver .Po en disco
               BG verifica → APPROVED → ejecuta
               funciona. listo.
```

Los drivers viven en el disco. No en el OS.  
Igual que en 2026 nadie instala software desde CD — nadie necesita drivers pre-instalados.

### 4. Herencia inteligente — sin copiar el museo

FastOS hereda lo mejor de Linux y Windows **sin sus capas de legacy:**

| Hereda de Linux | Hereda de Windows | No hereda de nadie |
|---|---|---|
| Lógica de syscalls POSIX | Win32 API (via ADead-BIB) | 40M líneas de legacy |
| Concepto de proceso | Driver model moderno | Drivers pre-instalados |
| VFS design | DX12 support nativo | Bloatware |
| | | Telemetría |

Los drivers se compilan como binarios .Po con ADead-BIB y se verifican con BG antes de ejecutarse. Cada driver declara qué hardware necesita — BG verifica que el binario no excede lo declarado.

---

## Arquitectura

```
FastOS/
├── boot/
│   ├── stage1.asm          # MBR 512 bytes — FASM permanente
│   │                       # Solo despierta el CPU, carga stage2
│   ├── stage2.asm          # Transición gradual 16→32→64 — FASM
│   │                       # El CPU se calienta aquí
│   │                       # Configura GDT, paginación, long mode
│   │                       # Salta al kernel limpiamente
│   └── uefi/               # UEFI support (entrada directa 64-bit)
│
├── kernel/                  # Kernel core — C puro via ADead-BIB
│   ├── main.c              # Entry point — CPU ya está despierto aquí
│   ├── memory.c            # Paginación, heap
│   ├── interrupts.c        # IDT, IRQ handlers
│   ├── scheduler.c         # Round-robin preemptivo
│   ├── syscall.c           # POSIX-like + extensiones FastOS
│   ├── hotplug.c           # PCI scan → buscar driver .Po → BG verify → ejecutar
│   └── panic.c             # Kernel panic handler
│
├── security/               # Binary Guardian (hereda BG Rust crate via FFI)
│   ├── bg_core.c           # Wrapper C → BG Rust FFI (bg_init, bg_verify)
│   ├── bg_fastos.c         # Integración BG con kernel FastOS
│   ├── bg_levels.c         # 4 niveles de seguridad
│   ├── bg_preexec.c        # Gate pre-ejecución con cache FNV-1a
│   ├── bg/                 # Crate Rust: bg-fastos (staticlib)
│   │   ├── lib.rs          # Re-exports: BinaryGuardian, PolicyEngine
│   │   ├── analyzer.rs     # ISA analysis → ArchitectureMap
│   │   ├── arch_map.rs     # Mapa completo de capabilities
│   │   ├── capability.rs   # ISA instruction → capability mapping
│   │   ├── policy.rs       # SecurityPolicy + PolicyEngine::evaluate()
│   │   └── binary_loader.rs # Loader PE/ELF/.Po
│   └── Cargo.toml          # bg-fastos → libfg_fastos.a
│
├── fs/                     # Sistemas de archivos
│   └── vfs.c               # Virtual File System (futuro: FAT32, EXT2)
│
├── lib/                    # C runtime mínimo
│   ├── string.c
│   ├── memory.c
│   └── printf.c
│
├── include/
│   ├── kernel.h
│   ├── types.h
│   └── fastos.h            # FastOS native API
│
└── userspace/
    ├── shell.c             # Shell básico
    └── init.c              # Init process
```

**Nota:** No existe `/drivers/` en el kernel.  
Los drivers viven en el disco y se cargan bajo demanda.

---

## Boot — El Despertar Gradual

### stage1.asm (512 bytes exactos)
```asm
; MBR — solo despierta y carga stage2
; No hace transiciones de modo
; No maneja hardware complejo
; Su único trabajo: cargar stage2 y saltar
```

### stage2.asm (sin límite de tamaño)
```asm
; 16-bit: CPU en su modo original
;   → A20 line enable
;   → detecta memoria disponible

; 32-bit: protected mode
;   → GDT configurada correctamente
;   → descriptor tables limpias

; 64-bit: long mode
;   → PAE habilitado
;   → paginación configurada
;   → CPU completamente orientado
;   → salta a kernel/main.c

; El CPU llega al kernel DESPIERTO
; No aturdido. No con contexto perdido.
```

### kernel/main.c
```c
#include "kernel.h"

// El CPU llegó aquí gradualmente
// Ya sabe dónde está
// C solo dirige lo que ya existe

void kernel_main() {
    memory_init();
    interrupts_init();
    scheduler_init();
    hotplug_init();    // PCI scan, carga drivers .Po desde disco
    bg_init();         // Binary Guardian activo
    shell_start();
}
```

---

## Compilación — ADead-BIB

```bash
# Boot stages (ADead-BIB raw output)
adb cc boot/stage1.adB --raw --org=0x7C00 -o build/stage1.bin
adb cc boot/stage2.adB --raw --org=0x7E00 -o build/stage2.bin

# BG Rust crate → staticlib
cargo build --release --manifest-path security/Cargo.toml

# Kernel (ADead-BIB — sin GCC, sin linker externo)
adb cc kernel/main.c --flat --org=0x100000 -o build/kernel64.bin

# Verificar con step mode (7 fases)
adb step kernel/main.c
#  [SOURCE] → [PREPROC] → [LEXER] → [PARSER] → [UB] → [CODEGEN] → [OUTPUT]
#  0 UB encontrados ✅

# Imagen booteable
powershell -File build64.ps1
```

### Step Mode — Verificación Obligatoria

`adb step` muestra las 7 fases de compilación: source, preprocessor, lexer, parser, UB detector, codegen, output. El kernel DEBE compilar con **0 UB**. Step mode lo verifica antes de cada cambio.

---

## Sistema de Seguridad — Binary Guardian

BG es un **guardián determinista a nivel ISA**, heredado del crate Rust `BG — Binary Guardian`.

```
Binario → Loader → ISA Decoder → ADead-BIB IR → Capability Mapper
       → ArchitectureMap → PolicyEngine → APPROVE / DENY
```

**Pipeline real (implementado en Rust):**
- `BinaryLoader` — parsea PE/ELF/.Po
- `CapabilityMapper` — decodifica cada instrucción ISA → capabilities
- `ArchitectureMap` — mapa completo: IO ports, syscalls, hardware, memory, control flow
- `PolicyEngine::evaluate()` — evalúa mapa contra SecurityPolicy → Verdict

**4 Niveles de Seguridad:**

| Nivel | Nombre | Implementación |
|---|---|---|
| 1 | Auto Rebuild | Re-compila con ADead-BIB, compara hash |
| 2 | Human Firewall | Capabilities vs. permisos del proceso |
| 3 | Pre-Execution | Análisis ISA completo antes de mapear en memoria |
| 4 | Dead Man's Switch | Heartbeat + integrity check del kernel |

**Security Levels (heredados de `policy.rs`):**
- `Kernel (Ring 0)` — todo permitido
- `Driver (Ring 1)` — IO + interrupts, sin MSR/descriptor tables
- `Service (Ring 2)` — sin IO directo, solo syscalls autorizados
- `User (Ring 3)` — máximas restricciones

**Sin heurística. Sin "parece sospechoso".**  
Mismo binario + misma policy = mismo veredicto. Siempre. Determinista y matemático.

---

## Formato .Po — Ejecutable Nativo

```
Header: 24 bytes exactos

"FASTOS" (6 bytes)  — identidad
version  (2 bytes)  — compatibilidad
code_offset (4 bytes)
code_size   (4 bytes)
data_offset (4 bytes)
data_size   (4 bytes)
─────────────────────
Total: 24 bytes. Listo.
```

PE de Windows: ~1KB mínimo de header.  
ELF de Linux: 64 bytes + program headers + section headers.  
`.Po` de FastOS: **24 bytes. Solo lo que existe.**

---

## Tamaño del OS

| Componente | Tamaño |
|---|---|
| stage1.asm | 512 bytes |
| stage2.asm | ~3 KB |
| kernel core | ~50-100 KB |
| Binary Guardian | ~20 KB |
| lib/ mínima | ~10 KB |
| **Total FastOS** | **~150 KB** |

| OS | Tamaño mínimo |
|---|---|
| Windows 11 | 4 GB |
| Linux (minimal) | 100 MB |
| Android | 2 GB |
| **FastOS** | **~150 KB** |

---

## Compatibilidad

**FastOS no compite con Windows ni Linux.**  
FastOS demuestra que el diseño original del OS era correcto — y todos se alejaron de él.

```
Binarios .Po nativos   → FastOS ✓  (formato nativo 24-byte header)
BIOS Legacy boot       → ✓  (MBR stage1 → stage2 → kernel)
x86-64 Long Mode       → ✓  (boot gradual 16→32→64)
SSE/AVX2 detection     → ✓  (CPUID runtime, no hardcoded)
BG security gate       → ✓  (todo binario verificado pre-ejecución)
```

**Futuro:** Compatibilidad Win32/POSIX via ADead-BIB cross-compilation targets.

---

## Hotplug Inteligente

```
Boot → hotplug_init() escanea bus PCI
        ↓
Para cada dispositivo: buscar drivers/<vendor>_<device>.po en disco
        ↓
Cargar .Po → BG Pre-Execution Gate
        ↓
APPROVED → ejecutar como proceso Driver (Ring 1)
DENIED  → log de violaciones, hardware queda sin driver
```

Sin drivers pre-instalados en el kernel.  
Sin 24 millones de líneas de código de drivers.  
**El OS no sabe de hardware hasta que lo detecta. Los drivers viven en disco como binarios .Po verificados por BG.**

---

## Referencias

| Proyecto | Lo que FastOS estudia | Lo que FastOS rechaza |
|---|---|---|
| **Linux** | Syscalls POSIX, lógica de procesos | 40M líneas, drivers en kernel |
| **Windows NT** | Win32 API, driver model | Bloatware, telemetría, registry |
| **Nouveau** | Driver NVIDIA open-source | Complejidad innecesaria |
| **ReactOS** | Compatibilidad Win32 | Dependencia de legacy |
| **ToaruOS** | OS desde cero en C | — |

---

## Filosofía Final

> *Linux y Windows son ingeniería de decoración — agregaron capas sobre capas hasta que nadie recuerda por qué existe cada una.*
>
> *FastOS es ingeniería de definición — cada línea existe porque debe existir.*
>
> *Un OS no es una muleta. Es un espacio limpio donde los procesos viven.*
>
> *El CPU ya sabe todo. FastOS solo lo deja recordar.*

---

**FastOS v2.0 — ~150 KB. Sin muletas. Sin museo. Sin cuernos dorados falsos. 💀🦈🇵🇪**

```
ADead-BIB compila → FastOS arranca → CPU despierta gradual
drivers bajo demanda → Binary Guardian matemático
.Po 24 bytes → Win32 heredado → NVIDIA funciona
Todo en 150 KB.
```

*Hecho en Perú 🇵🇪 — Eddi Andreé Salazar Matos*