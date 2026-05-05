// Test 18: Nested for loops
int main() {
    int sum = 0;
    int i;
    int j;
    for (i = 0; i < 5; i = i + 1) {
        for (j = 0; j < 5; j = j + 1) {
            sum = sum + 1;
        }
    }
    return sum - 25;
}
