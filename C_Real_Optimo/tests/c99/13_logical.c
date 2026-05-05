// Test 13: Logical operators
int main() {
    int x = 1;
    int y = 0;
    int and_ = x && y; // 0
    int or_  = x || y; // 1
    int neg  = !y;     // 1
    return and_ + or_ + neg - 2; // 0+1+1 - 2 = 0
}
