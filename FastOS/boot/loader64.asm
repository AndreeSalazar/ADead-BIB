; ============================================================
; FastOS v2.0 — Stage 2 Loader (64-bit transition)
; Loaded at 0x1000:0x0000 = 0x10000 linear
; Transitions to Long Mode and displays message
; ============================================================

format binary as 'bin'
org 0

use16

STACK_ADDR      equ 0x90000
LOAD_SEG        equ 0x1000

stage2_start:
    ; DL = boot drive number from MBR — save in BL temporarily
    mov bl, dl
    
    cli
    
    ; Setup DS to our segment (0x1000)
    mov ax, LOAD_SEG
    mov ds, ax
    mov es, ax
    
    ; Setup stack
    xor ax, ax
    mov ss, ax
    mov sp, 0x9000
    sti
    
    ; Now save boot drive with correct DS
    mov [boot_drive], bl
    
    ; Print stage2 message using BIOS
    mov si, msg_s2
    call print16
    
    ; Enable A20
    call enable_a20
    mov si, msg_a20
    call print16
    
    ; Print entering PM message
    mov si, msg_pm
    call print16
    
    ; Kernel already loaded by MBR to 0x20000
    mov si, msg_kernel_ok
    call print16
    ; Restore DS
    mov ax, LOAD_SEG
    mov ds, ax
    
    ; Disable interrupts for mode switch
    cli
    
    ; Load GDT (need linear address)
    lgdt [gdt_desc]
    
    ; Enter protected mode
    mov eax, cr0
    or eax, 1
    mov cr0, eax
    
    ; Far jump to 32-bit code using indirect jump
    db 0x66, 0xEA              ; 32-bit far jump opcode
    dd 0x10000 + pm_start      ; 32-bit offset
    dw 0x08                    ; segment selector

; ============================================================
; Enable A20 Line
; ============================================================
enable_a20:
    ; Try BIOS method
    mov ax, 0x2401
    int 0x15
    jnc .done
    
    ; Fast A20 method
    in al, 0x92
    or al, 2
    out 0x92, al
    
.done:
    ret

; ============================================================
; Print String (16-bit BIOS)
; ============================================================
print16:
    lodsb
    test al, al
    jz .done
    mov ah, 0x0E
    mov bx, 0x0007
    int 0x10
    jmp print16
.done:
    ret

; ============================================================
; 32-bit Protected Mode
; ============================================================
use32

pm_start:
    ; Setup 32-bit segments
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    mov esp, STACK_ADDR
    
    ; Print directly to VGA
    mov edi, 0xB8000 + 480  ; Row 3
    mov esi, 0x10000 + msg_32
    mov ah, 0x0E            ; Yellow
.print32:
    lodsb
    test al, al
    jz .print32_done
    stosw
    jmp .print32
.print32_done:
    
    ; Kernel copy moved to 64-bit mode (A20/paging guaranteed)
    
    ; Setup paging
    call setup_paging
    
    ; Enable PAE in CR4
    mov eax, cr4
    or eax, 0x20            ; PAE bit
    mov cr4, eax
    
    ; Enable Long Mode in EFER MSR
    mov ecx, 0xC0000080     ; EFER MSR
    rdmsr
    or eax, 0x100           ; LME bit
    wrmsr
    
    ; Enable paging (enters Long Mode)
    mov eax, cr0
    or eax, 0x80000000      ; PG bit
    mov cr0, eax
    
    ; Load 64-bit GDT
    lgdt [0x10000 + gdt64_desc]
    
    ; Far jump to 64-bit code
    db 0xEA                        ; far jump opcode
    dd 0x10000 + lm_start          ; 32-bit offset
    dw 0x08                        ; segment selector

; ============================================================
; print_hex_byte32: Print AL as 2 hex chars to VGA at [edi]
; AH = attribute, EDI = VGA ptr (updated), AL = byte
; ============================================================
print_hex_byte32:
    push eax
    push ebx
    mov bl, al              ; save byte
    shr al, 4              ; high nibble
    call .nibble
    mov al, bl
    and al, 0x0F           ; low nibble
    call .nibble
    ; space separator
    mov al, ' '
    mov ah, 0x0A
    stosw
    pop ebx
    pop eax
    ret
