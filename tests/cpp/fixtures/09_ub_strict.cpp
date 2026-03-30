// ADead-BIB C++ Fixture 09: UB Detection (Strict)
// Tests bit-width enforcement and narrowing detection
// This file has intentional UB for testing the detector

int printf(const char *format, ...);

int main() {
    // These should trigger bit-width violations in strict mode:
    // char c = 256;  // doesn't fit [-128, 127]
    // short s = 70000;  // doesn't fit [-32768, 32767]

    // Clean code — no UB
    int x = 42;
    char c = 65;
    short s = 1000;
    printf("x=%d c=%d s=%d\n", x, c, s);
    return 0;
}
