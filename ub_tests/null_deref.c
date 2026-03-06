int main() {
    int* ptr = 0; // NULL
    *ptr = 42;    // UB: Null Pointer Dereference
    return 0;
}
