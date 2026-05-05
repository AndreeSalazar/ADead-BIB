// Test 28: sizeof
int main() {
    int a = sizeof(char);     // 1
    int b = sizeof(short);    // 2
    int c = sizeof(int);      // 4
    int d = sizeof(long long);// 8
    return a + b + c + d - 15;
}
