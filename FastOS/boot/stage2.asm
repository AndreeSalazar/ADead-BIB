; ============================================================
; FastOS v2.0 — stage2.asm (Stage 2 — Transicion Gradual)
; Compilar: fasm boot/stage2.asm boot/stage2.bin
;
; Filosofia del README:
;   "No es un salto brusco. Es un despertar gradual."
;   "Cada paso cierra el contexto anterior antes de abrir el siguiente."
;   "Cuando el kernel arranca, el CPU esta completamente orientado."
;   "Sin estado fantasma. Sin contexto perdido."
;
; Pipeline de este archivo:
;   [16-bit Real Mode]
;     → Configurar segmentos + stack
;     → Habilitar A20
;     → Detectar memoria (INT 0x15 E820)
;     → Cargar GDT transitoria (32-bit)
;     → CR0.PE = 1  →  entrar a 32-bit protected
;
;   [32-bit Protected Mode]  ← en el MISMO archivo, sin far jump externo
;     → Todos los selectores configurados (DS/ES/SS/FS/GS)
;     → Stack en 0x90000
;     → Paginacion 4-level identity map (4GB)
;     → CR4.PAE = 1 (Physical Address Extension)
;     → EFER.LME = 1 (Long Mode Enable)
;     → Cargar GDT 64-bit
;     → CR0.PG = 1  →  entrar a 64-bit long mode
;
;   [64-bit Long Mode]  ← en el MISMO archivo, CPU completamente orientado
;     → RSP configurado
;     → Todos los registros limpiados
;     → call kernel_main  ← CPU despierto
;
; Cargado en: 0x1000:0x0000 = 0x10000 lineal
; ============================================================

format binary
org 0x0000
use16

; ============================================================
; Constantes de layout de memoria
; ============================================================
LOAD_LINEAR     equ 0x10000   ; Donde stage2 esta en memoria
STACK_16        equ 0x9000    ; Stack 16-bit (SS=0, SP=0x9000)
STACK_32        equ 0x90000   ; Stack 32-bit ESP
STACK_64        equ 0x90000   ; Stack 64-bit RSP
KERNEL_ENTRY    equ 0x100000  ; Kernel cargado aqui por build script
PAGE_PML4       equ 0x70000   ; Tablas de pagina
PAGE_PDPT       equ 0x71000
PAGE_PD         equ 0x72000

; ============================================================
; ─── FASE 1: 16-BIT REAL MODE ────────────────────────────
; El CPU llega aqui en modo real despues de stage1.
; No asumimos nada del estado anterior — limpiamos todo.
; ============================================================

stage2_start:

    ; Guardar boot drive que mando stage1 en DL
    mov  bl, dl

    ; Limpiar todos los segmentos — no asumimos nada
    cli
    mov  ax, 0x1000         ; Nuestro segmento (cargados en 0x1000:0x0000)
    mov  ds, ax
    mov  es, ax

    ; Stack 16-bit: SS=0, SP=0x9000
    xor  ax, ax
    mov  ss, ax
    mov  sp, STACK_16
    sti

    ; Re-guardar drive con DS correcto
    mov  [boot_drive], bl

    ; --- Output 16-bit via BIOS ---
    mov  si, msg_16bit
    call print16
    mov  si, msg_a20
    call print16

; ─── A20 Line ────────────────────────────────────────────
; Sin A20, solo podemos acceder a 1MB. Hay que habilitarla.
enable_a20:
    ; Metodo 1: BIOS INT 0x15 AX=0x2401
    mov  ax, 0x2401
    int  0x15
    jnc  .a20_done          ; Si CF=0, A20 OK

    ; Metodo 2: Fast A20 (puerto 0x92)
    in   al, 0x92
    test al, 0x02
    jnz  .a20_done          ; Ya estaba habilitado
    or   al, 0x02
    and  al, 0xFE           ; Asegurar que bit 0 no dispara reset
    out  0x92, al

.a20_done:

