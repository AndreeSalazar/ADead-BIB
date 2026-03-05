// ============================================================
// Canon C99 — §6.7.2.1 Unions y Memoria Compartida
// ============================================================
// Intención: Una union ocupa el tamaño de su miembro más
// grande. Todos los miembros comparten la misma dirección.
//
// NOTA: ADead-BIB aún no soporta la keyword `union` en el
// parser C. Este test usa structs con casting para simular
// el concepto de tipo-punning que unions representan.
// Cuando ADead-BIB añada soporte de union, este test se
// actualizará con la sintaxis canónica.
//
// El concepto probado: reinterpretar los mismos bytes
// como diferentes tipos — que es la intención de union.
// ============================================================

#include <stdio.h>

// --- Tagged value (simula tagged union con struct) ---
enum ValueType { TYPE_INT = 0, TYPE_CHAR = 1 };

struct TaggedValue {
    int type;
    int data;
};

void print_tagged(struct TaggedValue *tv) {
    if (tv->type == TYPE_INT) {
        printf("  int: %d\n", tv->data);
    } else if (tv->type == TYPE_CHAR) {
        printf("  char: '%c'\n", (char)tv->data);
    }
}

// --- Byte view via pointer cast ---
void print_bytes(int value) {
    char *bytes = (char *)&value;
    printf("  bytes of 0x%X: [0x%02X, 0x%02X, 0x%02X, 0x%02X]\n",
        value,
        (unsigned char)bytes[0],
        (unsigned char)bytes[1],
        (unsigned char)bytes[2],
        (unsigned char)bytes[3]);
}

// --- Type punning via pointer ---
int float_bits(float f) {
    int *p = (int *)&f;
    return *p;
}

int main() {
    printf("=== Canon C99: Union Concepts (via struct) ===\n\n");

    // --- Tagged value ---
    printf("Tagged values:\n");
    struct TaggedValue tv1;
    tv1.type = TYPE_INT;
    tv1.data = 999;
    print_tagged(&tv1);

    struct TaggedValue tv2;
    tv2.type = TYPE_CHAR;
    tv2.data = 88;
    print_tagged(&tv2);

    struct TaggedValue tv3;
    tv3.type = TYPE_INT;
    tv3.data = -42;
    print_tagged(&tv3);

    // --- Byte view (type punning) ---
    printf("\nByte view (pointer cast):\n");
    int val = 0x41424344;
    print_bytes(val);

    int val2 = 0x00FF00FF;
    print_bytes(val2);

    // --- Reinterpret via cast ---
    printf("\nReinterpret:\n");
    int num = 65;
    char as_char = (char)num;
    printf("  int %d as char: '%c'\n", num, as_char);

    unsigned int bits = 0xFF000000;
    int as_signed = (int)bits;
    printf("  unsigned 0x%X as signed: %d\n", bits, as_signed);

    // --- Array of tagged values ---
    printf("\nArray of tagged values:\n");
    struct TaggedValue arr[3];
    arr[0].type = TYPE_INT;  arr[0].data = 100;
    arr[1].type = TYPE_CHAR; arr[1].data = 65;
    arr[2].type = TYPE_INT;  arr[2].data = 200;

    int i;
    for (i = 0; i < 3; i++) {
        printf("  [%d] ", i);
        print_tagged(&arr[i]);
    }

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (tv1.data == 999)     { pass++; } else { printf("FAIL: tagged int\n"); }
    total++; if (tv2.data == 88)      { pass++; } else { printf("FAIL: tagged char\n"); }
    total++; if (as_char == 'A')      { pass++; } else { printf("FAIL: reinterpret\n"); }
    total++; if (arr[0].data == 100)  { pass++; } else { printf("FAIL: arr[0]\n"); }
    total++; if (arr[2].data == 200)  { pass++; } else { printf("FAIL: arr[2]\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
