// Canon C11 -- Anonymous structs and unions
// C11 6.7.2.1: members without name
int printf(const char *format, ...);

struct Point3D {
    union {
        struct { int x; int y; int z; };
        int coords[3];
    };
};

struct Packet {
    int type;
    union {
        struct { int code; int value; };
        int raw[2];
    };
};

int main() {
    printf("=== Canon C11: Anonymous Structs/Unions ===\n\n");
    int pass = 0;
    int total = 0;

    struct Point3D p;
    p.x = 10;
    p.y = 20;
    p.z = 30;
    printf("p.x=%d p.y=%d p.z=%d\n", p.x, p.y, p.z);
    total++; if (p.x == 10) { pass++; } else { printf("FAIL: p.x\n"); }
    total++; if (p.y == 20) { pass++; } else { printf("FAIL: p.y\n"); }
    total++; if (p.z == 30) { pass++; } else { printf("FAIL: p.z\n"); }

    struct Packet pkt;
    pkt.type = 1;
    pkt.code = 42;
    pkt.value = 100;
    printf("pkt.type=%d pkt.code=%d pkt.value=%d\n", pkt.type, pkt.code, pkt.value);
    total++; if (pkt.type == 1) { pass++; } else { printf("FAIL: pkt.type\n"); }
    total++; if (pkt.code == 42) { pass++; } else { printf("FAIL: pkt.code\n"); }
    total++; if (pkt.value == 100) { pass++; } else { printf("FAIL: pkt.value\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}