# ASM-BIB — Ensamblador de ADead-BIB

**ASM-BIB es MASM reconstruido** — base de árbol de ensamblador para el compilador C.

## Pipeline

```
.pasm → ASM-BIB parser → COFF .obj → adeb-bridge → merge → PE
```

## 21 Funciones x86-64 (Win64 fastcall ABI)

| Categoría | Funciones |
|---|---|
| String   | `asm_strlen`, `asm_strcpy`, `asm_strcmp`, `asm_strcat`, `asm_strchr`, `asm_memcpy`, `asm_memset`, `asm_memcmp` |
| Math     | `asm_abs`, `asm_min`, `asm_max`, `asm_clamp`, `asm_swap` |
| Bit      | `asm_popcount`, `asm_bsr64`, `asm_bsf64`, `asm_bswap32`, `asm_bswap64` |
| Utility  | `asm_is_aligned`, `asm_align_up`, `asm_noop` |

## Calling convention (Win64)

- Args: `RCX, RDX, R8, R9` (luego stack)
- Return: `RAX`
- Volatile: `RAX, RCX, RDX, R8, R9, R10, R11`
- Preserve: `RBX, RSI, RDI, RBP, R12-R15`
- Shadow space: 32 bytes en stack

## Ver también

- [functions/](functions/) — Implementaciones `.pasm`
- `compiler/asm_bib.rs` — Parser .pasm → COFF (próximamente)
