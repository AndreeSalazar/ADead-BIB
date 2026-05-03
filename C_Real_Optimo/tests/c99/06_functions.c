// Test 06: Funciones
int add(int a, int b) {
    return a + b;
}

int multiply(int x, int y) {
    return x * y;
}

int main() {
    int sum = add(3, 4);       // 7
    int prod = multiply(5, 6); // 30
    int total = add(sum, prod); // 37
    return total - 37;  // return 0
}
