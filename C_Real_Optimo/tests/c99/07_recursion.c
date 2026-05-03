// Test 07: Recursión
int factorial(int n) {
    if (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}

int main() {
    int result = factorial(5);  // 5! = 120
    return result - 120;  // return 0
}