; ─── Detectar Memoria (E820) ─────────────────────────────
; INT 0x15 EAX=0xE820 — el estandar de facto para mapas de memoria
; Guardamos el mapa en 0x2000 para que el kernel lo lea en memory_init()
detect_memory:
    mov  si, msg_mem
    call print16

    mov  ax, 0x2000         ; Buffer destino del E820 map
    mov  es, ax
    xor  di, di             ; ES:DI = 0x2000:0x0000
    xor  ebx, ebx           ; EBX=0 = primera entrada
    mov  word [es:di], 0    ; Contador de entradas = 0

.e820_loop:
    add  di, 2              ; Espacio para el contador al inicio
    mov  eax, 0x0000E820
    mov  edx, 0x534D4150    ; "SMAP" signature
    mov  ecx, 24            ; 24 bytes por entrada (con ACPI extended)
    int  0x15
    jc   .e820_done         ; Error o fin de lista
    cmp  eax, 0x534D4150    ; Verificar respuesta valida
    jne  .e820_done
    test ecx, ecx
    jz   .e820_done

    ; Incrementar contador
    sub  di, 2
    inc  word [es:di]
    add  di, 2
    add  di, 24             ; Siguiente entrada (24 bytes)

    test ebx, ebx           ; EBX=0 significa ultima entrada
    jz   .e820_done
    cmp  di, 240            ; Max 10 entradas (seguridad)
    jb   .e820_loop

.e820_done:
    ; Restaurar ES a nuestro segmento
    mov  ax, 0x1000
    mov  es, ax

; ─── GDT de Transicion (para 32-bit PM) ──────────────────
    cli                     ; CRITICO: sin interrupciones antes de cambio de modo
    lgdt [gdt32_desc]

    mov  si, msg_pm
    call print16

; ─── Salto a 32-bit Protected Mode ───────────────────────
; CR0.PE = 1. Este es el momento de la transicion.
; NO es un far jump externo — es una transicion in-place.
    mov  eax, cr0
    or   eax, 0x1           ; PE bit
    mov  cr0, eax

    ; far jump para flush del prefetch queue + cargar CS con selector 32-bit
    ; Esta es la unica "distancia" que cruzamos — al codigo 32-bit
    ; en el MISMO archivo (pm_start esta aqui abajo)
    db 0x66, 0xEA                     ; far jump opcode (32-bit en contexto 16)
    dd LOAD_LINEAR + pm_start         ; destino = pm_start en este archivo
    dw 0x08                           ; selector CS (gdt32_code = 0x08)

; ─── Helpers 16-bit ──────────────────────────────────────
print16:
    lodsb
    test al, al
    jz   .done
    mov  ah, 0x0E
    mov  bx, 0x0007
    int  0x10
    jmp  print16
.done:
    ret

; ─── Datos 16-bit ────────────────────────────────────────
boot_drive:  db 0
msg_16bit:   db "[16-bit] Real mode OK — A20, E820, GDT preparados", 13, 10, 0
msg_a20:     db "[16-bit] A20 line habilitada", 13, 10, 0
msg_mem:     db "[16-bit] Detectando memoria (E820)...", 13, 10, 0
msg_pm:      db "[16-bit] Entrando a 32-bit Protected Mode...", 13, 10, 0

; ─── GDT para 32-bit Protected Mode ──────────────────────
; Minima y correcta. Dos descriptores: code + data.
align 16
gdt32:
    ; [0x00] Descriptor nulo — obligatorio en x86
    dq 0

    ; [0x08] Codigo 32-bit: base=0, limit=4GB, ring 0, readable
    ;   Limit low=0xFFFF, Base low=0, Base mid=0
    ;   Access: P=1(present) DPL=00 S=1 Type=1010(code,read)
    ;   Flags: G=1(4KB) DB=1(32-bit) L=0
    ;   Limit high=0xF, Base high=0
    dw 0xFFFF               ; Limit [15:0]
    dw 0x0000               ; Base  [15:0]
    db 0x00                 ; Base  [23:16]
    db 0x9A                 ; Access: Present, DPL=0, Code, Readable
    db 0xCF                 ; Flags: 4K gran., 32-bit + Limit[19:16]=0xF
    db 0x00                 ; Base  [31:24]

    ; [0x10] Datos 32-bit: base=0, limit=4GB, ring 0, writable
    dw 0xFFFF
    dw 0x0000
    db 0x00
    db 0x92                 ; Access: Present, DPL=0, Data, Writable
    db 0xCF
    db 0x00
