; ============================================================
; FastOS v2.0 — 64-bit Kernel
; Desktop environment with mouse support
; ============================================================

format binary as 'bin'
org 0x100000

use64

; ============================================================
; Constants
; ============================================================
VGA_TEXT        equ 0xB8000
SCREEN_W        equ 80
SCREEN_H        equ 25

; Colors
ATTR_BLUE_WHITE equ 0x1F
ATTR_BLUE_YELLOW equ 0x1E
ATTR_GRAY_BLACK equ 0x70
ATTR_GREEN_BLACK equ 0x0A
ATTR_CYAN_BLACK equ 0x0B
ATTR_RED_WHITE  equ 0x4F
ATTR_WHITE_BLUE equ 0x17
ATTR_BLACK_CYAN equ 0x30

; Mouse ports
MOUSE_DATA      equ 0x60
MOUSE_STATUS    equ 0x64
MOUSE_CMD       equ 0x64

; ============================================================
; Kernel Entry Point
; ============================================================
kernel_start:
    ; Clear screen with blue background
    mov rdi, VGA_TEXT
    mov rcx, SCREEN_W * SCREEN_H
    mov ax, 0x1F20          ; Blue bg, white fg, space
    rep stosw
    
    ; Draw title bar
    mov rdi, VGA_TEXT
    mov rcx, SCREEN_W
    mov ax, 0x1720          ; White bg, blue fg
    rep stosw
    
    mov rdi, VGA_TEXT + 4
    mov rsi, title_fastos
    mov ah, 0x17
    call print_string
    
    ; Draw taskbar
    mov rdi, VGA_TEXT + (24 * SCREEN_W * 2)
    mov rcx, SCREEN_W
    mov ax, 0x7020          ; Gray bg
    rep stosw
    
    mov rdi, VGA_TEXT + (24 * SCREEN_W * 2) + 2
    mov rsi, taskbar_start
    mov ah, 0x70
    call print_string
    
    mov rdi, VGA_TEXT + (24 * SCREEN_W * 2) + 120
    mov rsi, taskbar_time
    mov ah, 0x70
    call print_string
    
    ; Draw desktop icons
    call draw_icons
    
    ; Draw terminal window
    call draw_terminal
    
    ; Initialize mouse
    call init_mouse
    
    ; Initialize cursor position
    mov dword [mouse_x], 40
    mov dword [mouse_y], 12
    
    ; Draw initial cursor
    call draw_cursor
    
    ; Main loop
    jmp main_loop

; ============================================================
; Main Loop - Handle keyboard and mouse
; ============================================================
main_loop:
    ; Check keyboard
    in al, 0x64
    test al, 1
    jz .check_mouse
    
    in al, 0x60
    
    ; ESC = exit
    cmp al, 0x01
    je halt_system
    
    ; F1 = help
    cmp al, 0x3B
    je show_help
    
.check_mouse:
    ; Check mouse data
    in al, 0x64
    test al, 0x21           ; Check if mouse data available
    jz .no_mouse
    test al, 0x20
    jz .no_mouse
    
    ; Read mouse packet
    call read_mouse_packet
    
    ; Update cursor
    call update_cursor
    
.no_mouse:
    ; Small delay
    mov ecx, 10000
.delay:
    dec ecx
    jnz .delay
    
    jmp main_loop

; ============================================================
; Initialize PS/2 Mouse
; ============================================================
init_mouse:
    ; Enable auxiliary device
    call mouse_wait_write
    mov al, 0xA8
    out MOUSE_CMD, al
    
    ; Enable interrupts
    call mouse_wait_write
    mov al, 0x20
    out MOUSE_CMD, al
    call mouse_wait_read
    in al, MOUSE_DATA
    or al, 2
    push ax
    call mouse_wait_write
    mov al, 0x60
    out MOUSE_CMD, al
    call mouse_wait_write
    pop ax
    out MOUSE_DATA, al
    
    ; Set defaults
    call mouse_wait_write
    mov al, 0xD4
    out MOUSE_CMD, al
    call mouse_wait_write
    mov al, 0xF6
    out MOUSE_DATA, al
    call mouse_wait_read
    in al, MOUSE_DATA
    
    ; Enable mouse
    call mouse_wait_write
    mov al, 0xD4
    out MOUSE_CMD, al
    call mouse_wait_write
    mov al, 0xF4
    out MOUSE_DATA, al
    call mouse_wait_read
    in al, MOUSE_DATA
    
    ret

mouse_wait_write:
    in al, MOUSE_STATUS
    test al, 2
    jnz mouse_wait_write
    ret

mouse_wait_read:
    in al, MOUSE_STATUS
    test al, 1
    jz mouse_wait_read
    ret

; ============================================================
; Read Mouse Packet
; ============================================================
read_mouse_packet:
    ; Read 3 bytes
    call mouse_wait_read
    in al, MOUSE_DATA
    mov [mouse_byte1], al
    
    call mouse_wait_read
    in al, MOUSE_DATA
    mov [mouse_byte2], al
    
    call mouse_wait_read
    in al, MOUSE_DATA
    mov [mouse_byte3], al
    
    ret

