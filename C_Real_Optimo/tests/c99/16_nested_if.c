// Test 16: Nested if/else
int main() {
    int x = 7;
    int r = 0;
    if (x > 0) {
        if (x > 5) {
            r = 1;
        } else {
            r = 2;
        }
    } else {
        r = 3;
    }
    return r - 1;  // expected r = 1
}
