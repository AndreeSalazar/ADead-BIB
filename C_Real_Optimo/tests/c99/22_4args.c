// Test 22: Function with 4 args (Win64 RCX, RDX, R8, R9)
int sum4(int a, int b, int c, int d) {
    return a + b + c + d;
}

int main() {
    return sum4(10, 20, 30, 40) - 100;
}
