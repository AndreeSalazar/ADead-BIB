int main() {
    int x;
    int y = 0;
    int z = 10 / y; // Division by zero (L4)
    int* ptr = 0;
    *ptr = 5; // Null pointer dereference (L6)
    return z;
}
