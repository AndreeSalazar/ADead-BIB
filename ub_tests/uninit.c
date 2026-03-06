int main() {
    int uninit_var;
    int a = uninit_var; // UB: Use of uninitialized variable
    printf("%d\n", a);
    return 0;
}
