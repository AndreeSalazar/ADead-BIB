// Test 09: Arrays
int main() {
    int arr[5];
    int i;
    int sum = 0;
    
    arr[0] = 1;
    arr[1] = 2;
    arr[2] = 3;
    arr[3] = 4;
    arr[4] = 5;
    
    for (i = 0; i < 5; i = i + 1) {
        sum = sum + arr[i];
    }
    
    return sum - 15;  // 1+2+3+4+5 = 15, return 0
}