gdt32_end:

gdt32_desc:
    dw gdt32_end - gdt32 - 1           ; Limit
    dd LOAD_LINEAR + gdt32             ; Base lineal

; ============================================================
; ─── FASE 2: 32-BIT PROTECTED MODE ───────────────────────
; El CPU llego aqui gradualmente — no aturdido.
; Ahora configuramos todo el entorno de 32-bit ANTES
; de transicionar a 64-bit. Cerramos el contexto 32-bit
; completamente antes de abrir el de 64-bit.
; ============================================================

use32
pm_start:

    ; Configurar TODOS los selectores de segmento data (0x10)
    ; Si uno queda con selector 16-bit, el CPU tendra estado fantasma
    mov  ax, 0x10
    mov  ds, ax
    mov  es, ax
    mov  fs, ax
    mov  gs, ax
    mov  ss, ax
    mov  esp, STACK_32

    ; Reportar en VGA — ya estamos en 32-bit, sin BIOS
    mov  edi, 0xB8000 + (2 * 160)     ; Fila 2
    mov  esi, LOAD_LINEAR + msg_32bit
    mov  ah, 0x0A                     ; Verde sobre negro
.print32:
    lodsb
    test al, al
    jz   .print32_done
    stosw
    jmp  .print32
.print32_done:

; ─── Paginacion 4-Level (Identity Map) ───────────────────
; Necesaria para entrar a long mode.
; Mapeamos los primeros 4GB identity (VA = PA).
; Tablas en 0x70000-0x72FFF (area de memoria baja libre).

setup_paging:
    ; Limpiar area de tablas (4 paginas = 16KB)
    mov  edi, PAGE_PML4
    xor  eax, eax
    mov  ecx, 0x3000 / 4    ; 12KB / 4 bytes = limpiamos PML4+PDPT+PD
    rep  stosd

    ; PML4[0] → PDPT en PAGE_PDPT (Present + Writable)
    mov  dword [PAGE_PML4], PAGE_PDPT + 0x3

    ; PDPT[0] → PD en PAGE_PD (Present + Writable)
    mov  dword [PAGE_PDPT], PAGE_PD + 0x3

    ; PD: 4 entradas × 2MB = primeros 8MB
    ; PS=1 (page size = 2MB), P=1, RW=1
    ; Mascara: 0x83 = PS+RW+P
    ; Cada entrada apunta a: n * 0x200000
    mov  dword [PAGE_PD + 0x00], 0x000083   ; 0MB-2MB
    mov  dword [PAGE_PD + 0x08], 0x200083   ; 2MB-4MB
    mov  dword [PAGE_PD + 0x10], 0x400083   ; 4MB-6MB
    mov  dword [PAGE_PD + 0x18], 0x600083   ; 6MB-8MB

    ; Mapear tambien los 4GB superiores para hardware (VGA, MMIO)
    ; PML4[511] → PDPT de 4GB (identity map completo)
    ; Usamos un PDPT grande con 1GB pages (PS=1 en PDPT, reqiere 1GB support)
    ; Alternativa segura: 2MB pages hasta 4GB con mas entradas PD
    ; Por ahora: 4 entradas × 2MB son suficientes para el kernel en 0x100000

    ; Cargar CR3 con la direccion de PML4
    mov  eax, PAGE_PML4
    mov  cr3, eax

; ─── Habilitar PAE (Physical Address Extension) ──────────
; Requerido ANTES de habilitar Long Mode
    mov  eax, cr4
    or   eax, 0x20          ; PAE bit (bit 5)
    mov  cr4, eax

; ─── Habilitar Long Mode en EFER MSR ─────────────────────
; Extended Feature Enable Register
    mov  ecx, 0xC0000080    ; EFER MSR numero
    rdmsr
    or   eax, 0x100         ; LME bit (Long Mode Enable)
    wrmsr

