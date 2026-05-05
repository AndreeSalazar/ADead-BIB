// Test 36: Negative division (signed)
int main() {
    int a = -10;
    int b = 3;
    int q = a / b;   // -3
    int r = a % b;   // -1
    return q + r + 4;  // -3 + -1 + 4 = 0
}
