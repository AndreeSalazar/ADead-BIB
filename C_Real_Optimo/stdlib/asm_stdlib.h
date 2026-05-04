/**
 * asm_stdlib.h — ASM-BIB Standard Library declarations for ADead-BIB
 *
 * These functions are implemented in ASM-BIB (.pasm) and compiled to
 * native COFF .obj via `asm-bib --native`. Link with:
 *   adB cc myfile.c --link-obj asm_stdlib.obj
 *
 * All functions follow Windows x64 fastcall ABI:
 *   Args: RCX, RDX, R8, R9 (first 4), stack for rest
 *   Return: RAX
 *   Caller-saved: RAX, RCX, RDX, R8-R11
 *   Callee-saved: RBX, RBP, RDI, RSI, R12-R15
 */

#ifndef ASM_STDLIB_H
#define ASM_STDLIB_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ===== Memory Operations ===== */

/** Copy n bytes from src to dst. Returns dst. */
void* asm_memcpy(void* dst, const void* src, size_t n);

/** Fill n bytes of dst with byte c. Returns dst. */
void* asm_memset(void* dst, int c, size_t n);

/* ===== String Operations ===== */

/** Return length of null-terminated string s. */
size_t asm_strlen(const char* s);

/* ===== Math Operations ===== */

/** Absolute value of x. */
int64_t asm_abs(int64_t x);

/** Minimum of a and b. */
int64_t asm_min(int64_t a, int64_t b);

/** Maximum of a and b. */
int64_t asm_max(int64_t a, int64_t b);

/** Clamp x to range [lo, hi]. */
int64_t asm_clamp(int64_t x, int64_t lo, int64_t hi);

/** Swap values at *a and *b. */
void asm_swap(int64_t* a, int64_t* b);

/* ===== Bit Operations ===== */

/** Population count (number of set bits). Requires POPCNT CPU support. */
uint32_t asm_popcount(uint64_t x);

/** Bit Scan Reverse — position of highest set bit. */
uint32_t asm_bsr64(uint64_t x);

/** Bit Scan Forward — position of lowest set bit. */
uint32_t asm_bsf64(uint64_t x);

/** Byte-swap 32-bit value (endian conversion). */
uint32_t asm_bswap32(uint32_t x);

/** Byte-swap 64-bit value (endian conversion). */
uint64_t asm_bswap64(uint64_t x);

/** Check if pointer p is aligned to `alignment` bytes. Returns 1 if yes. */
int asm_is_aligned(void* p, size_t alignment);

/** Round v up to next multiple of align. */
size_t asm_align_up(size_t v, size_t align);

/** No-operation (useful for benchmarking overhead). */
void asm_noop(void);

#ifdef __cplusplus
}
#endif

#endif /* ASM_STDLIB_H */