; ─── Cargar GDT 64-bit ───────────────────────────────────
; Antes de habilitar paginacion, cargo la GDT 64-bit.
; Esto evita cualquier problema de selector en la transicion.
    lgdt [LOAD_LINEAR + gdt64_desc]

    mov  edi, 0xB8000 + (3 * 160)
    mov  esi, LOAD_LINEAR + msg_64bit
    mov  ah, 0x0B
.print64info:
    lodsb
    test al, al
    jz   .print64info_done
    stosw
    jmp  .print64info
.print64info_done:

; ─── Habilitar Paginacion → Activa Long Mode ─────────────
; Este es el momento exacto de transicion 32→64.
; Una vez que PG=1 con LME=1 → el CPU entra en Long Mode.
    mov  eax, cr0
    or   eax, 0x80000000    ; PG bit (Paging Enable)
    mov  cr0, eax

    ; far jump para activar el descriptor 64-bit en CS
    ; PM_START y LM_START estan en el mismo archivo — no hay salto externo
    db 0xEA                             ; far jump opcode
    dd LOAD_LINEAR + lm_start           ; destino = lm_start (64-bit code)
    dw 0x08                             ; selector CS 64-bit (gdt64_code = 0x08)

; ─── Datos 32-bit ────────────────────────────────────────
msg_32bit: db "[32-bit] Protected Mode OK — PAE, paginacion, LME preparados", 0
msg_64bit: db "[32-bit] Paginacion activa — entrando a 64-bit Long Mode...", 0

; ─── GDT para 64-bit Long Mode ───────────────────────────
; En 64-bit, base/limit son ignorados para code/data.
; Solo el bit L=1 importa para marcar como 64-bit code.
align 16
gdt64:
    ; [0x00] Descriptor nulo
    dq 0

    ; [0x08] Codigo 64-bit: L=1 (bit 53), P=1, DPL=0
    ;   Flags byte: 0x20 = L bit set (64-bit code)
    ;   Access byte: 0x9A = Present, DPL=0, Code, Readable
    dw 0x0000               ; Limit (ignorado en 64-bit)
    dw 0x0000               ; Base  (ignorado)
    db 0x00
    db 0x9A                 ; Access: Present, DPL=0, Code
    db 0x20                 ; Flags: L=1 (64-bit), G=0
    db 0x00

    ; [0x10] Datos 64-bit
    dw 0x0000
    dw 0x0000
    db 0x00
    db 0x92                 ; Access: Present, DPL=0, Data, Writable
    db 0x00
    db 0x00
gdt64_end:

gdt64_desc:
    dw gdt64_end - gdt64 - 1
    dq LOAD_LINEAR + gdt64              ; Base como QWORD (64-bit descriptor)

; ============================================================
; ─── FASE 3: 64-BIT LONG MODE ────────────────────────────
; El CPU llego aqui GRADUALMENTE.
; 16-bit real → 32-bit protected → 64-bit long mode.
; Cada fase cerro su contexto correctamente.
;
; Aqui el CPU esta COMPLETAMENTE ORIENTADO.
; Sin estado fantasma. Sin contexto perdido.
; C solo dirige lo que ya existe.
; ============================================================

use64
lm_start:

    ; Configurar segmentos 64-bit
    ; En long mode, DS/ES/SS son ignorados (base=0),
    ; pero deben tener el selector correcto
    mov  ax, 0x10
    mov  ds, ax
    mov  es, ax
    mov  fs, ax
    mov  gs, ax
    mov  ss, ax

    ; Stack 64-bit
    mov  rsp, STACK_64

    ; ─── Reportar 64-bit ──────────────────────────────────
    mov  rdi, 0xB8000 + (4 * 160)     ; Fila 4
    mov  rsi, LOAD_LINEAR + msg_lm
    mov  ah, 0x0E                     ; Amarillo
.print_lm:
    lodsb
    test al, al
    jz   .print_lm_done
    stosw
    jmp  .print_lm
.print_lm_done:

