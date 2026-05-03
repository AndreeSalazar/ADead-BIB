// Test 08: Punteros básicos
int main() {
    int x = 10;
    int *p = &x;
    *p = 20;
    return x - 20;  // x ahora es 20, return 0
}
