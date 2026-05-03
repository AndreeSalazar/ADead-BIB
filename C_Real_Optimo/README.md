# C_Real_Optimo

Compilador C/C++ optimizado - Sin dependencias externas, 100% Rust.

## Arquitectura

```
C_Real_Optimo/
├── compiler/          # El compilador en sí
│   ├── frontend/      # C99: lexer, parser, preprocessor, AST
│   ├── middle/        # IR, optimizer, UB detector
│   ├── backend/       # codegen x86-64, encoder, PE/ELF
│   └── cli/           # adB — punto de entrada
│
├── runtime/           # adeb-rt-*.dll — runtime propio
│   ├── core/          # memoria, proceso, exit
│   ├── io/            # printf, fopen, read/write
│   ├── string/        # memcpy, strlen, strcmp
│   ├── math/          # sin, cos, sqrt
│   └── thread/        # threads, mutex, atomics
│
├── stdlib/            # C ABI headers — Win32, Vulkan, DX12
│   ├── win32/
│   ├── vulkan/
│   ├── dx12/
│   └── opengl/
│
├── asm/               # ASM-BIB — ensamblador
│
└── tests/             # Tests C separados por categoría
    ├── c99/
    ├── win32/
    └── gpu/
```

## Fuentes de conocimiento

Los siguientes repos fueron descargados en `../Real_compiler/`:

- wine/ — Windows API completo
- reactos/ — CRT + Kernel Windows
- glibc/ — CRT Linux/ELF
- cpython/ — Runtime Python/C ABI
- dxvk/ — D3D8/9/10/11 → Vulkan
- vkd3d-proton/ — D3D12 → Vulkan
- DirectX-Headers/ — oficial Microsoft MIT
- claudes-c-compiler/ — referencia

## Build

```bash
cargo build --release
```

## Uso

```bash
adB cc main.c -o main.exe
adB cxx main.cpp -o main.exe
adB run main.c
```

## Licencia

Techne License v1.0
