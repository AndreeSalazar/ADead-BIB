// Test 32: Long arithmetic chain (regression for P-01)
int main() {
    int a = 1;
    int b = 2;
    int c = 3;
    int d = 4;
    int e = 5;
    int f = 6;
    int g = 7;
    int h = 8;
    int sum = a + b + c + d + e + f + g + h;  // 36
    int prod = a * b * c * d;  // 24
    int total = sum + prod;    // 60
    return total - 60;
}
