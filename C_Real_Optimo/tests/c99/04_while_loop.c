// Test 04: while loop
int main() {
    int sum = 0;
    int i = 1;
    
    while (i <= 10) {
        sum = sum + i;
        i = i + 1;
    }
    
    return sum - 55;  // 1+2+...+10 = 55, return 0
}
