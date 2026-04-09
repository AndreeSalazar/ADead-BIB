// ADead-BIB Bridge Test 18 — Bitfields & Packed Structs
// Level: INTERMEDIATE
// Tests: struct bitfields, packed structs, union type-punning

#include <stdio.h>
#include <string.h>

struct Flags {
    unsigned int a:1;
    unsigned int b:3;
    unsigned int c:4;
    unsigned int d:8;
    unsigned int e:16;
};

struct Pixel {
    unsigned char r, g, b, a;
};

typedef union {
    int i;
    float f;
    unsigned char bytes[4];
} Pun;

int main() {
    printf("=== ADead-BIB Bridge Test 18: Bitfields ===\n");
    int pass = 0, fail = 0;

    // sizeof Flags == 4 (1+3+4+8+16 = 32 bits)
    if (sizeof(struct Flags) == 4) { pass++; } else { fail++; printf("FAIL: sizeof(Flags)=%d\n", (int)sizeof(struct Flags)); }

    // Set and read each bitfield independently
    struct Flags f;
    memset(&f, 0, sizeof(f));
    f.a = 1;
    if (f.a == 1) { pass++; } else { fail++; printf("FAIL: bitfield a\n"); }

    f.b = 5; // 3-bit field, max 7
    if (f.b == 5) { pass++; } else { fail++; printf("FAIL: bitfield b\n"); }

    f.c = 15; // 4-bit field, max 15
    if (f.c == 15) { pass++; } else { fail++; printf("FAIL: bitfield c\n"); }

    f.d = 200;
    if (f.d == 200) { pass++; } else { fail++; printf("FAIL: bitfield d\n"); }

    f.e = 50000;
    if (f.e == 50000) { pass++; } else { fail++; printf("FAIL: bitfield e\n"); }

    // Verify fields didn't clobber each other
    if (f.a == 1 && f.b == 5 && f.c == 15 && f.d == 200 && f.e == 50000) { pass++; } else { fail++; printf("FAIL: bitfield independence\n"); }

    // Union type-punning: write int, read bytes (little-endian)
    Pun p;
    p.i = 0x04030201;
    if (p.bytes[0] == 0x01 && p.bytes[1] == 0x02 && p.bytes[2] == 0x03 && p.bytes[3] == 0x04) { pass++; } else { fail++; printf("FAIL: union int->bytes\n"); }

    // Union type-punning: write float, read int bits
    Pun p2;
    p2.f = 1.0f; // IEEE 754: 0x3F800000
    if (p2.i == 0x3F800000) { pass++; } else { fail++; printf("FAIL: union float->int 0x%08X\n", p2.i); }

    // Packed Pixel struct — sizeof == 4
    if (sizeof(struct Pixel) == 4) { pass++; } else { fail++; printf("FAIL: sizeof(Pixel)=%d\n", (int)sizeof(struct Pixel)); }

    // Array of packed pixels — verify stride
    struct Pixel pixels[4];
    pixels[0] = (struct Pixel){255, 0, 0, 255};
    pixels[1] = (struct Pixel){0, 255, 0, 255};
    pixels[2] = (struct Pixel){0, 0, 255, 255};
    pixels[3] = (struct Pixel){128, 128, 128, 255};
    unsigned char *raw = (unsigned char*)pixels;
    // pixels[2].b should be at offset 2*4 + 2 = 10
    if (raw[10] == 255) { pass++; } else { fail++; printf("FAIL: pixel stride raw[10]=%d\n", raw[10]); }
    // pixels[3].r should be at offset 3*4 = 12
    if (raw[12] == 128) { pass++; } else { fail++; printf("FAIL: pixel stride raw[12]=%d\n", raw[12]); }

    printf("Results: %d passed, %d failed\n", pass, fail);
    printf("=== Test 18: %s ===\n", fail == 0 ? "PASS" : "FAIL");
    return fail;
}
