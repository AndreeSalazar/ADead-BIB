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

El Ryzen 5 5600X ya tiene quemado en silicio:
- Teclado, mouse, VGA, modos de memoria
- Toda la historia x86 desde 1978 hasta hoy
- Protocolos de hardware a nivel de microarquitectura

El problema de todos los OS es que **reimplementan en software lo que el CPU ya tiene en hardware.**

FastOS no reimplementa — **conversa con lo que ya existe.**

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
               el CPU detecta hardware nuevo
               FastOS pregunta "¿instalo el driver?"
               se descarga de internet
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

Un driver NVIDIA compilado para Windows **corre en FastOS sin modificaciones** porque ADead-BIB ya habla Win32. El driver cree que está en Windows. FastOS no miente — solo habla el mismo idioma.

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
│   ├── hotplug.c           # Detección hardware en tiempo real
│   │                       # "hardware desconocido → pregunta → descarga → instala"
│   └── panic.c             # Kernel panic handler
│
├── security/               # Binary Guardian
│   ├── bg_core.c           # Verificación matemática pura
│   │                       # Sin heurística — demuestra, no adivina
│   ├── bg_levels.c         # 4 niveles de seguridad
│   └── bg_preexec.c        # Verificación pre-ejecución
│
├── fs/                     # Sistemas de archivos
│   ├── vfs.c               # Virtual File System
│   ├── fat32.c             # FAT32
│   └── ext2.c              # EXT2
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
#include <header_main.h>

// El CPU llegó aquí gradualmente
// Ya sabe dónde está
// C solo dirige lo que ya existe

void kernel_main() {
    memory_init();
    interrupts_init();
    scheduler_init();
    hotplug_init();    // detecta hardware, descarga drivers
    bg_init();         // Binary Guardian activo
    shell_start();
}
```

---

## Compilación

```bash
# Bootloader (FASM directo)
fasm boot/stage1.asm boot/stage1.bin
fasm boot/stage2.asm boot/stage2.bin

# Kernel (ADead-BIB -- sin GCC, sin linker)
adb cc kernel/main.c -o kernel/kernel.bin --flat --org=0x100000

# Imagen de disco
./build/mkimage.sh
```

---

## Sistema de Seguridad — Binary Guardian

```
Nivel 1: Re-build automático
→ corrupción detectada → FastOS se repara solo

Nivel 2: Firewall Humano  
→ comportamiento anómalo → bloqueado antes de ejecutar

Nivel 3: BG Pre-execution
→ cada binario verificado matemáticamente ANTES de correr

Nivel 4: Dead Man's Switch
→ si el sistema es comprometido → se protege solo
```

**Sin heurística. Sin "parece sospechoso".**  
Binary Guardian usa verificación formal — **demuestra** si un binario viola propiedades de seguridad. No adivina.

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
Driver NVIDIA Windows  → corre en FastOS ✓  (Win32 via ADead-BIB)
Driver NVIDIA Linux    → corre en FastOS ✓  (POSIX syscalls)
Binarios .Po nativos   → FastOS ✓
BIOS Legacy            → ✓
UEFI Moderno           → ✓
x86-64                 → ✓
```

---

## Hotplug Inteligente

```
Usuario conecta hardware desconocido
        ↓
kernel/hotplug.c detecta via PCI/USB
        ↓
"Hardware desconocido: NVIDIA RTX 4070"
"¿Instalar driver? [S/N]"
        ↓
Descarga driver específico de internet
        ↓
Instala. Funciona. Listo.
```

Sin disco de drivers. Sin búsqueda manual.  
Sin 24 millones de líneas de drivers pre-instalados.  
**El OS no sabe de impresoras hasta que conectas una.**

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