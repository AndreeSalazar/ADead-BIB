int main() {
    long value = 42;
    int* bad_ptr = (int*)value; // UB: Type confusion / Invalid cast
    return 0;
}
