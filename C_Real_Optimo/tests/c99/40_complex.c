// Test 40: Mixed expression
int abs_(int x) { return x < 0 ? -x : x; }
int max_(int a, int b) { return a > b ? a : b; }

int main() {
    int a = -5;
    int b = 7;
    int m = max_(abs_(a), abs_(b));   // max(5,7) = 7
    int s = abs_(a) + abs_(b);        // 12
    return m + s - 19;                // 7+12-19 = 0
}
