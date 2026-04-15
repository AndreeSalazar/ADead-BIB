// ============================================================
// Test 10: Enums y Unions — enumeraciones, uniones, type punning
// ============================================================
// ADead-BIB Test Canon — C99 §6.7.2.2, §6.7.2.1
// Verifica: enum values, union overlapping, sizeof union
// ============================================================

#include <stdio.h>

enum Direction { NORTH = 0, EAST = 1, SOUTH = 2, WEST = 3 };

enum Color {
    COLOR_RED = 0xFF0000,
    COLOR_GREEN = 0x00FF00,
    COLOR_BLUE = 0x0000FF,
    COLOR_WHITE = 0xFFFFFF,
    COLOR_BLACK = 0x000000
};

enum AutoEnum { A, B, C, D, E };

enum MixedEnum { X = 10, Y, Z = 20, W };

union IntFloat {
    int i;
    float f;
};

union Variant {
    int as_int;
    float as_float;
    char as_char;
    unsigned char as_bytes[4];
};

const char *direction_name(enum Direction d) {
    switch (d) {
        case NORTH: return "North";
        case EAST:  return "East";
        case SOUTH: return "South";
        case WEST:  return "West";
        default:    return "Unknown";
    }
}

enum Direction turn_right(enum Direction d) {
    return (d + 1) % 4;
}

int main() {
    // --- Enum básico ---
    enum Direction dir = NORTH;
    printf("dir=%d name=%s\n", dir, direction_name(dir));

    dir = turn_right(dir);
    printf("turned=%d name=%s\n", dir, direction_name(dir));

    // --- Enum con valores explícitos ---
    enum Color c = COLOR_RED;
    printf("red=0x%x green=0x%x blue=0x%x\n", COLOR_RED, COLOR_GREEN, COLOR_BLUE);

    // --- Auto enum ---
    printf("auto: A=%d B=%d C=%d D=%d E=%d\n", A, B, C, D, E);

    // --- Mixed enum ---
    printf("mixed: X=%d Y=%d Z=%d W=%d\n", X, Y, Z, W);

    // --- Union básica ---
    union IntFloat u;
    u.i = 42;
    printf("union_int=%d\n", u.i);
    u.f = 3.14f;
    printf("union_float_as_int=0x%x\n", u.i);

    // --- Union sizeof ---
    printf("sizeof(IntFloat)=%d\n", (int)sizeof(union IntFloat));
    printf("sizeof(Variant)=%d\n", (int)sizeof(union Variant));

    // --- Union como byte inspector ---
    union Variant v;
    v.as_int = 0xDEADBEEF;
    printf("bytes: %02x %02x %02x %02x\n",
           v.as_bytes[0], v.as_bytes[1], v.as_bytes[2], v.as_bytes[3]);

    // --- Enum en switch ---
    int count = 0;
    enum Direction d;
    for (d = NORTH; d <= WEST; d++) {
        count++;
    }
    printf("directions=%d\n", count);

    return 0;
}
