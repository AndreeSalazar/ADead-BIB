// Test 37: Iterative factorial
int main() {
    int n = 6;
    int f = 1;
    int i;
    for (i = 1; i <= n; i = i + 1) {
        f = f * i;
    }
    return f - 720;  // 6! = 720
}
