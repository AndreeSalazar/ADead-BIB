// Test 19: Multiple functions chained
int square(int x) { return x * x; }
int cube(int x)   { return x * x * x; }
int double_(int x){ return x * 2; }

int main() {
    int a = square(3);    // 9
    int b = cube(2);      // 8
    int c = double_(5);   // 10
    return a + b + c - 27;
}