.nibble:
    cmp al, 10
    jb .digit
    add al, 'A' - 10
    jmp .store
.digit:
    add al, '0'
.store:
    mov ah, 0x0A           ; green
    stosw
    ret

; ============================================================
; Setup 4-Level Paging (Identity map first 4MB)
; ============================================================
setup_paging:
    ; Clear page table area (0x70000 - 0x74000)
    mov edi, 0x70000
    xor eax, eax
    mov ecx, 4096
    rep stosd
    
    ; PML4[0] -> PDPT at 0x71000
    mov dword [0x70000], 0x71003
    
    ; PDPT[0] -> PD at 0x72000
    mov dword [0x71000], 0x72003
    
    ; PD[0] -> 2MB page at 0x0 (PS=1 for 2MB page)
    ; This covers 0x0 - 0x1FFFFF including 0x100000 where kernel is
    mov dword [0x72000], 0x83
    
    ; PD[1] -> 2MB page at 0x200000
    mov dword [0x72008], 0x200083
    
    ; PD[2] -> 2MB page at 0x400000
    mov dword [0x72010], 0x400083
    
    ; PD[3] -> 2MB page at 0x600000
    mov dword [0x72018], 0x600083
    
    ; Set CR3 to PML4
    mov eax, 0x70000
    mov cr3, eax
    
    ret

; ============================================================
; 64-bit Long Mode
; ============================================================
use64

lm_start:
    ; Setup 64-bit segments
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    mov rsp, STACK_ADDR
    
    ; Copy kernel from 0x11000 to 0x100000 in 64-bit mode
    ; Kernel is embedded in loader at offset 0x1000 (0x10000 + 0x1000 = 0x11000)
    mov rsi, 0x11000
    mov rdi, 0x100000
    mov rcx, 4096           ; 32KB / 8 bytes per qword (kernel size is 32768)
    cld
    rep movsq
    
    ; Clear screen with green on black
    mov rdi, 0xB8000
    mov rcx, 2000
    mov rax, 0x0A200A200A200A20
    rep stosq
    
    ; Print "FastOS v2.0 - 64-bit Long Mode" at row 1
    mov rdi, 0xB8000 + 160
    mov rsi, 0x10000 + msg_64
    mov ah, 0x0A            ; Green
.print64:
    lodsb
    test al, al
    jz .print64_done
    stosw
    jmp .print64
.print64_done:
    
    ; Print "C Kernel Ready" at row 2
    mov rdi, 0xB8000 + 320
    mov rsi, 0x10000 + msg_kernel
    mov ah, 0x0E            ; Yellow
.print_k:
    lodsb
    test al, al
    jz .print_k_done
    stosw
    jmp .print_k
.print_k_done:
    
    ; Print "[BG] Binary Guardian: ACTIVE" at row 4
    mov rdi, 0xB8000 + 640
    mov rsi, 0x10000 + msg_bg
    mov ah, 0x0B            ; Cyan
.print_bg:
    lodsb
    test al, al
    jz .print_bg_done
    stosw
    jmp .print_bg
.print_bg_done:
    
    ; Print "ADead-BIB Compiler" at row 5
    mov rdi, 0xB8000 + 800
    mov rsi, 0x10000 + msg_compiler
    mov ah, 0x0D            ; Magenta
.print_c:
    lodsb
    test al, al
    jz .print_c_done
    stosw
    jmp .print_c
.print_c_done:
    
    ; Print "Jumping to C Kernel..." at row 6
    mov rdi, 0xB8000 + 960
    mov rsi, 0x10000 + msg_jump
    mov ah, 0x0E            ; Yellow
.print_jump:
    lodsb
    test al, al
    jz .print_jump_done
    stosw
    jmp .print_jump
.print_jump_done:
    
    ; Small delay to see messages
    mov ecx, 0x1000000
