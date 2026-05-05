// Test 17: Nested loops
int main() {
    int sum = 0;
    int i = 0;
    int j;
    while (i < 3) {
        j = 0;
        while (j < 4) {
            sum = sum + 1;
            j = j + 1;
        }
        i = i + 1;
    }
    return sum - 12;  // 3 * 4 = 12
}