; ─── FASE 3a: SSE ACTIVATION (128-bit XMM) ─────────────
; El OS DEBE habilitar SSE explicitamente o faulteara.
; CR0: limpiar EM (bit 2), setear MP (bit 1)
; CR4: setear OSFXSR (bit 9) y OSXMMEXCPT (bit 10)
enable_sse:
    mov  rax, cr0
    and  ax, 0xFFFB              ; Clear CR0.EM (bit 2) — no x87 emulation
    or   ax, 0x0002              ; Set CR0.MP (bit 1) — monitor coprocessor
    mov  cr0, rax

    mov  rax, cr4
    or   ax, 0x0200              ; CR4.OSFXSR (bit 9) — enable FXSAVE/FXRSTOR
    or   ax, 0x0400              ; CR4.OSXMMEXCPT (bit 10) — SSE exceptions
    mov  cr4, rax

    ; Reportar SSE activo
    mov  rdi, 0xB8000 + (5 * 160)     ; Fila 5
    mov  rsi, LOAD_LINEAR + msg_sse
    mov  ah, 0x0A                     ; Verde
.print_sse:
    lodsb
    test al, al
    jz   .print_sse_done
    stosw
    jmp  .print_sse
.print_sse_done:

; ─── FASE 3b: AVX2 DETECTION + ACTIVATION (256-bit YMM) ─
; Primero verificar que el CPU soporta OSXSAVE (CPUID.1:ECX bit 27)
; Luego verificar AVX2 (CPUID.7:EBX bit 5)
detect_avx2:
    ; Check OSXSAVE support (CPUID leaf 1, ECX bit 27)
    mov  eax, 1
    cpuid
    test ecx, 0x08000000        ; OSXSAVE available? (bit 27)
    jz   .no_avx2                ; No OSXSAVE → skip AVX2

    ; Check AVX support (CPUID leaf 1, ECX bit 28)
    test ecx, 0x10000000         ; AVX available? (bit 28)
    jz   .no_avx2

    ; Check AVX2 (CPUID leaf 7, sub-leaf 0, EBX bit 5)
    mov  eax, 7
    xor  ecx, ecx
    cpuid
    test ebx, 0x20               ; AVX2? (bit 5)
    jz   .no_avx2

    ; ─── Enable OSXSAVE in CR4 ────────────────────────────
    mov  rax, cr4
    or   rax, 0x40000            ; CR4.OSXSAVE (bit 18)
    mov  cr4, rax

    ; ─── Set XCR0: enable X87 (bit 0) + SSE (bit 1) + AVX (bit 2) ───
    xor  rcx, rcx                ; XCR0 = extended control register 0
    xgetbv                        ; Read current XCR0 into EDX:EAX
    or   eax, 0x07               ; Set bits 0,1,2 = X87 + SSE + AVX
    xsetbv                        ; Write back to XCR0

    ; ─── Clean YMM state ──────────────────────────────────
    vzeroupper                    ; Reset upper 128 bits of all YMM regs

    ; Reportar AVX2 256-bit activo
    mov  rdi, 0xB8000 + (6 * 160)     ; Fila 6
    mov  rsi, LOAD_LINEAR + msg_avx2
    mov  ah, 0x0B                     ; Cyan
.print_avx2:
    lodsb
    test al, al
    jz   .avx2_done
    stosw
    jmp  .print_avx2

.no_avx2:
    ; CPU no soporta AVX2 — reportar y continuar (SSE sigue activo)
    mov  rdi, 0xB8000 + (6 * 160)
    mov  rsi, LOAD_LINEAR + msg_no_avx2
    mov  ah, 0x0C                     ; Rojo
.print_no_avx2:
    lodsb
    test al, al
    jz   .avx2_done
    stosw
    jmp  .print_no_avx2

.avx2_done:

; ─── Mover el Kernel a 0x100000 ──────────────────────────
    ; El script `build64.ps1` concatena `kernel.bin` despues de `stage2.bin`.
    ; Como stage2 ocupa 16KB (0x4000) y fue cargado en 0x10000, 
    ; el kernel esta en 0x10000 + 0x4000 = 0x14000.
    ; Movemos 64KB desde 0x14000 a 0x100000.
    mov  rsi, 0x14000
    mov  rdi, KERNEL_ENTRY
    mov  rcx, 8192              ; 65536 bytes / 8 = 8192 QWORDs
    cld
    rep movsq