.delay:
    dec ecx
    jnz .delay
    
    ; ============================================================
    ; Jump to C Kernel at 0x100000
    ; The kernel is loaded by the build script after the loader
    ; ============================================================
    
    ; DEBUG: Show byte at 0x11000 (source) on row 7
    mov rdi, 0xB8000 + 1120  ; Row 7
    mov byte [rdi], 'S'
    mov byte [rdi+1], 0x0A
    mov byte [rdi+2], ':'
    mov byte [rdi+3], 0x0A
    mov al, [0x11000]
    mov bl, al
    shr al, 4
    add al, '0'
    cmp al, '9'
    jbe .s1
    add al, 7
.s1: mov [rdi+4], al
    mov byte [rdi+5], 0x0E
    mov al, bl
    and al, 0x0F
    add al, '0'
    cmp al, '9'
    jbe .s2
    add al, 7
.s2: mov [rdi+6], al
    mov byte [rdi+7], 0x0E

    ; DEBUG: Show byte at 0x100000 (dest) on row 7
    mov byte [rdi+10], 'D'
    mov byte [rdi+11], 0x0A
    mov byte [rdi+12], ':'
    mov byte [rdi+13], 0x0A
    mov al, [0x100000]
    mov bl, al
    shr al, 4
    add al, '0'
    cmp al, '9'
    jbe .d1
    add al, 7
.d1: mov [rdi+14], al
    mov byte [rdi+15], 0x0E
    mov al, bl
    and al, 0x0F
    add al, '0'
    cmp al, '9'
    jbe .d2
    add al, 7
.d2: mov [rdi+16], al
    mov byte [rdi+17], 0x0E

    ; Check if kernel exists
    mov al, [0x100000]
    test al, al
    jz .no_kernel
    
    mov rax, 0x100000
    call rax
    
    ; If kernel returns, fall through to shell
    jmp .start_shell
    
.no_kernel:
    ; Print "No kernel found, starting shell..." at row 7
    mov rdi, 0xB8000 + 1120
    mov rsi, 0x10000 + msg_no_kernel
    mov ah, 0x0C            ; Red
.print_nk:
    lodsb
    test al, al
    jz .print_nk_done
    stosw
    jmp .print_nk
.print_nk_done:
    
.start_shell:
    ; Print prompt at row 8
    mov rdi, 0xB8000 + 1280
    mov rsi, 0x10000 + msg_prompt
    mov ah, 0x0F            ; White
.print_p:
    lodsb
    test al, al
    jz .print_p_done
    stosw
    jmp .print_p
.print_p_done:
    
    ; ============================================================
    ; FastOS Shell - Modern Keyboard Support
    ; Features: Shift, Caps Lock, F1-F12, Arrow keys, Home/End
    ; ============================================================
    
    ; Initialize keyboard state
    xor r8, r8                    ; Shift state (0=off, 1=on)
    xor r9, r9                    ; Caps Lock state
    xor r10, r10                  ; Command buffer position
    mov r11, 0xB8000 + 1280 + 16  ; Cursor position (after "fastos> ")
    xor r12, r12                  ; Blink counter
    mov r13, 8                    ; Current row (starting row)
    
    ; Command buffer at 0x80000
    mov rdi, 0x80000
    xor eax, eax
    mov ecx, 256
    rep stosb
    
.shell_loop:
    ; Blink cursor
    inc r12d
    test r12d, 0x40000
    jnz .cursor_block
    mov word [r11], 0x0F5F        ; White underscore
    jmp .poll_keyboard
.cursor_block:
    mov word [r11], 0x0FDB        ; White block
    
.poll_keyboard:
    in al, 0x64
    test al, 1
    jz .shell_loop
    
    in al, 0x60
    movzx eax, al
    
    ; Handle key release
    cmp al, 0xAA                  ; Left Shift release
    je .shift_release
    cmp al, 0xB6                  ; Right Shift release
    je .shift_release
    test al, 0x80
    jnz .shell_loop
    
    ; Handle modifier keys
    cmp al, 0x2A                  ; Left Shift press
    je .shift_press
    cmp al, 0x36                  ; Right Shift press
    je .shift_press
    cmp al, 0x3A                  ; Caps Lock
    je .caps_toggle
    
    ; Special keys
    cmp al, 0x01                  ; ESC
    je .halt_system
    cmp al, 0x1C                  ; Enter
    je .process_command
    cmp al, 0x0E                  ; Backspace
    je .backspace
    cmp al, 0x0F                  ; Tab
    je .tab_key
    
    ; Function keys F1-F10
    cmp al, 0x3B
    jb .normal_key
    cmp al, 0x44
    jbe .function_key
    
    ; Arrow keys (extended)
    cmp al, 0x48                  ; Up
    je .arrow_up
    cmp al, 0x50                  ; Down
    je .arrow_down
    
