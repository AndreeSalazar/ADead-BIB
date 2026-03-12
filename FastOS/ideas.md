# FastOS v2.0 — Arquitectura Completa
## ADead-BIB Native OS + Binary Guardian

> **Filosofía:** Un OS es un OS. No una muleta.
> El CPU ya sabe todo — solo hay que dejarlo recordar gradualmente.
> Los drivers van en el disco, no en el OS.
> La seguridad es estructural, no heurística.
>
> — Eddi Andreé Salazar Matos

---

## § 1. Principios Fundamentales

### 1.1 Qué ES FastOS
- **OS puro:** ~150 KB de kernel. Sin bloat, sin abstracciones innecesarias.
- **Boot gradual:** El CPU despierta paso a paso: 16-bit → 32-bit → 64-bit → SSE → AVX2.
- **Drivers externos:** El kernel NO incluye drivers. Se detecta hardware via hotplug y se cargan desde disco bajo demanda.
- **Seguridad determinista:** Binary Guardian (BG) verifica cada binario antes de ejecución. Sin heurísticas. Sin antivirus. Análisis ISA puro.
- **Compilado con ADead-BIB:** Un comando. Un binario. Cero dolor. `adb step` para verificar cada fase.
- **Formato .Po:** Ejecutable nativo de 24 bytes de header. No PE. No ELF. Solo lo que existe.

### 1.2 Qué NO es FastOS
- No es Linux con otro nombre.
- No es un wrapper de UEFI.
- No es un microkernel académico que nunca bootea.
- No pre-instala drivers ni firmware blobs en el kernel.
- No usa heurísticas de seguridad — todo es estructural y matemático.

### 1.3 Metas de Tamaño

| Componente     | Meta          | Justificación                        |
|----------------|---------------|--------------------------------------|
| stage1.bin     | 512 bytes     | MBR exacto, cero bytes desperdiciados |
| stage2.bin     | ~8-16 KB      | Boot gradual completo hasta 64-bit   |
| kernel64.bin   | ~32-64 KB     | Kernel completo con BG integrado     |
| fastos64.img   | ~150 KB total | Cabe en L2 cache de cualquier CPU    |
| Driver típico  | ~4-16 KB      | Binario .Po verificado por BG        |

---

## § 2. Arquitectura de Archivos

```
FastOS/
├── boot/
│   ├── stage1.asm                 ← MBR 512 bytes (FASM/ADead-BIB raw)
│   └── stage2.adB                 ← Boot gradual 16→32→64→SSE→AVX2 (ADead-BIB)
├── kernel/
│   ├── main.c                     ← kernel_main() — punto de entrada
│   ├── memory/
│   │   └── e820.c                 ← Detección de memoria via BIOS E820
│   ├── interrupts.c               ← IDT + handlers de interrupción
│   ├── scheduler.c                ← Scheduler round-robin con prioridad
│   ├── hotplug.c                  ← Detección de hardware + carga de drivers
│   └── panic.c                    ← Kernel panic con diagnóstico
├── security/
│   ├── bg_core.c                  ← Wrapper C → BG Rust FFI
│   ├── bg_fastos.c                ← Integración BG con kernel FastOS
│   ├── bg_levels.c                ← 4 niveles de seguridad BG
│   ├── bg_preexec.c               ← Gate pre-ejecución con cache FNV-1a
│   ├── bg/                        ← Crate Rust: bg-fastos (staticlib)
│   │   ├── lib.rs                 ← Re-exports del crate BG
│   │   ├── analyzer.rs            ← BinaryGuardian::analyze()
│   │   ├── arch_map.rs            ← ArchitectureMap completo
│   │   ├── binary_loader.rs       ← Loader PE/ELF/.Po
│   │   ├── capability.rs          ← CapabilityMapper ISA→capabilities
│   │   ├── policy.rs              ← PolicyEngine::evaluate()
│   │   └── main.rs                ← CLI: bg-check <binary>
│   └── Cargo.toml                 ← Crate bg-fastos → libfg_fastos.a
├── fs/
│   └── vfs.c                      ← Virtual filesystem (futuro)
├── userspace/
│   ├── init.c                     ← Proceso init (PID 1)
│   └── shell.c                    ← Shell interactivo
├── include/
│   ├── kernel.h                   ← Header del kernel completo
│   ├── types.h                    ← Tipos base (u8, u16, u32, u64, size_t)
│   ├── boot_types.h               ← Tipos para boot stage
│   ├── fastos.h                   ← API nativa FastOS + formato .Po
│   └── bg_guardian.h              ← Interface C completa de BG
├── build/
│   ├── fastos64.img               ← Imagen booteable final
│   ├── stage1.bin                 ← 512 bytes
│   ├── stage2.bin                 ← ~8-16 KB
│   └── kernel64.bin               ← ~32-64 KB
└── build64.ps1                    ← Script de build completo
```

---

