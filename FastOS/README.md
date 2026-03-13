# FastOS v4.0 💀🦈 🇵🇪
**El OS Definitivo — GUI Nativo VESA VBE + 256-bit + Sin Muletas**

> *"Un OS es un OS. No una muleta."*  
> *"El CPU ya sabe todo — solo hay que dejarlo recordar gradualmente."*  
> *"ASM despierta. C controla. 256-bit vuela."*

**Compilador:** ADead-BIB v8.0 (GPLv2)  
**Formato nativo:** `.Po` v8.0 (32 bytes header, 256-bit YMM)  
**GUI:** VESA VBE 1024×768×32 — sin X11, sin Wayland, sin GDI  
**Rendering:** AVX2 256-bit — 8 píxeles/ciclo via YMM  
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

La respuesta: **~200 KB con GUI completo.**

---

## Filosofía Central

### 1. El CPU ya sabe todo — solo hay que dejarlo recordar

Cualquier CPU x86-64 moderno ya tiene quemado en silicio:
- Modos 16/32/64-bit, SSE, AVX2 (256-bit YMM0-YMM15)
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
│   ├── kernel.c            # Entry point — CPU ya está despierto aquí
│   ├── memory/             # Paginación, heap, E820
│   │   └── memory_init.c   # Bump allocator + E820 map
│   ├── drivers/            # Hardware drivers (C puro, polling)
│   │   ├── keyboard.c      # PS/2 keyboard (i8042 port 0x60)
│   │   ├── mouse_drv.c     # PS/2 mouse (auxiliary port, 3-byte packets)
│   │   ├── timer.c         # PIT 8254 @ 100Hz
│   │   └── fb.c            # Framebuffer VESA VBE + AVX2 256-bit
│   │                         #   fill_rect: 8 pixels/cycle (VPBROADCASTD+VMOVAPS)
│   │                         #   blit: 8 pixels/cycle (VMOVAPS load+store)
│   │                         #   alpha blend, gradient, cursor
│   ├── gui/                # GUI Desktop (v4.0)
│   │   ├── font.c          # Bitmap font 8×16 CP437 (built-in, 4KB)
│   │   ├── wm.c            # Window Manager (PoWindow, z-order, drag)
│   │   │                     #   16 windows max, titlebar, close, focus
│   │   ├── svg.c           # Icon renderer (procedural, 32×32 ARGB)
│   │   │                     #   Built-in: folder, terminal, settings, app, adead
│   │   ├── api.c           # FastOSAPI — function table for Po apps
│   │   │                     #   create_window, draw_rect, draw_text, input
│   │   └── desktop.c       # Desktop compositor
│   │                         #   titlebar, taskbar, icons, shell window
│   │                         #   event loop: keyboard + mouse + compose + flip
│   ├── apps/               # Native Po applications (v4.0)
│   │   └── terminal.c      # Built-in terminal emulator
│   │                         #   help, ver, bg, bg256, mem, clear, exit
│   ├── hotplug.c           # PCI scan → buscar driver .Po → BG verify
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
├── compat/                 # Compatibility Layer (v2.2)
│   ├── fastos_syscall.h    # Syscalls nativas FastOS
│   ├── fastos_stdlib.h     # Stdlib mínima (mem, str, I/O, math AVX2)
│   ├── fastos_win32.h      # Win32 subset → traduce a syscalls FastOS
│   ├── fastos_posix.h      # POSIX subset → traduce a syscalls FastOS
│   └── compat_test.c       # Test suite de traducción
│
├── include/
│   ├── kernel.h            # v3.0: kernel + GUI subsystem declarations
│   ├── types.h             # Fixed-width types, macros
│   ├── fastos.h            # FastOS Native API v3.0 + GUI
│   └── po.h                # Po executable format (detailed)
│
└── userspace/
    ├── shell.c             # Shell básico
    └── init.c              # Init process