; ============================================================
; Update Cursor Position
; ============================================================
update_cursor:
    ; Erase old cursor
    call erase_cursor
    
    ; Get X movement
    movsx eax, byte [mouse_byte2]
    test byte [mouse_byte1], 0x10
    jz .x_positive
    or eax, 0xFFFFFF00
.x_positive:
    add [mouse_x], eax
    
    ; Get Y movement (inverted)
    movsx eax, byte [mouse_byte3]
    test byte [mouse_byte1], 0x20
    jz .y_positive
    or eax, 0xFFFFFF00
.y_positive:
    neg eax
    add [mouse_y], eax
    
    ; Clamp X
    cmp dword [mouse_x], 0
    jge .x_min_ok
    mov dword [mouse_x], 0
.x_min_ok:
    cmp dword [mouse_x], 79
    jle .x_max_ok
    mov dword [mouse_x], 79
.x_max_ok:
    
    ; Clamp Y
    cmp dword [mouse_y], 0
    jge .y_min_ok
    mov dword [mouse_y], 0
.y_min_ok:
    cmp dword [mouse_y], 24
    jle .y_max_ok
    mov dword [mouse_y], 24
.y_max_ok:
    
    ; Draw new cursor
    call draw_cursor
    
    ; Check for click
    test byte [mouse_byte1], 1
    jz .no_click
    call handle_click
.no_click:
    
    ret

; ============================================================
; Draw/Erase Cursor
; ============================================================
draw_cursor:
    mov eax, [mouse_y]
    imul eax, SCREEN_W * 2
    mov ebx, [mouse_x]
    shl ebx, 1
    add eax, ebx
    add rax, VGA_TEXT
    
    ; Save character under cursor
    mov bx, [rax]
    mov [cursor_save], bx
    
    ; Draw cursor (arrow character)
    mov word [rax], 0x0F18  ; White arrow
    ret

erase_cursor:
    mov eax, [mouse_y]
    imul eax, SCREEN_W * 2
    mov ebx, [mouse_x]
    shl ebx, 1
    add eax, ebx
    add rax, VGA_TEXT
    
    ; Restore character
    mov bx, [cursor_save]
    mov [rax], bx
    ret

; ============================================================
; Handle Mouse Click
; ============================================================
handle_click:
    ; Check if clicking on Start button
    cmp dword [mouse_y], 24
    jne .not_start
    cmp dword [mouse_x], 8
    ja .not_start
    
    ; Show start menu
    call show_start_menu
    ret
    
.not_start:
    ; Check if clicking on terminal icon
    cmp dword [mouse_y], 3
    jb .not_terminal
    cmp dword [mouse_y], 5
    ja .not_terminal
    cmp dword [mouse_x], 2
    jb .not_terminal
    cmp dword [mouse_x], 14
    ja .not_terminal
    
    ; Open terminal
    call open_terminal
    ret
    
.not_terminal:
    ret

; ============================================================
; Show Start Menu
; ============================================================
show_start_menu:
    ; Draw menu background
    mov rdi, VGA_TEXT + (18 * SCREEN_W * 2)
    mov ecx, 6
.menu_row:
    push rcx
    mov rcx, 20
    mov ax, 0x7020
    rep stosw
    add rdi, (SCREEN_W - 20) * 2
    pop rcx
    dec ecx
    jnz .menu_row
    
    ; Menu items
    mov rdi, VGA_TEXT + (18 * SCREEN_W * 2) + 2
    mov rsi, menu_programs
    mov ah, 0x70
    call print_string
    
    mov rdi, VGA_TEXT + (19 * SCREEN_W * 2) + 2
    mov rsi, menu_files
    mov ah, 0x70
    call print_string
    
    mov rdi, VGA_TEXT + (20 * SCREEN_W * 2) + 2
    mov rsi, menu_settings
    mov ah, 0x70
    call print_string
    
    mov rdi, VGA_TEXT + (21 * SCREEN_W * 2) + 2
    mov rsi, menu_terminal
    mov ah, 0x70
    call print_string
    
    mov rdi, VGA_TEXT + (22 * SCREEN_W * 2) + 2
    mov rsi, menu_shutdown
    mov ah, 0x74          ; Red text
    call print_string
    
    ret

