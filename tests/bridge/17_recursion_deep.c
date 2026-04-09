// ADead-BIB Bridge Test 17 — Deep Recursion
// Level: BASIC
// Tests: Ackermann function, recursive factorial, recursive binary search,
//        mutual recursion (is_even/is_odd), deep fibonacci, stack depth stress

#include <stdio.h>

// Ackermann function
int ackermann(int m, int n) {
    if (m == 0) return n + 1;
    if (n == 0) return ackermann(m - 1, 1);
    return ackermann(m - 1, ackermann(m, n - 1));
}

// Recursive factorial
long long factorial(int n) {
    if (n <= 1) return 1;
    return (long long)n * factorial(n - 1);
}

// Recursive binary search
int binary_search(int *arr, int lo, int hi, int target) {
    if (lo > hi) return -1;
    int mid = lo + (hi - lo) / 2;
    if (arr[mid] == target) return mid;
    if (arr[mid] < target) return binary_search(arr, mid + 1, hi, target);
    return binary_search(arr, lo, mid - 1, target);
}

// Mutual recursion: is_even / is_odd
int is_odd(int n);
int is_even(int n) {
    if (n == 0) return 1;
    return is_odd(n - 1);
}
int is_odd(int n) {
    if (n == 0) return 0;
    return is_even(n - 1);
}

// Recursive fibonacci
long long fib(int n) {
    if (n <= 1) return n;
    return fib(n - 1) + fib(n - 2);
}

// Tree-like recursion: count nodes in a complete binary tree of depth d
int tree_count(int depth) {
    if (depth == 0) return 1;
    return 1 + tree_count(depth - 1) + tree_count(depth - 1);
}

int main() {
    printf("=== ADead-BIB Bridge Test 17: Deep Recursion ===\n");
    int pass = 0, fail = 0;

    // 1. Ackermann(3, 4) = 125
    int ack = ackermann(3, 4);
    if (ack == 125) { pass++; } else { fail++; printf("FAIL: ackermann(3,4)=%d expected=125\n", ack); }

    // 2. Recursive factorial(12) = 479001600
    long long fact = factorial(12);
    if (fact == 479001600LL) { pass++; } else { fail++; printf("FAIL: factorial(12)=%lld\n", fact); }

    // 3. Recursive factorial(0) = 1
    if (factorial(0) == 1) { pass++; } else { fail++; printf("FAIL: factorial(0)\n"); }

    // 4. Recursive binary search
    int arr[16];
    for (int i = 0; i < 16; i++) arr[i] = i * 3; // 0,3,6,...,45
    int idx = binary_search(arr, 0, 15, 21);
    if (idx == 7) { pass++; } else { fail++; printf("FAIL: binary_search(21)=%d expected=7\n", idx); }

    // 5. Binary search — not found
    idx = binary_search(arr, 0, 15, 22);
    if (idx == -1) { pass++; } else { fail++; printf("FAIL: binary_search(22)=%d expected=-1\n", idx); }

    // 6. Mutual recursion: is_even / is_odd
    int mut_ok = is_even(100) && !is_even(99) && is_odd(77) && !is_odd(42);
    if (mut_ok) { pass++; } else { fail++; printf("FAIL: mutual recursion is_even/is_odd\n"); }

    // 7. Deep fibonacci — fib(30) = 832040
    long long f30 = fib(30);
    if (f30 == 832040LL) { pass++; } else { fail++; printf("FAIL: fib(30)=%lld expected=832040\n", f30); }

    // 8. Tree-like recursion — tree_count(10) = 2^11 - 1 = 2047
    int tc = tree_count(10);
    if (tc == 2047) { pass++; } else { fail++; printf("FAIL: tree_count(10)=%d expected=2047\n", tc); }

    printf("Results: %d passed, %d failed\n", pass, fail);
    printf("=== Test 17: %s ===\n", fail == 0 ? "PASS" : "FAIL");
    return fail;
}
