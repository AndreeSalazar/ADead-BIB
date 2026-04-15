// ============================================================
// Test 30: Algoritmos Clásicos — sort, search, hash, data structures
// ============================================================
// ADead-BIB Test Canon — Algoritmos fundamentales en C
// Verifica: implementaciones correctas con punteros/arrays/structs
// ============================================================

#include <stdio.h>
#include <stdlib.h>

// --- Binary search ---
int binary_search(int *arr, int n, int target) {
    int lo = 0, hi = n - 1;
    while (lo <= hi) {
        int mid = lo + (hi - lo) / 2;
        if (arr[mid] == target) return mid;
        if (arr[mid] < target) lo = mid + 1;
        else hi = mid - 1;
    }
    return -1;
}

// --- Insertion sort ---
void insertion_sort(int *arr, int n) {
    int i, j;
    for (i = 1; i < n; i++) {
        int key = arr[i];
        j = i - 1;
        while (j >= 0 && arr[j] > key) {
            arr[j + 1] = arr[j];
            j--;
        }
        arr[j + 1] = key;
    }
}

// --- Quick sort partition ---
int partition(int *arr, int lo, int hi) {
    int pivot = arr[hi];
    int i = lo - 1;
    int j;
    for (j = lo; j < hi; j++) {
        if (arr[j] <= pivot) {
            i++;
            int tmp = arr[i];
            arr[i] = arr[j];
            arr[j] = tmp;
        }
    }
    int tmp = arr[i + 1];
    arr[i + 1] = arr[hi];
    arr[hi] = tmp;
    return i + 1;
}

void quicksort(int *arr, int lo, int hi) {
    if (lo < hi) {
        int pi = partition(arr, lo, hi);
        quicksort(arr, lo, pi - 1);
        quicksort(arr, pi + 1, hi);
    }
}

// --- Stack (array-based) ---
struct Stack {
    int data[100];
    int top;
};

void stack_init(struct Stack *s) { s->top = -1; }
void stack_push(struct Stack *s, int val) { s->data[++s->top] = val; }
int stack_pop(struct Stack *s) { return s->data[s->top--]; }
int stack_peek(struct Stack *s) { return s->data[s->top]; }
int stack_empty(struct Stack *s) { return s->top == -1; }

// --- Hash table (open addressing, simple) ---
#define HT_SIZE 64

struct HashTable {
    int keys[HT_SIZE];
    int values[HT_SIZE];
    int used[HT_SIZE];
};

void ht_init(struct HashTable *ht) {
    int i;
    for (i = 0; i < HT_SIZE; i++) {
        ht->used[i] = 0;
    }
}

unsigned int ht_hash(int key) {
    unsigned int k = (unsigned int)key;
    k = ((k >> 16) ^ k) * 0x45d9f3b;
    k = ((k >> 16) ^ k) * 0x45d9f3b;
    k = (k >> 16) ^ k;
    return k % HT_SIZE;
}

void ht_set(struct HashTable *ht, int key, int value) {
    unsigned int idx = ht_hash(key);
    while (ht->used[idx] && ht->keys[idx] != key) {
        idx = (idx + 1) % HT_SIZE;
    }
    ht->keys[idx] = key;
    ht->values[idx] = value;
    ht->used[idx] = 1;
}

int ht_get(struct HashTable *ht, int key, int *found) {
    unsigned int idx = ht_hash(key);
    int start = idx;
    while (ht->used[idx]) {
        if (ht->keys[idx] == key) {
            *found = 1;
            return ht->values[idx];
        }
        idx = (idx + 1) % HT_SIZE;
        if (idx == start) break;
    }
    *found = 0;
    return 0;
}

// --- GCD / LCM ---
int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

int lcm(int a, int b) {
    return a / gcd(a, b) * b;
}

// --- Is prime ---
int is_prime(int n) {
    if (n < 2) return 0;
    if (n < 4) return 1;
    if (n % 2 == 0 || n % 3 == 0) return 0;
    int i;
    for (i = 5; i * i <= n; i += 6) {
        if (n % i == 0 || n % (i + 2) == 0) return 0;
    }
    return 1;
}

int main() {
    // --- Binary search ---
    int sorted[] = {2, 5, 8, 12, 16, 23, 38, 56, 72, 91};
    printf("bsearch(23)=%d\n", binary_search(sorted, 10, 23));
    printf("bsearch(99)=%d\n", binary_search(sorted, 10, 99));

    // --- Insertion sort ---
    int data1[] = {64, 25, 12, 22, 11};
    insertion_sort(data1, 5);
    printf("isort: %d %d %d %d %d\n", data1[0], data1[1], data1[2], data1[3], data1[4]);

    // --- Quicksort ---
    int data2[] = {10, 80, 30, 90, 40, 50, 70};
    quicksort(data2, 0, 6);
    printf("qsort: %d %d %d %d %d %d %d\n",
           data2[0], data2[1], data2[2], data2[3], data2[4], data2[5], data2[6]);

    // --- Stack ---
    struct Stack s;
    stack_init(&s);
    stack_push(&s, 10);
    stack_push(&s, 20);
    stack_push(&s, 30);
    printf("stack peek=%d\n", stack_peek(&s));
    printf("stack pop=%d\n", stack_pop(&s));
    printf("stack pop=%d\n", stack_pop(&s));
    printf("stack empty=%d\n", stack_empty(&s));

    // --- Hash table ---
    struct HashTable ht;
    ht_init(&ht);
    ht_set(&ht, 42, 100);
    ht_set(&ht, 17, 200);
    ht_set(&ht, 99, 300);

    int found;
    int v = ht_get(&ht, 17, &found);
    printf("ht[17]=%d found=%d\n", v, found);
    v = ht_get(&ht, 999, &found);
    printf("ht[999] found=%d\n", found);

    // --- GCD / LCM ---
    printf("gcd(48,18)=%d\n", gcd(48, 18));
    printf("lcm(12,18)=%d\n", lcm(12, 18));

    // --- Primes ---
    printf("primes: ");
    int i;
    for (i = 2; i < 30; i++) {
        if (is_prime(i)) printf("%d ", i);
    }
    printf("\n");

    return 0;
}
// Expected:
// bsearch(23)=5
// bsearch(99)=-1
// isort: 11 12 22 25 64
// qsort: 10 30 40 50 70 80 90
// stack peek=30
// gcd(48,18)=6
// lcm(12,18)=36
// primes: 2 3 5 7 11 13 17 19 23 29
