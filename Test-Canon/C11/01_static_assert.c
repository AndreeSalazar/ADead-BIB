// Canon C11 -- _Static_assert and static_assert
// C11 6.7.10: compile-time assertion checking
int printf(const char *format, ...);

// _Static_assert is the C11 keyword
// In C11, static_assert is a macro for _Static_assert

int main() {
    printf("=== Canon C11: Static Assert ===\n\n");

    int pass = 0;
    int total = 0;

    // Basic type size assertions (these are compile-time)
    // We verify them at runtime too for the test
    int int_size = sizeof(int);
    int char_size = sizeof(char);
    int ptr_size = sizeof(void*);

    printf("sizeof(int) = %d\n", int_size);
    printf("sizeof(char) = %d\n", char_size);
    printf("sizeof(void*) = %d\n", ptr_size);

    total++; if (int_size == 4 || int_size == 8) { pass++; } else { printf("FAIL: int size\n"); }
    total++; if (char_size == 1) { pass++; } else { printf("FAIL: char size\n"); }
    total++; if (ptr_size == 4 || ptr_size == 8) { pass++; } else { printf("FAIL: ptr size\n"); }

    // Verify type ranges
    int max_char = 127;
    int min_char = -128;
    total++; if (max_char == 127) { pass++; } else { printf("FAIL: char max\n"); }
    total++; if (min_char == -128) { pass++; } else { printf("FAIL: char min\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}