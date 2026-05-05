// Test 25: break / continue
int main() {
    int sum = 0;
    int i;
    for (i = 0; i < 100; i = i + 1) {
        if (i == 5) break;
        if (i == 2) continue;
        sum = sum + i;
    }
    // sum = 0+1+3+4 = 8
    return sum - 8;
}
