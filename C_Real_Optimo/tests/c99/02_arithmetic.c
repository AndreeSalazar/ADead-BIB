// Test 02: Aritmética completa
int main() {
    int a = 20;
    int b = 4;
    int sum = a + b;      // 24
    int diff = a - b;     // 16
    int prod = a * b;     // 80
    int quot = a / b;     // 5
    int mod = a % b;      // 0
    
    int result = sum + diff + prod + quot + mod;
    return result - 125;  // 24+16+80+5+0 = 125, return 0
}