.normal_key:
    ; Convert scancode to ASCII
    cmp eax, 58
    jae .shell_loop
    
    ; Choose table based on Shift/Caps state
    mov rsi, 0x10000 + scancode_lower
    test r8, r8
    jnz .use_upper
    test r9, r9
    jz .do_lookup
    ; Caps Lock only affects letters
    cmp al, 0x10
    jb .do_lookup
    cmp al, 0x32
    ja .do_lookup
.use_upper:
    mov rsi, 0x10000 + scancode_upper
    
.do_lookup:
    add rsi, rax
    mov al, [rsi]
    test al, al
    jz .shell_loop
    
    ; Store in command buffer
    cmp r10, 60
    jae .shell_loop
    mov rdi, 0x80000
    add rdi, r10
    mov [rdi], al
    inc r10
    
    ; Print character
    mov word [r11], 0x0A20        ; Clear cursor
    mov ah, 0x0A                  ; Green
    mov [r11], ax
    add r11, 2
    jmp .shell_loop
    
.shift_press:
    mov r8, 1
    jmp .shell_loop
    
.shift_release:
    xor r8, r8
    jmp .shell_loop
    
.caps_toggle:
    xor r9, 1
    jmp .shell_loop
    
.tab_key:
    ; Insert 4 spaces
    mov ecx, 4
.tab_loop:
    cmp r10, 60
    jae .shell_loop
    mov rdi, 0x80000
    add rdi, r10
    mov byte [rdi], ' '
    inc r10
    mov word [r11], 0x0A20
    add r11, 2
    dec ecx
    jnz .tab_loop
    jmp .shell_loop
    
.backspace:
    test r10, r10
    jz .shell_loop
    dec r10
    sub r11, 2
    mov word [r11], 0x0A20
    jmp .shell_loop
    
.arrow_up:
    ; TODO: Command history
    jmp .shell_loop
    
.arrow_down:
    ; TODO: Command history
    jmp .shell_loop
    
.function_key:
    ; F1 = Help, F2 = Info, F10 = Exit
    sub al, 0x3B                  ; F1 = 0, F2 = 1, etc.
    cmp al, 0                     ; F1
    je .show_help
    cmp al, 1                     ; F2
    je .show_info
    cmp al, 9                     ; F10
    je .halt_system
    jmp .shell_loop
    
.show_help:
    ; Print help on next line
    add r13, 1
    mov rdi, 0xB8000
    mov rax, r13
    imul rax, 160
    add rdi, rax
    mov rsi, 0x10000 + msg_help
    mov ah, 0x0E                  ; Yellow
.print_help:
    lodsb
    test al, al
    jz .after_help
    stosw
    jmp .print_help
.after_help:
    inc r13
    jmp .new_prompt
    
.show_info:
    add r13, 1
    mov rdi, 0xB8000
    mov rax, r13
    imul rax, 160
    add rdi, rax
    mov rsi, 0x10000 + msg_info
    mov ah, 0x0B                  ; Cyan
.print_info:
    lodsb
    test al, al
    jz .after_info
    stosw
    jmp .print_info
.after_info:
    inc r13
    jmp .new_prompt
    