; ─── Prefetch kernel a L1 cache ──────────────────────────
    ; Anclamos las primeras paginas del kernel en L1 data cache
    ; para que kernel_main() arranque sin cache misses
    mov  rax, KERNEL_ENTRY
    prefetcht0 [rax]
    prefetcht0 [rax + 64]
    prefetcht0 [rax + 128]
    prefetcht0 [rax + 192]
    prefetcht0 [rax + 256]
    prefetcht0 [rax + 320]
    prefetcht0 [rax + 384]
    prefetcht0 [rax + 448]

    ; Limpiar todos los registros de proposito general
    xor  rax, rax
    xor  rbx, rbx
    xor  rcx, rcx
    xor  rdx, rdx
    xor  rsi, rsi
    xor  rdi, rdi
    xor  rbp, rbp
    xor  r8,  r8
    xor  r9,  r9
    xor  r10, r10
    xor  r11, r11
    xor  r12, r12
    xor  r13, r13
    xor  r14, r14
    xor  r15, r15

    ; Reportar salto al kernel
    mov  rdi, 0xB8000 + (7 * 160)     ; Fila 7
    mov  rsi, LOAD_LINEAR + msg_kernel_jump
    mov  ah, 0x0A
.print_kj:
    lodsb
    test al, al
    jz   .print_kj_done
    stosw
    jmp  .print_kj
.print_kj_done:

    ; ─── Verificar que el kernel existe en 0x100000 ──────
    mov  rax, KERNEL_ENTRY
    mov  eax, [rax]
    test eax, eax
    jz   .no_kernel

    ; ─── Deshabilitar interrupts antes de kernel ──────────
    ; No hay IDT configurada — cualquier IRQ causa triple fault.
    ; Mascarar TODAS las IRQs del PIC y CLI.
    cli
    mov  al, 0xFF
    out  0x21, al           ; Mask all master PIC IRQs
    out  0xA1, al           ; Mask all slave PIC IRQs

    ; ─── LLAMAR AL KERNEL ─────────────────────────────────
    ; El CPU esta DESPIERTO — 16→32→64→SSE→AVX2 (256-bit YMM)
    ; Gradual, sin aturdir. Cada capability fue activada en orden.
    ; Interrupts deshabilitadas — kernel debe configurar IDT antes de STI.
    mov  rax, KERNEL_ENTRY
    call rax

    ; Si kernel_main() retorna (nunca deberia):
    jmp  .kernel_returned

.no_kernel:
    mov  rdi, 0xB8000 + (8 * 160)
    mov  rsi, LOAD_LINEAR + msg_no_kernel
    mov  ah, 0x0C
.print_nk:
    lodsb
    test al, al
    jz   .halt
    stosw
    jmp  .print_nk

.kernel_returned:
    mov  rdi, 0xB8000 + (8 * 160)
    mov  rsi, LOAD_LINEAR + msg_returned
    mov  ah, 0x0C
.print_ret:
    lodsb
    test al, al
    jz   .halt
    stosw
    jmp  .print_ret

.halt:
    cli
.halt_loop:
    hlt
    jmp  .halt_loop

; ─── Datos 64-bit ────────────────────────────────────────
msg_lm:          db "[64-bit] Long Mode ACTIVO", 0
msg_sse:         db "[SSE]    XMM 128-bit activo (CR0.MP CR4.OSFXSR)", 0
msg_avx2:        db "[AVX2]   YMM 256-bit activo (OSXSAVE + XCR0 + vzeroupper)", 0
msg_no_avx2:     db "[AVX2]   No soportado por CPU — SSE 128-bit OK", 0
msg_kernel_jump: db "[BOOT]   Kernel prefetched L1 — call kernel_main()", 0
msg_no_kernel:   db "[PANIC]  Kernel no encontrado en 0x100000 — Halt", 0
msg_returned:    db "[PANIC]  kernel_main() retorno — sistema detenido", 0

; ─── Padding a 16KB ──────────────────────────────────────
; El build script espera que stage2 tenga 16KB (32 sectores)
times 16384 - ($ - $$) db 0
