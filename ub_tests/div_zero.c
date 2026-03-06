int main() {
    int x = 10 / 0; // UB: Division by zero
    printf("%d\n", x);
    return 0;
}
