; ============================================================================
; FastOS Bootloader - Stage 1
; ============================================================================
; Bootloader mínimo que carga el kernel
; MBR (512 bytes) -> Modo protegido -> Modo largo -> Kernel
;
; Author: Eddi Andreé Salazar Matos
; ============================================================================

[BITS 16]
[ORG 0x7C00]

start:
    ; Configurar segmentos
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00

    ; Guardar drive number
    mov [boot_drive], dl

    ; Limpiar pantalla
    mov ax, 0x0003
    int 0x10

    ; Mostrar mensaje de inicio
    mov si, msg_boot
    call print_string

    ; Habilitar A20
    call enable_a20

    ; Cargar kernel desde disco
    mov si, msg_loading
    call print_string
    call load_kernel

    ; Entrar a modo protegido
    mov si, msg_protected
    call print_string
    call enter_protected_mode

; ============================================================================
; Funciones de 16 bits
; ============================================================================

print_string:
    lodsb
    or al, al
    jz .done
    mov ah, 0x0E
    int 0x10
    jmp print_string
.done:
    ret

enable_a20:
    in al, 0x92
    or al, 2
    out 0x92, al
    ret

load_kernel:
    ; Cargar kernel en 0x100000 (1MB)
    ; Por ahora, simulamos que el kernel está en los sectores 2-64
    mov ah, 0x02        ; Función: leer sectores
    mov al, 63          ; Número de sectores
    mov ch, 0           ; Cilindro 0
    mov cl, 2           ; Sector 2 (1-indexed)
    mov dh, 0           ; Cabeza 0
    mov dl, [boot_drive]
    mov bx, 0x1000      ; Cargar temporalmente en 0x1000:0x0000
    mov es, bx
    xor bx, bx
    int 0x13
    jc disk_error
    ret

disk_error:
    mov si, msg_disk_error
    call print_string
    jmp $

enter_protected_mode:
    cli
    lgdt [gdt_descriptor]
    
    mov eax, cr0
    or eax, 1
    mov cr0, eax
    
    jmp 0x08:protected_mode_start

; ============================================================================
; GDT (Global Descriptor Table)
; ============================================================================

gdt_start:
    ; Null descriptor
    dq 0

gdt_code:
    ; Code segment: base=0, limit=4GB, executable, readable
    dw 0xFFFF       ; Limit (bits 0-15)
    dw 0x0000       ; Base (bits 0-15)
    db 0x00         ; Base (bits 16-23)
    db 10011010b    ; Access byte
    db 11001111b    ; Flags + Limit (bits 16-19)
    db 0x00         ; Base (bits 24-31)

gdt_data:
    ; Data segment: base=0, limit=4GB, writable
    dw 0xFFFF
    dw 0x0000
    db 0x00
    db 10010010b
    db 11001111b
    db 0x00

gdt_code64:
    ; 64-bit code segment
    dw 0xFFFF
    dw 0x0000
    db 0x00
    db 10011010b
    db 10101111b    ; Long mode flag
    db 0x00

gdt_data64:
    ; 64-bit data segment
    dw 0xFFFF
    dw 0x0000
    db 0x00
    db 10010010b
    db 10101111b
    db 0x00

gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

; ============================================================================
; Modo Protegido (32 bits)
; ============================================================================

[BITS 32]
protected_mode_start:
    ; Configurar segmentos de datos
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    mov esp, 0x90000

    ; Configurar paging para modo largo
    call setup_paging

    ; Entrar a modo largo
    mov ecx, 0xC0000080     ; EFER MSR
    rdmsr
    or eax, 0x100           ; Set LME (Long Mode Enable)
    wrmsr

    ; Habilitar paging
    mov eax, cr0
    or eax, 0x80000000
    mov cr0, eax

    ; Saltar a modo largo
    jmp 0x18:long_mode_start

setup_paging:
    ; PML4 en 0x1000
    mov edi, 0x1000
    mov cr3, edi
    xor eax, eax
    mov ecx, 4096
    rep stosd
    mov edi, cr3

    ; PML4[0] -> PDPT
    mov dword [edi], 0x2003
    add edi, 0x1000

    ; PDPT[0] -> PD
    mov dword [edi], 0x3003
    add edi, 0x1000

    ; PD[0] -> 2MB page (identity map first 2MB)
    mov dword [edi], 0x83
    
    ; Habilitar PAE
    mov eax, cr4
    or eax, 0x20
    mov cr4, eax

    ret

; ============================================================================
; Modo Largo (64 bits)
; ============================================================================

[BITS 64]
long_mode_start:
    ; Configurar segmentos
    mov ax, 0x20
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Stack en 2MB
    mov rsp, 0x200000

    ; Saltar al kernel en 0x100000
    ; Por ahora, mostrar mensaje y halt
    mov rdi, 0xB8000
    mov rax, 0x0F460F610F730F74  ; "Fast" en verde
    mov [rdi], rax
    mov rax, 0x0F530F4F0F20      ; "OS " en verde
    mov [rdi+8], rax

    ; Halt
    hlt
    jmp $

; ============================================================================
; Datos
; ============================================================================

boot_drive: db 0
msg_boot: db "FastOS Bootloader v0.1", 13, 10, 0
msg_loading: db "Loading kernel...", 13, 10, 0
msg_protected: db "Entering protected mode...", 13, 10, 0
msg_disk_error: db "Disk error!", 13, 10, 0

; Padding y firma de boot
times 510-($-$$) db 0
dw 0xAA55
