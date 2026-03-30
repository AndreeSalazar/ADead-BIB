// ADead-BIB C++ Fixture 06: Assignments & Compound Assigns
// Tests proper lowering of x = val, x += val, ptr->field = val, etc.

int printf(const char *format, ...);

int main() {
    int x = 10;
    x = 20;
    x += 5;
    x -= 3;
    x *= 2;

    int arr[4];
    arr[0] = 100;

    printf("x = %d\n", x);
    return 0;
}