.process_command:
    ; Null-terminate command
    mov rdi, 0x80000
    add rdi, r10
    mov byte [rdi], 0
    
    ; Clear cursor
    mov word [r11], 0x0A20
    
    ; Move to next line
    inc r13
    
    ; Check commands
    mov rsi, 0x80000
    
    ; "help" command
    mov rdi, 0x10000 + cmd_help
    call .strcmp
    test eax, eax
    jz .exec_help
    
    ; "clear" command
    mov rdi, 0x10000 + cmd_clear
    call .strcmp
    test eax, eax
    jz .exec_clear
    
    ; "info" command
    mov rdi, 0x10000 + cmd_info
    call .strcmp
    test eax, eax
    jz .exec_info
    
    ; "exit" command
    mov rdi, 0x10000 + cmd_exit
    call .strcmp
    test eax, eax
    jz .halt_system
    
    ; Unknown command
    cmp r10, 0
    je .new_prompt
    
    mov rdi, 0xB8000
    mov rax, r13
    imul rax, 160
    add rdi, rax
    mov rsi, 0x10000 + msg_unknown
    mov ah, 0x0C                  ; Red
.print_unknown:
    lodsb
    test al, al
    jz .after_unknown
    stosw
    jmp .print_unknown
.after_unknown:
    inc r13
    jmp .new_prompt
    
.exec_help:
    mov rdi, 0xB8000
    mov rax, r13
    imul rax, 160
    add rdi, rax
    mov rsi, 0x10000 + msg_help_full
    mov ah, 0x0E
.print_help_full:
    lodsb
    test al, al
    jz .help_done
    cmp al, 10
    je .help_newline
    stosw
    jmp .print_help_full
.help_newline:
    inc r13
    mov rdi, 0xB8000
    mov rax, r13
    imul rax, 160
    add rdi, rax
    jmp .print_help_full
.help_done:
    inc r13
    jmp .new_prompt
    
.exec_clear:
    ; Clear screen
    mov rdi, 0xB8000
    mov rcx, 2000
    mov rax, 0x0A200A200A200A20
    rep stosq
    mov r13, 0
    jmp .new_prompt
    
.exec_info:
    mov rdi, 0xB8000
    mov rax, r13
    imul rax, 160
    add rdi, rax
    mov rsi, 0x10000 + msg_info_full
    mov ah, 0x0B
.print_info_full:
    lodsb
    test al, al
    jz .info_done
    stosw
    jmp .print_info_full
.info_done:
    inc r13
    jmp .new_prompt
    
.new_prompt:
    ; Clear command buffer
    xor r10, r10
    mov rdi, 0x80000
    xor eax, eax
    mov ecx, 64
    rep stosb
    
    ; Print new prompt
    mov rdi, 0xB8000
    mov rax, r13
    imul rax, 160
    add rdi, rax
    mov rsi, 0x10000 + msg_prompt
    mov ah, 0x0F                  ; White
.print_prompt:
    lodsb
    test al, al
    jz .prompt_done
    stosw
    jmp .print_prompt
.prompt_done:
    mov r11, rdi
    jmp .shell_loop
    
.strcmp:
    ; Compare strings at RSI and RDI
    ; Returns EAX=0 if equal
    push rsi
    push rdi
.strcmp_loop:
    mov al, [rsi]
    mov ah, [rdi]
    cmp al, ah
    jne .strcmp_ne
    test al, al
    jz .strcmp_eq
    inc rsi
    inc rdi
    jmp .strcmp_loop
.strcmp_eq:
    xor eax, eax
    pop rdi
    pop rsi
    ret
.strcmp_ne:
    mov eax, 1
    pop rdi
    pop rsi
    ret
    
.halt_system:
    mov rdi, 0xB8000
    mov rcx, 2000
    mov rax, 0x4F204F204F204F20
    rep stosq
    
    mov rdi, 0xB8000 + 1920
    mov rsi, 0x10000 + msg_halt
    mov ah, 0x4F
.print_halt:
    lodsb
    test al, al
    jz .halt_done
    stosw
    jmp .print_halt
.halt_done:
    cli
.halt_loop:
    hlt
    jmp .halt_loop

; ============================================================
; Scancode Tables (Lower and Upper case)
; ============================================================
scancode_lower:
    db 0, 27, '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', 8
    db 9, 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', 13
    db 0, 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', "'", '`'
    db 0, '\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', 0, '*', 0, ' '

scancode_upper:
    db 0, 27, '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', 8
    db 9, 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '{', '}', 13
    db 0, 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ':', '"', '~'
    db 0, '|', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', '<', '>', '?', 0, '*', 0, ' '

