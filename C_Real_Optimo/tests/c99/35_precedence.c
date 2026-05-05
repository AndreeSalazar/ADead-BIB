// Test 35: Operator precedence
int main() {
    int x = 2 + 3 * 4;       // 14
    int y = (2 + 3) * 4;     // 20
    int z = 2 * 3 + 4 * 5;   // 26
    int w = 100 / 5 / 4;     // 5  (left-to-right)
    return x + y + z + w - 65;
}
