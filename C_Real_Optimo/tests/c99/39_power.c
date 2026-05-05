// Test 39: Integer power (recursive)
int power(int base, int exp) {
    if (exp == 0) return 1;
    return base * power(base, exp - 1);
}

int main() {
    return power(2, 10) - 1024;  // 2^10 = 1024
}
