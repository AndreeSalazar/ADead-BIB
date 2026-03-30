// ADead-BIB C++ Fixture 05: Control Flow
// Tests if/else, while, for, switch

int printf(const char *format, ...);

int main() {
    int x = 10;

    if (x > 5) {
        printf("x > 5\n");
    } else {
        printf("x <= 5\n");
    }

    int sum = 0;
    for (int i = 0; i < 5; i = i + 1) {
        sum = sum + i;
    }
    printf("sum = %d\n", sum);

    int count = 3;
    while (count > 0) {
        count = count - 1;
    }

    switch (x) {
        case 10:
            printf("ten\n");
            break;
        default:
            printf("other\n");
            break;
    }

    return 0;
}
