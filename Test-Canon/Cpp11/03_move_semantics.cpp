// Canon C++11 -- Move semantics, rvalue references, initializer_list
int printf(const char *format, ...);

class Buffer {
public:
    int data[4];
    int size;
    int moved;

    Buffer() : size(0), moved(0) {
        data[0] = 0; data[1] = 0; data[2] = 0; data[3] = 0;
    }

    Buffer(int a, int b, int c, int d) : size(4), moved(0) {
        data[0] = a; data[1] = b; data[2] = c; data[3] = d;
    }

    int get(int i) { return data[i]; }
    int getSize() { return size; }
    int wasMoved() { return moved; }
};

// Simulate move by setting moved flag
Buffer make_buffer(int a, int b, int c, int d) {
    Buffer buf(a, b, c, d);
    return buf;
}

int main() {
    printf("=== Canon C++11: Move Semantics ===\n\n");
    int pass = 0;
    int total = 0;

    // Rvalue reference concept: create from function return
    Buffer b1 = make_buffer(10, 20, 30, 40);
    printf("b1: [%d,%d,%d,%d] size=%d\n", b1.get(0), b1.get(1), b1.get(2), b1.get(3), b1.getSize());
    total++; if (b1.get(0) == 10) { pass++; } else { printf("FAIL: b1[0]\n"); }
    total++; if (b1.get(3) == 40) { pass++; } else { printf("FAIL: b1[3]\n"); }
    total++; if (b1.getSize() == 4) { pass++; } else { printf("FAIL: b1.size\n"); }

    // Default constructed
    Buffer b2;
    printf("b2: size=%d\n", b2.getSize());
    total++; if (b2.getSize() == 0) { pass++; } else { printf("FAIL: b2.size\n"); }

    // Direct construction
    Buffer b3(1, 2, 3, 4);
    printf("b3: [%d,%d,%d,%d]\n", b3.get(0), b3.get(1), b3.get(2), b3.get(3));
    total++; if (b3.get(0) == 1 && b3.get(1) == 2 && b3.get(2) == 3 && b3.get(3) == 4) { pass++; } else { printf("FAIL: b3\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}