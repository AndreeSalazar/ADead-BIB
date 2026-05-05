// Test 21: Fibonacci recursive
int fib(int n) {
    if (n < 2) return n;
    return fib(n - 1) + fib(n - 2);
}

int main() {
    return fib(10) - 55;  // F(10) = 55
}
