# C Header Reference

Maps each C standard header to its implementation locations in ADead-BIB.

## stdio.h
- **Declarations:** `frontend/c/adeb-frontend-c/src/stdlib.rs` → `HEADER_STDIO`
- **Symbol Registry:** `shared/adeb-stdlib/src/c/fastos_stdio.rs`
- **IAT (msvcrt.dll):** `backend/cpu/adeb-backend-x64/src/lib.rs` → DLL_IMPORTS[0]
- **Codegen:** `backend/cpu/adeb-backend-x64/src/isa/compiler/helpers.rs` → emit_iat_call()

## stdlib.h
- **Declarations:** `frontend/c/adeb-frontend-c/src/stdlib.rs` → `HEADER_STDLIB`
- **Symbol Registry:** `shared/adeb-stdlib/src/c/fastos_stdlib.rs`
- **IAT:** Same as stdio.h
- **Codegen:** Same as stdio.h

## string.h
- **Declarations:** `frontend/c/adeb-frontend-c/src/stdlib.rs` → `HEADER_STRING`
- **Symbol Registry:** `shared/adeb-stdlib/src/c/fastos_string.rs`
- **IAT:** Same as stdio.h
- **ASM-BIB Bridge:** `shared/adeb-bridge/` → asm_strlen, asm_strcpy, etc.

## math.h
- **Declarations:** `frontend/c/adeb-frontend-c/src/stdlib.rs` → `HEADER_MATH`
- **Symbol Registry:** `shared/adeb-stdlib/src/c/fastos_math.rs`
- **IAT:** msvcrt.dll (sin, cos, tan, sqrt, pow, etc.)
- **Requires:** Float/SSE2 codegen (Fix C-02)

## time.h
- **Declarations:** `frontend/c/adeb-frontend-c/src/stdlib.rs` → `HEADER_TIME`
- **Symbol Registry:** `shared/adeb-stdlib/src/c/fastos_time.rs`
- **IAT:** msvcrt.dll (time, clock, mktime, localtime, etc.)
- **Requires:** Struct codegen for `struct tm` (Fix C-01)

## signal.h
- **Declarations:** `frontend/c/adeb-frontend-c/src/stdlib.rs` → `HEADER_SIGNAL`
- **IAT:** msvcrt.dll (signal, raise)
- **Requires:** Function pointer codegen (Fix C-04)

## ctype.h
- **Declarations:** `frontend/c/adeb-frontend-c/src/stdlib.rs` → `HEADER_CTYPE`
- **Implementation:** Inline functions in header (static inline)
- **Status:** ✅ COMPLETE — no IAT needed
