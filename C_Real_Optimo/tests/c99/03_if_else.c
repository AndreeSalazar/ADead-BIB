// Test 03: if/else
int main() {
    int x = 10;
    int result;
    
    if (x > 5) {
        result = 1;
    } else {
        result = 0;
    }
    
    if (x < 5) {
        result = result + 10;
    }
    
    return result - 1;  // debe retornar 0
}
