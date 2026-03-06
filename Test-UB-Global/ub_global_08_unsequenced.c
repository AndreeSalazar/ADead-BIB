#include <stdio.h>

void unsequenced_assignment() {
    int i = 0;
    i = i++ + 1; // UB: Unsequenced modification
    printf("%d\n", i);
}

void unsequenced_array() {
    int arr[10];
    int i = 0;
    arr[i] = i++; // UB: Unsequenced modification
}

void sequenced_safe() {
    int i = 0;
    i = i + 1; // Safe
    int arr[10];
    arr[i] = i; // Safe
    i++;        // Safe
}

int main() {
    // Only warnings will show because we run with --warn-ub
    unsequenced_assignment();
    unsequenced_array();
    sequenced_safe();
    return 0;
}