```

**Nota:** Los drivers en `kernel/drivers/` son drivers de hardware esencial (teclado, mouse, timer, framebuffer).  
Drivers de dispositivos externos viven en disco como binarios `.Po` verificados por BG.

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

# Verificar con step mode (8 fases)
adb step kernel/main.c
#  [SOURCE] → [PREPROC] → [LEXER] → [PARSER] → [IR] → [UB] → [CODEGEN] → [OUTPUT]
#  0 UB encontrados ✅

# Imagen booteable
powershell -File build64.ps1
```

### Step Mode — Verificación Obligatoria

`adb step` muestra las 8 fases de compilación: source, preprocessor, lexer, parser, IR, UB detector, codegen, output. El kernel DEBE compilar con **0 UB**. Step mode lo verifica antes de cada cambio.

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

## Formato .Po v8.0 — Ejecutable Nativo 256-bit

```
Header: 32 bytes (v8.0)

Offset  Size  Campo
------  ----  ----------------
  0       4   magic: 0x506F4F53 ("PoOS")
  4       1   version: 0x80
  5       1   bits: 16|64|128|256
  6       2   ymm_used: bitmask YMM0-YMM15
  8       4   code_offset
 12       4   code_size
 16       4   data_offset
 20       4   data_size
 24       4   soa_map: offset to SoA descriptor
 28       4   bg_stamp: FNV-1a hash for BG
──────────────────────
Total: 32 bytes.
```

PE de Windows: ~1KB mínimo de header.  
ELF de Linux: 64 bytes + program headers + section headers.  
`.Po` v8.0: **32 bytes. 256-bit nativo. BG integrado.**

---

## Tamaño del OS

| Componente | Tamaño |
|---|---|
| stage1.asm | 512 bytes |
| stage2.asm | ~3 KB |
| kernel core | ~50 KB |
| GUI (fb+wm+font+svg+desktop) | ~40 KB |
| Mouse driver | ~3 KB |
| Binary Guardian | ~20 KB |
| Font data (8×16 CP437) | ~4 KB |
| Icon cache (5 icons) | ~20 KB |
| lib/ mínima | ~10 KB |
| **Total FastOS v3.0** | **~200 KB** |

| OS | Tamaño mínimo |
|---|---|
| Windows 11 | 4 GB |
| Linux (minimal) | 100 MB |
| Android | 2 GB |
| **FastOS v3.0** | **~200 KB** |

---

## Compatibilidad

**FastOS no compite con Windows ni Linux.**  
FastOS demuestra que el diseño original del OS era correcto — y todos se alejaron de él.

```
Binarios .Po v8.0      → FastOS ✓  (formato nativo 32-byte header, 256-bit)
BIOS Legacy boot       → ✓  (MBR stage1 → stage2 → kernel)
x86-64 Long Mode       → ✓  (boot gradual 16→32→64)
AVX2 256-bit           → ✓  (YMM0-YMM15, 8 pixels/cycle rendering)
GUI Desktop            → ✓  (framebuffer 1024×768, window manager, icons)
PS/2 Mouse             → ✓  (3-byte packets, cursor overlay)
BG security gate       → ✓  (todo binario verificado pre-ejecución)
Win32 compat layer     → ✓  (CreateFile, VirtualAlloc, etc. → fastos_syscall)
POSIX compat layer     → ✓  (open, malloc, mmap, etc. → fastos_syscall)
```

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

**FastOS v3.0 — ~200 KB. GUI nativo. 256-bit. Sin muletas. 💀🦈🇵🇪**

```
ADead-BIB v8.0 compila → FastOS arranca → CPU despierta gradual
framebuffer directo → AVX2 256-bit rendering → 8 pixels/cycle
window manager nativo → PoWindow + z-order + drag
mouse PS/2 → cursor overlay → click dispatch
iconos procedurales → folder, terminal, settings, app
.Po v8.0 32 bytes → YMM registers → BG stamp
Win32/POSIX compat → TRADUCIR, no heredar
Todo en 200 KB.
```

*Hecho en Perú 🇵🇪 — Eddi Andreé Salazar Matos*