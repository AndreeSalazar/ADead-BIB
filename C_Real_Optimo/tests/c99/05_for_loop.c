// Test 05: for loop
int main() {
    int sum = 0;
    int i;
    
    for (i = 0; i < 5; i = i + 1) {
        sum = sum + i;
    }
    
    return sum - 10;  // 0+1+2+3+4 = 10, return 0
}