; ============================================================
; Draw Desktop Icons
; ============================================================
draw_icons:
    ; Terminal icon
    mov rdi, VGA_TEXT + (3 * SCREEN_W * 2) + 4
    mov rsi, icon_terminal
    mov ah, ATTR_BLUE_YELLOW
    call print_string
    
    ; Files icon
    mov rdi, VGA_TEXT + (6 * SCREEN_W * 2) + 4
    mov rsi, icon_files
    mov ah, ATTR_BLUE_YELLOW
    call print_string
    
    ; Settings icon
    mov rdi, VGA_TEXT + (9 * SCREEN_W * 2) + 4
    mov rsi, icon_settings
    mov ah, ATTR_BLUE_YELLOW
    call print_string
    
    ; Info icon
    mov rdi, VGA_TEXT + (12 * SCREEN_W * 2) + 4
    mov rsi, icon_info
    mov ah, ATTR_BLUE_YELLOW
    call print_string
    
    ; Power icon
    mov rdi, VGA_TEXT + (15 * SCREEN_W * 2) + 4
    mov rsi, icon_power
    mov ah, 0x1C          ; Blue bg, red fg
    call print_string
    
    ret

; ============================================================
; Draw Terminal Window
; ============================================================
draw_terminal:
    ; Window frame (rows 2-20, cols 20-75)
    mov ebx, 2            ; Start row
.term_row:
    cmp ebx, 20
    jge .term_done
    
    mov eax, ebx
    imul eax, SCREEN_W * 2
    add eax, 40           ; Col 20 * 2
    add rax, VGA_TEXT
    mov rdi, rax
    
    cmp ebx, 2
    je .title_bar
    
    ; Content area
    mov rcx, 55
    mov ax, 0x0A20        ; Green on black
    rep stosw
    jmp .next_row
    
.title_bar:
    mov rcx, 55
    mov ax, 0x1720        ; White on blue
    rep stosw
    
    ; Title text
    mov rdi, VGA_TEXT + (2 * SCREEN_W * 2) + 42
    mov rsi, term_title
    mov ah, 0x17
    call print_string
    
    ; Close button
    mov rdi, VGA_TEXT + (2 * SCREEN_W * 2) + 146
    mov rsi, term_close
    mov ah, 0x4F
    call print_string
    
.next_row:
    inc ebx
    jmp .term_row
    
.term_done:
    ; Terminal content
    mov rdi, VGA_TEXT + (4 * SCREEN_W * 2) + 42
    mov rsi, term_welcome
    mov ah, 0x0A
    call print_string
    
    mov rdi, VGA_TEXT + (5 * SCREEN_W * 2) + 42
    mov rsi, term_info1
    mov ah, 0x0A
    call print_string
    
    mov rdi, VGA_TEXT + (6 * SCREEN_W * 2) + 42
    mov rsi, term_info2
    mov ah, 0x0B
    call print_string
    
    mov rdi, VGA_TEXT + (8 * SCREEN_W * 2) + 42
    mov rsi, term_prompt
    mov ah, 0x0F
    call print_string
    
    ret

open_terminal:
    ; Redraw terminal
    call draw_terminal
    ret

; ============================================================
; Show Help
; ============================================================
show_help:
    mov rdi, VGA_TEXT + (10 * SCREEN_W * 2) + 42
    mov rsi, help_text
    mov ah, 0x0E
    call print_string
    jmp main_loop

; ============================================================
; Halt System
; ============================================================
halt_system:
    ; Red screen
    mov rdi, VGA_TEXT
    mov rcx, SCREEN_W * SCREEN_H
    mov ax, 0x4F20
    rep stosw
    
    mov rdi, VGA_TEXT + (12 * SCREEN_W * 2) + 60
    mov rsi, halt_msg
    mov ah, 0x4F
    call print_string
    
    cli
.halt:
    hlt
    jmp .halt

; ============================================================
; Print String
; ============================================================
print_string:
    lodsb
    test al, al
    jz .done
    stosw
    jmp print_string
.done:
    ret

; ============================================================
; Data Section
; ============================================================
title_fastos:   db "FastOS v2.0 - Desktop Environment", 0
taskbar_start:  db "[Start]", 0
taskbar_time:   db "12:00 PM", 0

icon_terminal:  db "[>_] Terminal", 0
icon_files:     db "[#] Files", 0
icon_settings:  db "[@] Settings", 0
icon_info:      db "[i] Info", 0
icon_power:     db "[!] Power", 0

menu_programs:  db "> Programs", 0
menu_files:     db "> Files", 0
menu_settings:  db "> Settings", 0
menu_terminal:  db "> Terminal", 0
menu_shutdown:  db "> Shutdown", 0

term_title:     db "Terminal - FastOS", 0
term_close:     db "[X]", 0
term_welcome:   db "FastOS v2.0 - 64-bit Long Mode", 0
term_info1:     db "Compiler: ADead-BIB", 0
term_info2:     db "[BG] Binary Guardian: ACTIVE", 0
term_prompt:    db "fastos> _", 0

help_text:      db "F1=Help ESC=Exit Mouse=Navigate", 0
halt_msg:       db "System Halted", 0

; Mouse state
mouse_x:        dd 40
mouse_y:        dd 12
mouse_byte1:    db 0
mouse_byte2:    db 0
mouse_byte3:    db 0
cursor_save:    dw 0

; ============================================================
; Padding to 32KB
; ============================================================
times 32768 - ($ - $$) db 0
