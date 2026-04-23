# ADead-BIB C Standard Library Organization

## Purpose
Central reference for all C standard library components across the ADead-BIB toolchain.

## Architecture

The C standard library in ADead-BIB spans multiple crates:

### 1. Header Declarations (Frontend)
**Crate:** `adeb-frontend-c`  
**File:** `src/stdlib.rs`  
**Contains:** All C header content (stdio.h, stdlib.h, string.h, etc.)  
**Status:** ✅ ~95% complete — all C99/C11 headers declared

### 2. Symbol Registries (Shared)
**Crate:** `adeb-stdlib`  
**Directory:** `src/c/`  
**Contains:** `fastos_*.rs` files — function/macro/type lists per header  
**Status:** ✅ Complete for core headers

### 3. IAT Registry (Backend)
**Crate:** `adeb-backend-x64`  
**File:** `src/lib.rs` → `iat_registry` module  
**Contains:** DLL import mappings (msvcrt.dll: 94 functions)  
**Status:** ✅ Expanded to 94 functions

### 4. Code Generation (Backend)
**Crate:** `adeb-backend-x64`  
**Directory:** `src/isa/compiler/`  
**Contains:** ISA codegen for C constructs  
**Status:** 🔴 Needs fixes (structs, floats, fn pointers, etc.)

### 5. C Driver (App)
**Crate:** `ADead-BIB-Main`  
**File:** `src/driver/c_driver.rs`  
**Contains:** Full C compilation pipeline orchestration  
**Status:** ✅ Working

## Header Coverage

| Header | Declarations | IAT | Codegen | Tests |
|--------|-------------|-----|---------|-------|
| stdio.h | ✅ 35 func | ✅ 37 func | 🟡 printf works | fase1_libc/01-04 |
| stdlib.h | ✅ 30 func | ✅ 27 func | 🟡 malloc/free work | fase1_libc/05-09 |
| string.h | ✅ 24 func | ✅ 23 func | 🟡 10/24 work | fase1_libc/10-12 |
| math.h | ✅ 50+ func | ✅ 22 func | 🔴 needs float codegen | fase1_libc/13-14 |
| time.h | ✅ 15 func | ✅ 9 func | 🔴 needs struct codegen | fase1_libc/15 |
| ctype.h | ✅ 13 func | N/A (inline) | ✅ complete | fase1_libc/16 |
| signal.h | ✅ 4 func | ✅ 2 func | 🔴 needs fn ptr | fase1_libc/17 |
| errno.h | ✅ macros | ✅ _errno | 🔴 needs globals | fase1_libc/18 |
| stdarg.h | ✅ macros | N/A (intrinsic) | 🔴 needs va_list | fase1_libc/19 |
| locale.h | ✅ 2 func | ✅ 2 func | 🔴 needs struct | fase1_libc/20 |
| limits.h | ✅ macros | N/A | ✅ complete | fase1_libc/21 |
| setjmp.h | ✅ 2 func | 🔴 needs ASM | 🔴 needs ASM | fase1_libc/22 |
| float.h | ✅ macros | N/A | ✅ complete | N/A |
| stdint.h | ✅ types+macros | N/A | ✅ complete | fase1_libc/21 |
| stdbool.h | ✅ macros | N/A | ✅ complete | N/A |
| stddef.h | ✅ types+macros | N/A | ✅ complete | N/A |