## § 3. Boot Gradual — El CPU Recuerda

### 3.1 Filosofía del Boot

El CPU moderno arranca en modo 16-bit por compatibilidad histórica. FastOS lo despierta **gradualmente**, activando capacidades en orden:

```
BIOS POST → stage1 (16-bit MBR)
         → stage2 (16→32→64-bit + SSE + AVX2)
         → kernel_main() en 64-bit pleno
```

Cada transición es **explícita y verificable**. No hay magia. El CPU pasa por los mismos registros de control que usa desde el 80386, solo que ahora los controlamos nosotros.

### 3.2 Stage 1 — MBR (512 bytes)

```
Offset  Contenido              Bytes
──────  ─────────────────────  ─────
0x000   Código de carga         446
0x1BE   Tabla de particiones     64
0x1FE   Firma 0x55AA              2
──────────────────────────────────
Total:                          512
```

Stage 1 hace exactamente 3 cosas:
1. Configurar segmentos (DS=ES=SS=0)
2. Cargar stage2 desde sectores 2+ del disco via BIOS INT 13h
3. Saltar a stage2 en 0x7E00

**Compilación:**
```
adb cc boot/stage1.adB --raw --org=0x7C00 -o build/stage1.bin
```

### 3.3 Stage 2 — Despertar Gradual

Stage 2 ejecuta la transición completa del CPU:

```
FASE 1: Real Mode 16-bit
  ├── Detectar memoria via E820
  ├── Cargar GDT (Global Descriptor Table)
  └── Activar bit PE en CR0 → Protected Mode

FASE 2: Protected Mode 32-bit
  ├── Configurar segmentos de datos (0x10)
  ├── Habilitar PAE en CR4 (Physical Address Extension)
  ├── Cargar PML4 en CR3 (Page Map Level 4)
  ├── Activar Long Mode en EFER MSR (bit LME)
  └── Activar paginación en CR0 → Long Mode

FASE 3: Long Mode 64-bit
  ├── Configurar stack en 0x90000
  ├── Activar SSE (CR0.EM=0, CR0.MP=1, CR4.OSFXSR=1)
  ├── Detectar AVX2 via CPUID leaf 7, EBX bit 5
  ├── Si AVX2: activar OSXSAVE en CR4, XCR0 bits [X87|SSE|AVX]
  ├── vzeroupper para limpiar estado YMM
  ├── Prefetch kernel a L1 cache
  └── call kernel_main
```

**Compilación con ADead-BIB:**
```
adb cc boot/stage2.adB --raw --org=0x7E00 -o build/stage2.bin
```

**Verificación con step mode:**
```
adb step boot/stage2.adB

[SOURCE]   boot/stage2.adB: 180 lines, 4832 bytes
[LEXER]    tokens: label, raw, jmp, jz, jnz, db, dw, dd, times...
[PARSER]   labels: 12, jumps: 8, data directives: 6
[CODEGEN]  code: 446 bytes, data: 66 bytes
[CODEGEN]  labels resolved: 12/12 ✅
[CODEGEN]  forward references: 3, all resolved ✅
[OUTPUT]   stage2.bin: 512 bytes (fits in one sector? NO → multi-sector)
```

### 3.4 Registros de Control Involucrados

| Registro | Bits Modificados | Propósito                              |
|----------|------------------|----------------------------------------|
| CR0      | PE (bit 0)       | Activar Protected Mode                 |
| CR0      | PG (bit 31)      | Activar Paginación                     |
| CR4      | PAE (bit 5)      | Physical Address Extension             |
| CR4      | OSFXSR (bit 9)   | SSE support del OS                     |
| CR4      | OSXSAVE (bit 18) | XSAVE/XRSTOR support (para AVX)       |
| EFER     | LME (bit 8)      | Long Mode Enable                       |
| XCR0     | bits 0,1,2       | X87 + SSE + AVX state management       |
| CR3      | PML4 base        | Page table root                        |

---

## § 4. Kernel — kernel_main()

### 4.1 Secuencia de Inicialización

```c
void kernel_main(void) {
    term_init();            // VGA text mode 80×25
    memory_init();          // E820 → physical memory manager
    interrupts_init();      // IDT + PIC remapping + handlers
    scheduler_init();       // Process table + round-robin
    hotplug_init();         // PCI scan → detectar hardware → cargar drivers
    bg_init();              // Binary Guardian activo — gate de pre-ejecución
    shell_start();          // Shell interactivo (PID 2)
}
```

### 4.2 Subsistemas del Kernel

**Memoria (`kernel/memory/e820.c`):**
- Parsear mapa E820 del BIOS
- Physical memory allocator (bitmap o buddy)
- `kmalloc()` / `kfree()` / `kzalloc()`

