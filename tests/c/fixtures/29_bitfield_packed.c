// ============================================================
// Test 29: Bitfields y Packed Structs — campos de bits, layout
// ============================================================
// ADead-BIB Test Canon — C99 §6.7.2.1
// Verifica: bitfield declaración, lectura, escritura, sizeof
// ============================================================

#include <stdio.h>

struct Flags {
    unsigned int visible : 1;
    unsigned int enabled : 1;
    unsigned int selected : 1;
    unsigned int focused : 1;
    unsigned int readonly : 1;
    unsigned int reserved : 27;
};

struct Color565 {
    unsigned short red : 5;
    unsigned short green : 6;
    unsigned short blue : 5;
};

struct Register {
    unsigned int opcode : 8;
    unsigned int dst : 4;
    unsigned int src1 : 4;
    unsigned int src2 : 4;
    unsigned int immediate : 12;
};

void print_flags(struct Flags f) {
    printf("vis=%d en=%d sel=%d foc=%d ro=%d\n",
           f.visible, f.enabled, f.selected, f.focused, f.readonly);
}

int main() {
    // --- Flags ---
    struct Flags f;
    f.visible = 1;
    f.enabled = 1;
    f.selected = 0;
    f.focused = 1;
    f.readonly = 0;
    f.reserved = 0;
    print_flags(f);

    // --- Toggle ---
    f.selected = 1;
    f.focused = 0;
    print_flags(f);

    // --- Sizeof (should be 4 bytes = 32 bits) ---
    printf("sizeof(Flags)=%d\n", (int)sizeof(struct Flags));

    // --- Color565 ---
    struct Color565 c;
    c.red = 31;
    c.green = 63;
    c.blue = 31;
    printf("color565: r=%d g=%d b=%d\n", c.red, c.green, c.blue);
    printf("sizeof(Color565)=%d\n", (int)sizeof(struct Color565));

    // --- Register encoding ---
    struct Register reg;
    reg.opcode = 0x3A;
    reg.dst = 0x1;
    reg.src1 = 0x2;
    reg.src2 = 0x3;
    reg.immediate = 0xFF;
    printf("reg: op=0x%x dst=%d src1=%d src2=%d imm=0x%x\n",
           reg.opcode, reg.dst, reg.src1, reg.src2, reg.immediate);
    printf("sizeof(Register)=%d\n", (int)sizeof(struct Register));

    // --- Bitfield con zero-width (alignment) ---
    struct Aligned {
        unsigned int a : 4;
        unsigned int : 0;
        unsigned int b : 4;
    };
    printf("sizeof(Aligned)=%d\n", (int)sizeof(struct Aligned));

    // --- Max values ---
    struct Flags max_f;
    max_f.visible = 1;
    max_f.enabled = 1;
    max_f.selected = 1;
    max_f.focused = 1;
    max_f.readonly = 1;
    max_f.reserved = (1 << 27) - 1;
    printf("max_reserved=%d\n", max_f.reserved);

    return 0;
}
