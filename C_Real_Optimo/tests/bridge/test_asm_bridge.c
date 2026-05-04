// test_asm_bridge.c — Test ASM-BIB bridge integration
// Compile with: adB cc test_asm_bridge.c --link-obj asm_stdlib.obj
//
// This file calls functions implemented in ASM-BIB .pasm files.
// The bridge linker resolves these calls at link time.

#include "../stdlib/asm_stdlib.h"

int main() {
    // Test asm_strlen
    const char* hello = "Hello, World!";
    size_t len = asm_strlen(hello);  // Should be 13

    // Test asm_abs
    int64_t neg = -42;
    int64_t pos = asm_abs(neg);  // Should be 42

    // Test asm_min / asm_max
    int64_t a = 10, b = 20;
    int64_t mn = asm_min(a, b);  // Should be 10
    int64_t mx = asm_max(a, b);  // Should be 20

    // Test asm_clamp
    int64_t clamped = asm_clamp(50, 0, 100);  // Should be 50
    int64_t clamped_lo = asm_clamp(-5, 0, 100);  // Should be 0
    int64_t clamped_hi = asm_clamp(200, 0, 100);  // Should be 100

    // Test asm_popcount
    uint32_t bits = asm_popcount(0xFF);  // Should be 8

    // Test asm_bswap32
    uint32_t swapped = asm_bswap32(0x12345678);  // Should be 0x78563412

    // Return 0 if all tests pass
    int result = 0;
    if (len != 13) result = 1;
    if (pos != 42) result = 2;
    if (mn != 10) result = 3;
    if (mx != 20) result = 4;
    if (clamped != 50) result = 5;
    if (clamped_lo != 0) result = 6;
    if (clamped_hi != 100) result = 7;
    if (bits != 8) result = 8;
    if (swapped != 0x78563412) result = 9;

    return result;
}