**Interrupciones (`kernel/interrupts.c`):**
- IDT de 256 entradas
- PIC remapping (IRQ 0-15 → INT 32-47)
- Handlers: timer (IRQ0), keyboard (IRQ1), page fault (#PF)

**Scheduler (`kernel/scheduler.c`):**
- Tabla de procesos: `MAX_PROCESSES = 64`
- Context switch completo: todos los registros generales + RIP + RFLAGS + CR3
- Round-robin con quantum por prioridad
- `process_create()`, `process_yield()`, `process_exit()`

**Hotplug (`kernel/hotplug.c`):**
- Escaneo PCI bus 0-255 al boot
- Para cada dispositivo encontrado: buscar driver .Po en disco
- Cargar driver, verificar con BG, ejecutar si APPROVED
- No hay drivers pre-instalados — todo es on-demand

**Panic (`kernel/panic.c`):**
- `kernel_panic(code, message, file, line)` — nunca retorna
- Dump de registros, stack trace si es posible
- VGA en rojo: error visible inmediatamente

### 4.3 Kernel API Completa

```c
// VGA Terminal
void term_init(void);
void term_putchar(char c);
void term_write(const char *str);
void term_write_color(const char *str, uint8_t color);
void kprintf(const char *fmt, ...);

// Memory
void *kmalloc(size_t size);
void kfree(void *ptr);
void *kzalloc(size_t size);
void *kmemcpy(void *dest, const void *src, size_t n);
void *kmemset(void *s, int c, size_t n);

// Process
int  process_create(const char *name, void (*entry)(void), uint8_t security_level);
void process_exit(int code);
void process_yield(void);
process_t *process_current(void);

// Syscall
int64_t syscall(uint64_t num, uint64_t a1, uint64_t a2,
                uint64_t a3, uint64_t a4, uint64_t a5);

// CPU
void cli(void);  void sti(void);  void hlt(void);
uint64_t rdtsc(void);
void cpuid(uint32_t leaf, uint32_t *eax, uint32_t *ebx,
           uint32_t *ecx, uint32_t *edx);

// Port I/O
void outb(uint16_t port, uint8_t val);
uint8_t inb(uint16_t port);

// BG Security
void bg_init(void);
bg_result_t bg_verify_binary(const uint8_t *binary, size_t size,
                              bg_capability_t caps);
bg_result_t bg_preexec_gate(const uint8_t *binary, size_t size,
                             bg_capability_t caps, uint32_t pid);
```

---

## § 5. Binary Guardian — Seguridad Heredada

### 5.1 Qué es BG

BG (Binary Guardian) es un **guardián determinista a nivel ISA**. No es antivirus. No es sandbox. No usa heurísticas.

```
Binary → Loader → ISA Decoder → ADead-BIB IR → Capability Mapper
      → Architecture Map → Policy Engine → APPROVE / DENY
```

- **Pre-ejecución:** Analiza el binario una vez, genera mapa compacto.
- **Determinista:** Mismo binario + misma policy = mismo veredicto. Siempre.
- **O(n) build, O(1) query.**
- **ISA directo:** No depende de lenguaje, formato, ni alto nivel.
- **Hardware map:** Clasifica exactamente qué hardware toca el binario.

### 5.2 Herencia: BG Rust Crate → FastOS C Kernel

BG está implementado en Rust (`BG — Binary Guardian/`). FastOS lo hereda via FFI:

```
┌─────────────────────────────────────────────┐
│  BG — Binary Guardian (Rust Crate)          │
│  ├── analyzer.rs    → BinaryGuardian        │
│  ├── arch_map.rs    → ArchitectureMap       │
│  ├── capability.rs  → CapabilityMapper      │
│  ├── policy.rs      → PolicyEngine          │
│  └── binary_loader.rs → BinaryLoader        │
└────────────────┬────────────────────────────┘
                 │ FFI (extern "C")
                 ▼
┌─────────────────────────────────────────────┐
│  FastOS security/ (C Wrappers)              │
│  ├── bg/lib.rs      → staticlib (libfg_fastos.a)
│  ├── bg_core.c      → bg_init(), bg_verify_binary()
│  ├── bg_fastos.c    → bg_fastos_init(), bg_fastos_can_execute()
│  ├── bg_levels.c    → 4 niveles de seguridad
│  └── bg_preexec.c   → Gate pre-ejecución + cache FNV-1a
└────────────────┬────────────────────────────┘
                 │ C function calls
                 ▼
┌─────────────────────────────────────────────┐
│  FastOS Kernel (kernel/main.c)              │
│  └── bg_init() llamado en kernel_main()     │
│  └── bg_preexec_gate() en cada exec/load    │
└─────────────────────────────────────────────┘
```

### 5.3 Los 4 Niveles de Seguridad BG

| Nivel | Nombre            | Qué hace                                       |
|-------|-------------------|-------------------------------------------------|
| 1     | Auto Rebuild      | Re-compila binario con ADead-BIB, compara hash  |
| 2     | Human Firewall    | Verifica capabilities vs. permisos del proceso   |
| 3     | Pre-Execution     | Análisis ISA completo antes de mapear en memoria |
| 4     | Dead Man's Switch | Heartbeat periódico + integrity check del kernel |

**Nivel 1 — Auto Rebuild:**
```c
bg_result_t bg_level1_rebuild_check(const char *path, uint64_t expected_hash);
```
Re-compila el binario fuente con ADead-BIB y compara el hash del resultado con el binario en disco. Si difieren → DENIED (binario fue modificado post-compilación).

**Nivel 2 — Human Firewall:**
```c
bg_result_t bg_level2_capability_check(uint32_t pid, bg_capability_t req,
                                        bg_capability_t allowed);
```
Verifica que las capabilities requeridas por el binario no excedan las otorgadas al proceso.

**Nivel 3 — Pre-Execution Gate:**
```c
bg_result_t bg_level3_preexec(const uint8_t *binary, size_t size,
                               bg_capability_t caps);
```
Análisis ISA completo: decodifica cada instrucción, construye ArchitectureMap, evalúa contra SecurityPolicy. El gate más importante.

**Nivel 4 — Dead Man's Switch:**
```c
void bg_level4_heartbeat(void);
bg_result_t bg_level4_integrity_check(void);
```
El kernel envía heartbeats periódicos. Si falla → kernel panic controlado. Integrity check verifica que el código del kernel no fue modificado en runtime.

### 5.4 SecurityPolicy — Policies por Nivel

Heredadas directamente del Rust crate `policy.rs`:

```
SecurityLevel::Kernel  → Ring 0: todo permitido
SecurityLevel::Driver  → Ring 1: IO + interrupts, sin MSR/descriptor tables
SecurityLevel::Service → Ring 2: sin IO directo, solo syscalls autorizados
SecurityLevel::User    → Ring 3: máximas restricciones, sin privilegios
```

**Policy de Driver (ejemplo real del crate):**
```rust
SecurityPolicy {
    name: "Driver",
    level: SecurityLevel::Driver,
    allowed_io_ports: None,           // Drivers pueden acceder IO
    max_indirect_sites: Some(50),
    allow_rwx: false,
    allow_self_modifying: false,
    allow_far_jumps: true,
    require_structural_integrity: true,
    allow_process_injection: false,
    allow_timing_instructions: true,
    allow_hidden_entry_points: false,
    allowed_hardware_devices: None,   // Drivers acceden hardware
}
```

**Policy de User (máxima restricción):**
```rust
SecurityPolicy {
    name: "User",
    level: SecurityLevel::User,
    allowed_syscall_vectors: Some(vec![0x80]),
    allowed_io_ports: Some(Vec::new()),        // Sin IO
    max_indirect_sites: Some(10),
    allow_rwx: false,
    allow_self_modifying: false,
    allow_far_jumps: false,
    require_structural_integrity: true,
    allow_process_injection: false,
    allow_timing_instructions: false,
    allow_hidden_entry_points: false,
    allowed_hardware_devices: Some(Vec::new()), // Sin hardware
}
```

### 5.5 ArchitectureMap — Lo que BG Sabe del Binario

El ArchitectureMap es el producto del análisis ISA. Contiene TODO lo que el binario puede hacer:

```
ArchitectureMap {
    instruction_map    → total, safe, privileged, io, syscall, float, simd counts
    memory_map         → regiones RWX, self-modifying code, stack NX
    control_flow_map   → calls directos/indirectos, jumps, far jumps
    io_map             → puertos accedidos (IN/OUT) con dirección y tamaño
    syscall_map        → vectores INT usados, SYSCALL/SYSENTER
    capabilities       → flags: privileged, io_port, interrupt, msr, etc.
    integrity          → entry point válido, secciones superpuestas, TLS callbacks
    import_export_map  → DLLs importadas, APIs de inyección de proceso
    hardware_map       → dispositivos accedidos, timing (RDTSC), debug regs
}
```

### 5.6 Violation Types

BG detecta estas violaciones (heredadas de `policy.rs`):

| Violación                    | Severidad | Descripción                                |
|------------------------------|-----------|--------------------------------------------|
| PrivilegedInstruction        | CRITICAL  | Instrucción Ring 0 en nivel inferior       |
| UnauthorizedSyscall          | HIGH      | Vector INT no autorizado                   |
| UnauthorizedIO               | CRITICAL  | Acceso a puerto IO no permitido            |
| ExcessiveIndirectControl     | MEDIUM    | Demasiados call/jmp indirectos             |
| FarJumpNotAllowed            | MEDIUM    | Far jump en nivel que no lo permite        |
| SelfModifyingCode            | HIGH      | Código que se auto-modifica                |
| RWXMemory                    | HIGH      | Sección con Read+Write+Execute             |
| InvalidEntryPoint            | CRITICAL  | Entry point fuera de sección de código     |
| OverlappingSections          | CRITICAL  | Secciones del binario se superponen        |
| AnomalousPermissions         | HIGH      | Data+Execute en misma sección              |
| ProcessInjectionImports      | CRITICAL  | APIs como WriteProcessMemory importadas    |
| TimingAttackCapability       | MEDIUM    | RDTSC/RDTSCP sin justificación             |
| HiddenEntryPoint             | HIGH      | TLS callbacks (código pre-entry point)     |
| UnauthorizedHardwareAccess   | CRITICAL  | Acceso a dispositivo no autorizado         |

### 5.7 Pre-Execution Gate con Cache

`bg_preexec.c` implementa un cache FNV-1a para evitar re-analizar binarios idénticos:

```
Binario llega → FNV-1a hash del contenido
             → ¿Hash en cache?
                SÍ → retornar resultado cacheado (O(1))
                NO → análisis ISA completo → guardar en cache → retornar
```

El cache se invalida cuando:
- El binario cambia (hash diferente)
- Se llama `bg_preexec_invalidate(hash)` explícitamente
- El sistema reinicia

---

## § 6. Formato .Po — Ejecutable Nativo

### 6.1 Header .Po

```
Offset  Size  Campo
──────  ────  ──────────────
  0       6   magic: "FASTOS"
  6       2   version: 2
  8       4   code_offset
 12       4   code_size
 16       4   data_offset
 20       4   data_size
──────────────────────────────
Total:   24 bytes
```

Comparación:
- PE (Windows): ~1 KB mínimo de headers
- ELF (Linux): 64 bytes + program headers + section headers
- **.Po (FastOS): 24 bytes. Solo lo que existe.**

### 6.2 Flujo de Ejecución de un .Po

```
1. Shell recibe comando: "run miapp.po"
2. Kernel lee archivo .Po desde disco
3. Verifica magic "FASTOS" + version
4. BG Pre-Execution Gate:
   a. bg_preexec_gate(binary, size, caps, pid)
   b. Si cache hit → resultado inmediato
   c. Si cache miss → análisis ISA completo:
      - Decodificar instrucciones
      - Construir ArchitectureMap
      - Evaluar contra SecurityPolicy del nivel del proceso
   d. Resultado: APPROVE / DENY
5. Si APPROVED:
   a. Mapear código en memoria virtual del proceso
   b. Configurar stack + data segment
   c. Establecer capabilities aprobadas (IOPL, etc.)
   d. Saltar a entry point
6. Si DENIED:
   a. Log de violaciones
   b. Retornar error al shell
   c. No se ejecuta nada
```

### 6.3 Compilar un .Po con ADead-BIB

```bash
# Compilación directa
adb cc miapp.c --target fastos -o miapp.po

# Verificar con step mode (7 fases)
adb step miapp.c

# Multi-target simultáneo
adb cc miapp.c --target all -o miapp
# → miapp.po (FastOS), miapp.exe (PE), miapp (ELF)
```

---

## § 7. ADead-BIB — Compilador del Sistema

### 7.1 Rol de ADead-BIB en FastOS

ADead-BIB es el compilador que construye TODO en FastOS:
- **Boot stages:** `adb cc stage1.adB --raw --org=0x7C00`
- **Kernel:** `adb cc kernel/main.c --flat --org=0x100000`
- **Drivers .Po:** `adb cc driver.c --target fastos -o driver.po`
- **Apps .Po:** `adb cc app.c --target fastos -o app.po`

No hay GCC. No hay LD. No hay Make. Un comando, un binario.

### 7.2 Step Mode — Las 7 Fases

`adb step <archivo>` muestra cada fase de compilación en detalle:

```
$ adb step kernel/main.c

═══════════════════════════════════════════════════
  PHASE 1: SOURCE
═══════════════════════════════════════════════════
[SOURCE]   kernel/main.c: 194 lines, 5284 bytes

═══════════════════════════════════════════════════
  PHASE 2: PREPROCESSOR
═══════════════════════════════════════════════════
[PREPROC]  #include "kernel.h" → 496 lines expanded
[PREPROC]  #include "fastos.h" → 210 lines expanded
[PREPROC]  #include "bg_guardian.h" → 233 lines expanded
[PREPROC]  total after expansion: 1133 lines

═══════════════════════════════════════════════════
  PHASE 3: LEXER
═══════════════════════════════════════════════════
[LEXER]    tokens generated: 4521
[LEXER]    first 20: void(1:1) kernel_main(1:6) ((1:17) )(1:18) {(1:20) ...

═══════════════════════════════════════════════════
  PHASE 4: PARSER
═══════════════════════════════════════════════════
[PARSER]   functions: 12
[PARSER]   structs: 8
[PARSER]   typedefs: 15
[PARSER]   enums: 6
[PARSER]   globals: 3

═══════════════════════════════════════════════════
  PHASE 5: UB DETECTOR
═══════════════════════════════════════════════════
[UB]       functions analyzed: 12
[UB]       variables declared: 47
[UB]       pointer operations: 23
[UB]       null checks found:  18
[UB]       result: 0 UB encontrados ✅

═══════════════════════════════════════════════════
  PHASE 6: CODEGEN
═══════════════════════════════════════════════════
[CODEGEN]  machine code: 2048 bytes
[CODEGEN]  data section: 384 bytes
[CODEGEN]  functions compiled: 12
[CODEGEN]  dead code eliminated: 3 functions (unreachable from main)

═══════════════════════════════════════════════════
  PHASE 7: OUTPUT
═══════════════════════════════════════════════════
[OUTPUT]   format: flat binary
[OUTPUT]   org: 0x100000
[OUTPUT]   total: 2432 bytes
[OUTPUT]   written: build/kernel64.bin ✅
```

### 7.3 UB Detector — Seguridad en Compilación

ADead-BIB detecta Undefined Behavior en tiempo de compilación:
- Null pointer dereferences
- Buffer overflows estáticos
- Use-after-free patterns
- Double free
- Uninitialized variable reads
- Integer overflow en operaciones constantes

El kernel de FastOS DEBE compilar con 0 UB. Step mode lo verifica.

### 7.4 Dead Code Elimination

ADead-BIB implementa DCE (Dead Code Elimination) en Fase 6:
- Camina el AST desde `main()` construyendo el grafo de llamadas transitivo
- Solo compila funciones alcanzables desde el entry point
- Headers con funciones inline no inflan el binario

Esto es crítico para FastOS: los headers (`kernel.h`, `bg_guardian.h`) definen muchas funciones inline, pero solo las usadas terminan en el binario.

### 7.5 Sintaxis ADead-BIB para Boot (labels + raw)

Para los boot stages, ADead-BIB soporta sintaxis de bajo nivel:

```
label start16:
    raw { 0xFA }              // cli
    raw { 0x31, 0xC0 }       // xor eax, eax

label gdt_load:
    raw { 0x0F, 0x01, 0x16 } // lgdt [addr]

    jz protected_mode         // jump if zero flag
    jmp start16               // unconditional jump

label protected_mode:
    // ... modo 32-bit

db "FASTOS", 0x00             // define bytes
dw 0xAA55                     // define word (boot signature)
dd 0x100000                   // define dword (kernel load address)
times 510 db 0                // pad to 510 bytes
dw 0xAA55                     // MBR signature at offset 510
```

---

## § 8. Hotplug — Drivers Externos

### 8.1 Filosofía

**Los drivers NO viven en el kernel.** Viven en disco como binarios .Po.

Al boot:
1. `hotplug_init()` escanea el bus PCI
2. Para cada dispositivo, busca `drivers/<vendor_id>_<device_id>.po`
3. Carga el .Po, lo pasa por BG Pre-Execution Gate
4. Si APPROVED → lo ejecuta como proceso con `BG_LEVEL_DRIVER` policy
5. Si DENIED → log de violaciones, dispositivo queda sin driver

### 8.2 Driver Manifest (Futuro)

Cada driver declarará qué hardware necesita:

```c
// En el header del driver .Po
static const driver_manifest_t MANIFEST = {
    .name = "keyboard_ps2",
    .vendor_id = 0x0000,
    .device_id = 0x0000,
    .required_ports = { 0x60, 0x64 },
    .required_irq = 1,
    .required_caps = BG_CAP_IO_DIRECT,
};
```

BG verifica que el binario del driver NO excede lo declarado en el manifest. Si el driver dice "solo necesito puertos 0x60 y 0x64" pero su código accede al puerto 0x3F8 (COM1), BG lo detecta y lo DENIEGA.

### 8.3 PCI Device Detection

```c
void hotplug_on_pci_device(uint16_t vendor, uint16_t device,
                            uint8_t bus, uint8_t slot, uint8_t func) {
    kprintf("[HOTPLUG] PCI %02X:%02X.%X vendor=%04X device=%04X\n",
            bus, slot, func, vendor, device);

    // Buscar driver en disco
    char path[64];
    kprintf("[HOTPLUG] Buscando drivers/%04X_%04X.po\n", vendor, device);

    // Cargar y verificar con BG
    // bg_preexec_gate(driver_binary, size, BG_CAP_DRIVER, pid);
}
```

---

## § 9. Syscalls FastOS

### 9.1 Números de Syscall

```c
// POSIX-like (compatibilidad)
#define SYS_EXIT         0x03C
#define SYS_READ         0x000
#define SYS_WRITE        0x001
#define SYS_OPEN         0x002
#define SYS_CLOSE        0x003
#define SYS_FORK         0x039
#define SYS_EXEC         0x03B
#define SYS_GETPID       0x027

// Extensiones FastOS
#define SYS_HOTPLUG_QUERY  200  // consultar hardware detectado
#define SYS_BG_VERIFY      201  // verificar binario con Binary Guardian
#define SYS_PO_EXEC        202  // ejecutar binario .Po
#define SYS_DRIVER_LOAD    203  // cargar driver desde disco

// Graphics (futuro)
#define SYS_FB_INIT      0xF00
#define SYS_FB_PIXEL     0xF01
#define SYS_FB_RECT      0xF02
#define SYS_GPU_DETECT   0xF10
```

### 9.2 Syscall BG_VERIFY

```c
// Desde userspace:
int result = syscall(SYS_BG_VERIFY, (uint64_t)binary_ptr, binary_size,
                     BG_CAP_SYSCALL, 0, 0);
// result: BG_RESULT_OK (0) o código de error
```

Este syscall permite que procesos de usuario verifiquen binarios antes de pasarlos a `SYS_PO_EXEC`. Transparencia total.

---

## § 10. Proceso de Build Completo

### 10.1 Compilación del Kernel

```powershell
# 1. Boot stage 1 (MBR)
adb cc boot/stage1.adB --raw --org=0x7C00 -o build/stage1.bin

# 2. Boot stage 2 (gradual CPU wake)
adb cc boot/stage2.adB --raw --org=0x7E00 -o build/stage2.bin

# 3. BG Rust crate → staticlib
cargo build --release --manifest-path security/Cargo.toml
# → target/release/libfg_fastos.a

# 4. Kernel C + BG staticlib
adb cc kernel/main.c \
       kernel/memory/e820.c \
       kernel/interrupts.c \
       kernel/scheduler.c \
       kernel/hotplug.c \
       kernel/panic.c \
       security/bg_core.c \
       security/bg_fastos.c \
       security/bg_levels.c \
       security/bg_preexec.c \
       --flat --org=0x100000 \
       --link target/release/libfg_fastos.a \
       -o build/kernel64.bin

# 5. Verificar con step mode
adb step kernel/main.c
# → 7 fases, 0 UB, todas las funciones verificadas

# 6. Crear imagen booteable
# (ver build64.ps1)
```

### 10.2 Imagen Booteable

```
Offset      Contenido         Tamaño
──────────  ────────────────  ──────
0x00000     stage1.bin        512 bytes (MBR)
0x00200     stage2.bin        ~16 KB
0x08000     kernel64.bin      ~32-64 KB
0x20000     drivers/          Espacio para drivers .Po
0x80000     filesystem        Espacio para VFS
──────────────────────────────────────
Total:      ~150 KB usados de imagen de 10 MB
```

### 10.3 Ejecutar en QEMU

```bash
qemu-system-x86_64 -drive format=raw,file=build/fastos64.img -m 64M
```

### 10.4 Output Esperado

```
[FASTOS] Iniciando FastOS v2.0...
[MEM]    E820: 64MB RAM detectada
[BOOT]   16-bit → 32-bit → 64-bit ✓
[SSE]    XMM 128-bit activo ✓
[AVX2]   YMM 256-bit activo ✓ (CPUID leaf 7, EBX.5)
[CACHE]  Kernel anclado en L1
[IDT]    256 vectores configurados
[SCHED]  Scheduler activo, 64 slots
[PCI]    Escaneando bus...
[PCI]    03:00.0 vendor=10DE device=2504 → NVIDIA GPU
[HOTPLUG] Buscando drivers/10DE_2504.po
[BG]     Binary Guardian v2.0 activo
[BG]     Niveles: [1] Rebuild [2] Firewall [3] PreExec [4] DeadMan
[BG]     Pre-execution gate: ENABLED
[SHELL]  FastOS shell listo
[FASTOS] ~150KB. Puro. Seguro. Desde Lima.
$
```

---

## § 11. Rust Safety Layer (FFI)

### 11.1 Por Qué Rust en un Kernel C

El kernel de FastOS está en C por control directo del hardware. Pero BG está en Rust por:

- **Memory safety:** BG analiza binarios potencialmente maliciosos. Un buffer overflow en BG sería catastrófico.
- **No runtime:** `panic = "abort"`, sin unwinding, sin allocator estándar.
- **Tamaño:** `opt-level = "z"` + LTO + strip → staticlib mínima.
- **Determinismo:** Los tipos de Rust (enums exhaustivos, pattern matching) garantizan que el análisis es total y completo.

### 11.2 FFI Boundary

```c
// Desde C (kernel):
extern void bg_rust_init(void);
extern int bg_rust_analyze(const uint8_t *binary, uint32_t size,
                            BgAnalysisResult *result);
extern int bg_rust_check_policy(const BgAnalysisResult *result,
                                 BgSecurityLevel level);
```

```rust
// Desde Rust (bg-fastos crate):
#[no_mangle]
pub extern "C" fn bg_rust_init() { ... }

#[no_mangle]
pub extern "C" fn bg_rust_analyze(binary: *const u8, size: u32,
                                   result: *mut BgAnalysisResult) -> i32 { ... }
```

### 11.3 Safe Memory Operations

Además de BG, Rust provee operaciones de memoria seguras:

```c
void *rust_malloc(size_t size);
void rust_free(void *ptr);
int rust_memcpy_safe(void *dest, size_t dest_size, const void *src, size_t count);
int rust_memset_safe(void *dest, size_t dest_size, int value, size_t count);
```

Estas funciones verifican bounds antes de operar. Si `count > dest_size` → retorna error en vez de overflow.

---

## § 12. Hardware Detection — CPUID

### 12.1 Detección Genérica (No Hardcoded)

FastOS detecta capabilities del CPU via CPUID en runtime, no hardcoded a un modelo:

```c
typedef struct {
    uint8_t  has_sse;       // CPUID.1:EDX.25
    uint8_t  has_sse2;      // CPUID.1:EDX.26
    uint8_t  has_sse3;      // CPUID.1:ECX.0
    uint8_t  has_sse4_1;    // CPUID.1:ECX.19
    uint8_t  has_sse4_2;    // CPUID.1:ECX.20
    uint8_t  has_avx;       // CPUID.1:ECX.28
    uint8_t  has_avx2;      // CPUID.7:EBX.5
    uint8_t  has_aesni;     // CPUID.1:ECX.25
    uint8_t  has_rdtsc;     // CPUID.1:EDX.4
    uint32_t cache_line;    // Tamaño de cache line (64 bytes típico)
} cpu_features_t;
```

Stage 2 usa esta información para decidir qué activar:
- Si `has_avx2` → activar XCR0 bits, vzeroupper
- Si solo `has_sse` → no tocar XCR0, solo CR4.OSFXSR
- Si nada → modo escalar puro

### 12.2 AVX2 Activation (Genérica)

```
1. CPUID leaf 7, ECX=0 → EBX bit 5 = AVX2
2. Si AVX2 presente:
   a. CR4.OSXSAVE = 1 (bit 18)
   b. XGETBV → leer XCR0
   c. XCR0 |= 0b111 (X87 + SSE + AVX)
   d. XSETBV
   e. vzeroupper (limpiar estado YMM)
3. Verificar: CPUID leaf 7 de nuevo para confirmar
```

---

## § 13. Roadmap de Implementación

### Fase 1: Boot Funcional (Actual)
- [x] Stage 1 MBR
- [x] Stage 2 gradual (16→32→64)
- [x] VGA text mode
- [x] kernel_main() ejecuta
- [ ] E820 memory detection completa
- [ ] SSE/AVX2 activation genérica

### Fase 2: Kernel Core
- [x] Kernel headers (kernel.h, fastos.h, bg_guardian.h)
- [x] VGA terminal (term_write, kprintf)
- [ ] Physical memory manager (bitmap)
- [ ] IDT completo con handlers
- [ ] PIC remapping
- [ ] Timer (PIT) + sleep

### Fase 3: BG Integration
- [x] BG Rust crate compilable
- [x] C wrappers (bg_core.c, bg_fastos.c, bg_levels.c, bg_preexec.c)
- [x] bg_guardian.h con tipos completos
- [ ] FFI linkeado (libfg_fastos.a → kernel)
- [ ] Pre-execution gate funcional en kernel
- [ ] Cache FNV-1a verificado

### Fase 4: Process Management
- [x] PCB structure definida
- [ ] Scheduler round-robin funcional
- [ ] Context switch (registros + CR3)
- [ ] process_create() / process_exit()
- [ ] Syscall interface (INT 0x80 o SYSCALL)

### Fase 5: Hotplug + Drivers
- [x] Hotplug design documentado
- [ ] PCI bus scan
- [ ] .Po loader funcional
- [ ] Driver loading + BG verification
- [ ] Driver manifest verification

### Fase 6: Userspace
- [ ] Shell básico
- [ ] Ejecutar .Po desde shell
- [ ] Filesystem básico (VFS)
- [ ] Init process (PID 1)

---

## § 14. Reglas de Oro

1. **El kernel NO tiene drivers.** Si no hay driver en disco, el hardware no funciona. Punto.
2. **Todo binario pasa por BG.** Sin excepciones. Kernel incluido (Level 4 integrity).
3. **ADead-BIB compila todo.** Un comando. Sin linker externo. Sin flags misteriosos.
4. **Step mode para verificar.** `adb step` antes de cada cambio al kernel. 0 UB obligatorio.
5. **24 bytes de header .Po.** No PE. No ELF. Solo lo que existe.
6. **~150 KB total.** Si el kernel crece más, algo está mal.
7. **Determinismo absoluto.** Mismo binario + misma policy = mismo veredicto. Siempre.
8. **Desde Lima.** 🇵🇪

---

> **FastOS v2.0** — Un OS es un OS. No una muleta.
> Compilado con ADead-BIB. Guardado por Binary Guardian. Puro.
>
> — Eddi Andreé Salazar Matos