; Commands
cmd_help:   db "help", 0
cmd_clear:  db "clear", 0
cmd_info:   db "info", 0
cmd_exit:   db "exit", 0

; Messages
msg_help:       db "F1=Help F2=Info F10=Exit | Shift+Key for symbols", 0
msg_info:       db "FastOS v2.0 - 64-bit - ADead-BIB Compiler", 0
msg_unknown:    db "Unknown command. Type 'help' for commands.", 0
msg_help_full:  db "Commands: help, clear, info, exit", 0
msg_info_full:  db "FastOS v2.0 | 64-bit Long Mode | ADead-BIB | BG Active", 0

; ============================================================
; Data Section
; ============================================================
msg_s2:       db "FastOS Stage2 Loaded", 13, 10, 0
msg_a20:      db "A20 Line Enabled", 13, 10, 0
msg_pm:       db "Entering Protected Mode...", 13, 10, 0
msg_32:       db "32-bit Protected Mode OK", 0
msg_64:       db "FastOS v2.0 - 64-bit Long Mode Active!", 0
msg_kernel:   db "C Kernel: Ready for ADead-BIB compilation", 0
msg_bg:       db "[BG] Binary Guardian: ACTIVE", 0
msg_compiler: db "Compiler: ADead-BIB (No GCC, No LLVM, No Clang)", 0
msg_prompt:   db "fastos> ", 0
msg_halt:     db "=== System Halted - Press RESET ===", 0
msg_jump:     db "Jumping to C Kernel...", 0
msg_no_kernel: db "No kernel found, starting shell...", 0
msg_load_kernel: db "Loading kernel from disk...", 13, 10, 0
msg_kernel_ok: db "Kernel loaded OK", 13, 10, 0
msg_kernel_fail: db "Kernel load failed", 13, 10, 0
msg_dbg_src: db "src: ", 0
msg_dbg_dst: db "dst: ", 0

boot_drive: db 0

; Disk Address Packet for LBA read
align 4
dap:
    db 0x10           ; Size of DAP (16 bytes)
    db 0              ; Reserved
    dw 64             ; Number of sectors to read
    dw 0x0000         ; Offset (0x2000:0x0000)
    dw 0x2000         ; Segment
    dq 33             ; LBA (sector 33 = after MBR + 32 loader sectors)

; ============================================================
; GDT for Protected Mode (32-bit)
; ============================================================
align 16
gdt_start:
    ; Null descriptor
    dq 0
    
    ; Code segment 0x08: base=0, limit=4GB, 32-bit, ring 0
    dw 0xFFFF       ; Limit low
    dw 0x0000       ; Base low
    db 0x00         ; Base mid
    db 0x9A         ; Access: present, ring 0, code, readable
    db 0xCF         ; Flags: 4KB granularity, 32-bit
    db 0x00         ; Base high
    
    ; Data segment 0x10: base=0, limit=4GB, 32-bit, ring 0
    dw 0xFFFF
    dw 0x0000
    db 0x00
    db 0x92         ; Access: present, ring 0, data, writable
    db 0xCF
    db 0x00
gdt_end:

gdt_desc:
    dw gdt_end - gdt_start - 1
    dd 0x10000 + gdt_start

; ============================================================
; GDT for Long Mode (64-bit)
; ============================================================
align 16
gdt64_start:
    ; Null descriptor
    dq 0
    
    ; Code segment 0x08: 64-bit, ring 0
    dw 0x0000       ; Limit (ignored in 64-bit)
    dw 0x0000       ; Base low
    db 0x00         ; Base mid
    db 0x9A         ; Access: present, ring 0, code
    db 0x20         ; Flags: 64-bit code (L bit)
    db 0x00         ; Base high
    
    ; Data segment 0x10: 64-bit, ring 0
    dw 0x0000
    dw 0x0000
    db 0x00
    db 0x92         ; Access: present, ring 0, data
    db 0x00
    db 0x00
gdt64_end:

gdt64_desc:
    dw gdt64_end - gdt64_start - 1
    dq 0x10000 + gdt64_start

; ============================================================
; Padding to 16KB
; ============================================================
times 16384 - ($ - $$) db 0
