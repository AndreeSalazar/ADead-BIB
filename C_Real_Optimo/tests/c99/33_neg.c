// Test 33: Unary negation
int main() {
    int a = -5;
    int b = -(-3);
    int c = -a;     // 5
    return a + b + c - 3;  // -5 + 3 + 5 - 3 = 0
}
