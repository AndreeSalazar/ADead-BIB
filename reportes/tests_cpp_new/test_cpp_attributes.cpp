#include <header_main.h>

[[nodiscard]] int compute(int x) {
    return x * x;
}

[[deprecated("use compute instead")]]
int old_compute(int x) {
    return x * x;
}

void test_switch(int val) {
    switch (val) {
    case 1:
        printf("one\n");
        [[fallthrough]];
    case 2:
        printf("two or fallthrough\n");
        break;
    default:
        printf("other\n");
        break;
    }
}

int main() {
    int result = compute(5);
    printf("result: %d\n", result);

    [[maybe_unused]] int unused_var = 42;

    test_switch(1);
    test_switch(3);

    return 0;
}
