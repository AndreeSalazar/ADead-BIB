// Test: UB Detection — intentional undefined behavior for testing the UB detector
// Expected: UB detector should flag these issues

int test_div_zero() {
    int x = 10 / 0;  // UB: division by zero
    return x;
}

int test_shift_overflow() {
    int x = 1 << 64;  // UB: shift amount >= width
    return x;
}

int test_negative_index() {
    int arr[5] = {1, 2, 3, 4, 5};
    return arr[-1];  // UB: negative index
}

int main() {
    return 0;
}
