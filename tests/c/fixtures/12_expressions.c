// Test: Expression coverage — all operators, casts, sizeof, ternary, comma
// Expected: All parse + lower correctly

int test_arithmetic() {
    int a = 10, b = 3;
    int add = a + b;
    int sub = a - b;
    int mul = a * b;
    int div = a / b;
    int mod = a % b;
    return add + sub + mul + div + mod;
}

int test_bitwise() {
    int a = 0xFF;
    int b = 0x0F;
    int and_r = a & b;
    int or_r = a | b;
    int xor_r = a ^ b;
    int not_r = ~a;
    int shl = a << 2;
    int shr = a >> 4;
    return and_r + or_r + xor_r + not_r + shl + shr;
}

int test_comparison() {
    int a = 5, b = 10;
    int eq = (a == b);
    int ne = (a != b);
    int lt = (a < b);
    int gt = (a > b);
    int le = (a <= b);
    int ge = (a >= b);
    return eq + ne + lt + gt + le + ge;
}

int test_logical() {
    int a = 1, b = 0;
    int and_r = a && b;
    int or_r = a || b;
    int not_r = !a;
    return and_r + or_r + not_r;
}

int test_compound_assign() {
    int x = 10;
    x += 5;
    x -= 2;
    x *= 3;
    x /= 2;
    x %= 7;
    x &= 0xFF;
    x |= 0x10;
    x ^= 0x01;
    x <<= 2;
    x >>= 1;
    return x;
}

int test_ternary() {
    int x = 5;
    int y = (x > 3) ? 100 : 200;
    return y;
}

int test_cast() {
    double d = 3.14;
    int i = (int)d;
    void *p = (void *)0;
    return i;
}

int test_sizeof() {
    int a = sizeof(int);
    int b = sizeof(char);
    int c = sizeof(double);
    int d = sizeof(void *);
    return a + b + c + d;
}

int test_increment() {
    int x = 0;
    x++;
    ++x;
    x--;
    --x;
    return x;
}

int main() {
    test_arithmetic();
    test_bitwise();
    test_comparison();
    test_logical();
    test_compound_assign();
    test_ternary();
    test_cast();
    test_sizeof();
    test_increment();
    return 0;
}
