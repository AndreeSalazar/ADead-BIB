#include <stdio.h>
#include <stdint.h>
#include <limits.h>
#include <stddef.h>
int main() {
    int8_t i8 = -128;
    uint8_t u8 = 255;
    int16_t i16 = -32768;
    uint16_t u16 = 65535;
    int32_t i32 = -2147483647;
    uint32_t u32 = 4294967295;
    int64_t i64 = -1;
    uint64_t u64 = 0;
    size_t sz = sizeof(int);
    ptrdiff_t pd = 42;
    intptr_t ip = 0;
    printf("i8=%d u8=%u i16=%d u16=%u\n", i8, u8, i16, u16);
    printf("i32=%d u32=%u\n", i32, u32);
    printf("i64=%lld u64=%llu\n", (long long)i64, (unsigned long long)u64);
    printf("size_t=%lu ptrdiff=%ld intptr=%ld\n", (unsigned long)sz, (long)pd, (long)ip);
    printf("INT_MAX=%d INT_MIN=%d\n", INT_MAX, INT_MIN);
    printf("CHAR_BIT=%d\n", CHAR_BIT);
    return 0;
}