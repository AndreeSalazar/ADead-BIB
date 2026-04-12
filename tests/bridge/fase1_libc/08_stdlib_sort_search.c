// ADead-BIB Test: stdlib sort/search (qsort, bsearch, rand, srand)
#include <stdio.h>
#include <stdlib.h>

int compare_ints(const void *a, const void *b) {
    return (*(int*)a) - (*(int*)b);
}

int main() {
    int pass = 0, fail = 0;

    // qsort
    int arr[8];
    arr[0] = 42; arr[1] = 7; arr[2] = 99; arr[3] = 1;
    arr[4] = 55; arr[5] = 23; arr[6] = 88; arr[7] = 3;
    qsort(arr, 8, sizeof(int), compare_ints);
    if (arr[0] == 1 && arr[1] == 3 && arr[7] == 99) {
        pass++; printf("PASS: qsort sorted [%d,%d,%d,...,%d]\n", arr[0], arr[1], arr[2], arr[7]);
    } else {
        fail++; printf("FAIL: qsort [%d,%d,%d,...,%d]\n", arr[0], arr[1], arr[2], arr[7]);
    }

    // bsearch (array must be sorted from qsort above)
    int key = 42;
    int *found = (int*)bsearch(&key, arr, 8, sizeof(int), compare_ints);
    if (found && *found == 42) { pass++; printf("PASS: bsearch found %d\n", *found); }
    else { fail++; printf("FAIL: bsearch for 42\n"); }

    key = 100;
    found = (int*)bsearch(&key, arr, 8, sizeof(int), compare_ints);
    if (found == 0) { pass++; printf("PASS: bsearch not found 100\n"); }
    else { fail++; printf("FAIL: bsearch found 100?\n"); }

    // srand + rand
    srand(12345);
    int r1 = rand();
    int r2 = rand();
    if (r1 != r2) { pass++; printf("PASS: rand() r1=%d r2=%d (different)\n", r1, r2); }
    else { fail++; printf("FAIL: rand() same values\n"); }

    // srand reproducibility
    srand(12345);
    int r3 = rand();
    if (r1 == r3) { pass++; printf("PASS: srand reproducible r1=%d r3=%d\n", r1, r3); }
    else { fail++; printf("FAIL: srand not reproducible r1=%d r3=%d\n", r1, r3); }

    // getenv
    char *path = getenv("PATH");
    if (path) { pass++; printf("PASS: getenv(PATH) len=%d\n", (int)strlen(path)); }
    else { fail++; printf("FAIL: getenv(PATH) null\n"); }

    printf("\n=== stdlib_sort_search: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
