# FASM Integration with ADead-BIB

## Descargar FASM

**Sitio Oficial:** https://flatassembler.net/download.php

### Windows x64
1. Descargar `fasmw17330.zip` (o versión más reciente)
2. Extraer a `C:\fasm\` o directorio preferido
3. Agregar `C:\fasm\` al PATH del sistema

### Archivos importantes de FASM:
```
fasm/
├── FASM.EXE          # Ensamblador principal (32-bit)
├── FASM.X64.EXE      # Ensamblador 64-bit (si disponible)
├── INCLUDE/          # Macros y headers
│   ├── MACRO/        # Macros estándar
│   ├── EQUATES/      # Constantes Windows
│   └── FORMAT/       # Formatos de salida (PE, ELF, etc.)
└── EXAMPLES/         # Ejemplos de código
```

## Integración con ADead-BIB

### Opción 1: FASM como backend de ensamblado
ADead-BIB genera código ensamblador → FASM lo ensambla a binario

```bash
# ADead-BIB genera .asm
adB cc myfile.c --emit-asm -o myfile.asm

# FASM ensambla a binario
fasm myfile.asm myfile.exe
```

### Opción 2: Inline assembly en C/C++
```c
// En código C/C++ de ADead-BIB
void fast_memcpy(void* dst, void* src, int size) {
    __asm {
        mov rdi, [dst]
        mov rsi, [src]
        mov rcx, [size]
        rep movsb
    }
}
```

### Opción 3: Raw bytes directos
```c
// ADead-BIB soporta raw bytes inline
raw { 0x90, 0x90, 0x90 }  // 3 NOPs
```

## Macros FASM útiles para OS Development

### Boot Sector (MBR)
```asm
format binary
org 0x7C00

start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    sti
    
    ; Tu código aquí
    
    jmp $

times 510-($-$$) db 0
dw 0xAA55
```

### PE Executable (Windows)
```asm
format PE64 console
entry main

section '.text' code readable executable
main:
    sub rsp, 40
    ; código
    xor eax, eax
    add rsp, 40
    ret

section '.data' data readable writeable
    message db 'Hello!', 0
```

### ELF Executable (Linux)
```asm
format ELF64 executable
entry main

segment readable executable
main:
    ; código
    mov eax, 60     ; sys_exit
    xor edi, edi
    syscall
```

## Configuración recomendada

Crear archivo `adead_fasm.bat` en tu PATH:
```batch
@echo off
REM ADead-BIB + FASM workflow
set FASM_PATH=C:\fasm
set ADEAD_PATH=C:\ADead-BIB

REM Compilar con ADead-BIB
%ADEAD_PATH%\target\debug\adead-bib.exe %*
```

## Recursos FASM

- **Manual oficial:** https://flatassembler.net/docs.php
- **Foro:** https://board.flatassembler.net/
- **GitHub (source):** https://github.com/tgrysztar/fasm
- **Wiki:** https://wiki.osdev.org/FASM

## Ventajas de FASM para ADead-BIB

1. **Multi-pass optimization** - Optimiza saltos automáticamente
2. **Macros potentes** - Sistema de macros Turing-completo
3. **Formatos múltiples** - PE, ELF, COFF, MachO, flat binary
4. **Self-hosting** - Escrito en ensamblador, muy rápido
5. **Sin dependencias** - No requiere linker externo
