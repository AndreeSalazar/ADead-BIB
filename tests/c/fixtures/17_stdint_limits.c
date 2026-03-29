// Test: <stdint.h> + <limits.h> + <float.h> — Type sizes and limits
// Expected: Compile + Run OK

#include <stdint.h>
#include <limits.h>
#include <float.h>
#include <stdio.h>

int main() {
    printf("=== stdint/limits/float test ===\n");

    // stdint.h fixed-width types
    int8_t i8 = -128;
    uint8_t u8 = 255;
    int16_t i16 = -32768;
    uint16_t u16 = 65535;
    int32_t i32 = -2147483647;
    uint32_t u32 = 4294967295u;
    int64_t i64 = -1;
    uint64_t u64 = 0;

    printf("int8=%d uint8=%u\n", i8, u8);
    printf("int16=%d uint16=%u\n", i16, u16);
    printf("int32=%d uint32=%u\n", i32, u32);

    // limits.h
    printf("INT_MAX=%d INT_MIN=%d\n", INT_MAX, INT_MIN);
    printf("CHAR_BIT=%d\n", CHAR_BIT);

    printf("=== stdint/limits/float OK ===\n");
    return 0;
}
