// Test 12: Comparison operators
int main() {
    int a = 5;
    int b = 10;
    int eq = (a == 5);   // 1
    int ne = (a != b);   // 1
    int lt = (a < b);    // 1
    int le = (a <= 5);   // 1
    int gt = (b > a);    // 1
    int ge = (b >= 10);  // 1
    int sum = eq + ne + lt + le + gt + ge; // 6
    return sum - 6;
}
