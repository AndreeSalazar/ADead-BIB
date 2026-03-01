// ============================================================
// ADead-BIB C Example — Algoritmos Clásicos
// ============================================================
// Sorting, searching, hashing — todo en C puro,
// compilado por ADead-BIB sin dependencias externas.
// ============================================================

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// ==================== Sorting ====================

void bubble_sort(int *arr, int n) {
    for (int i = 0; i < n - 1; i++) {
        for (int j = 0; j < n - i - 1; j++) {
            if (arr[j] > arr[j + 1]) {
                int temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}

void insertion_sort(int *arr, int n) {
    for (int i = 1; i < n; i++) {
        int key = arr[i];
        int j = i - 1;
        while (j >= 0 && arr[j] > key) {
            arr[j + 1] = arr[j];
            j--;
        }
        arr[j + 1] = key;
    }
}

void quicksort_impl(int *arr, int low, int high) {
    if (low >= high) return;
    int pivot = arr[high];
    int i = low - 1;
    for (int j = low; j < high; j++) {
        if (arr[j] <= pivot) {
            i++;
            int temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
        }
    }
    int temp = arr[i + 1];
    arr[i + 1] = arr[high];
    arr[high] = temp;
    int pi = i + 1;
    quicksort_impl(arr, low, pi - 1);
    quicksort_impl(arr, pi + 1, high);
}

void quicksort(int *arr, int n) {
    quicksort_impl(arr, 0, n - 1);
}

// ==================== Searching ====================

int binary_search(int *arr, int n, int target) {
    int low = 0;
    int high = n - 1;
    while (low <= high) {
        int mid = low + (high - low) / 2;
        if (arr[mid] == target) return mid;
        if (arr[mid] < target) low = mid + 1;
        else high = mid - 1;
    }
    return -1;
}

int linear_search(int *arr, int n, int target) {
    for (int i = 0; i < n; i++) {
        if (arr[i] == target) return i;
    }
    return -1;
}

// ==================== Hash Table ====================

struct HashEntry {
    int key;
    int value;
    int occupied;
};

struct HashMap {
    struct HashEntry *entries;
    int capacity;
    int size;
};

unsigned int hash_int(int key, int capacity) {
    unsigned int h = (unsigned int)key;
    h = ((h >> 16) ^ h) * 0x45d9f3b;
    h = ((h >> 16) ^ h) * 0x45d9f3b;
    h = (h >> 16) ^ h;
    return h % capacity;
}

struct HashMap *hashmap_create(int capacity) {
    struct HashMap *map = malloc(sizeof(struct HashMap));
    if (map == NULL) return NULL;
    map->entries = malloc(capacity * sizeof(struct HashEntry));
    map->capacity = capacity;
    map->size = 0;
    for (int i = 0; i < capacity; i++) {
        map->entries[i].occupied = 0;
    }
    return map;
}

void hashmap_put(struct HashMap *map, int key, int value) {
    unsigned int idx = hash_int(key, map->capacity);
    int probes = 0;
    while (map->entries[idx].occupied && map->entries[idx].key != key) {
        idx = (idx + 1) % map->capacity;
        probes++;
        if (probes >= map->capacity) return;
    }
    if (!map->entries[idx].occupied) {
        map->size++;
    }
    map->entries[idx].key = key;
    map->entries[idx].value = value;
    map->entries[idx].occupied = 1;
}

int hashmap_get(struct HashMap *map, int key, int *out_value) {
    unsigned int idx = hash_int(key, map->capacity);
    int probes = 0;
    while (map->entries[idx].occupied) {
        if (map->entries[idx].key == key) {
            *out_value = map->entries[idx].value;
            return 1;
        }
        idx = (idx + 1) % map->capacity;
        probes++;
        if (probes >= map->capacity) break;
    }
    return 0;
}

void hashmap_free(struct HashMap *map) {
    if (map != NULL) {
        free(map->entries);
        free(map);
    }
}

// ==================== Stack ====================

struct Stack {
    int *data;
    int top;
    int capacity;
};

struct Stack *stack_create(int capacity) {
    struct Stack *s = malloc(sizeof(struct Stack));
    if (s == NULL) return NULL;
    s->data = malloc(capacity * sizeof(int));
    s->top = -1;
    s->capacity = capacity;
    return s;
}

int stack_push(struct Stack *s, int value) {
    if (s->top >= s->capacity - 1) return 0;
    s->data[++s->top] = value;
    return 1;
}

int stack_pop(struct Stack *s, int *out) {
    if (s->top < 0) return 0;
    *out = s->data[s->top--];
    return 1;
}

int stack_peek(struct Stack *s) {
    if (s->top < 0) return -1;
    return s->data[s->top];
}

int stack_empty(struct Stack *s) {
    return s->top < 0;
}

void stack_free(struct Stack *s) {
    if (s != NULL) {
        free(s->data);
        free(s);
    }
}

// ==================== Utility ====================

void print_array(const char *label, int *arr, int n) {
    printf("  %s: [", label);
    for (int i = 0; i < n; i++) {
        if (i > 0) printf(", ");
        printf("%d", arr[i]);
    }
    printf("]\n");
}

void copy_array(int *dst, int *src, int n) {
    for (int i = 0; i < n; i++) {
        dst[i] = src[i];
    }
}

// ==================== Main ====================

int main() {
    printf("=== ADead-BIB: Algorithms ===\n\n");

    int data[] = {64, 25, 12, 22, 11, 90, 45, 33, 7, 55};
    int n = 10;
    int sorted[10];

    // Bubble Sort
    copy_array(sorted, data, n);
    bubble_sort(sorted, n);
    printf("Sorting:\n");
    print_array("Original", data, n);
    print_array("Bubble  ", sorted, n);

    // Insertion Sort
    copy_array(sorted, data, n);
    insertion_sort(sorted, n);
    print_array("Insert  ", sorted, n);

    // Quicksort
    copy_array(sorted, data, n);
    quicksort(sorted, n);
    print_array("Quick   ", sorted, n);

    // Binary Search (on sorted array)
    printf("\nSearching (sorted):\n");
    int idx = binary_search(sorted, n, 33);
    printf("  binary_search(33): index %d\n", idx);
    idx = binary_search(sorted, n, 99);
    printf("  binary_search(99): index %d (not found)\n", idx);

    // Linear Search
    idx = linear_search(data, n, 45);
    printf("  linear_search(45): index %d\n", idx);

    // Hash Map
    printf("\nHash Map:\n");
    struct HashMap *map = hashmap_create(32);
    hashmap_put(map, 100, 1000);
    hashmap_put(map, 200, 2000);
    hashmap_put(map, 300, 3000);
    hashmap_put(map, 42, 9001);

    int val;
    if (hashmap_get(map, 42, &val)) {
        printf("  map[42] = %d\n", val);
    }
    if (hashmap_get(map, 200, &val)) {
        printf("  map[200] = %d\n", val);
    }
    if (!hashmap_get(map, 999, &val)) {
        printf("  map[999] = not found\n");
    }
    printf("  Size: %d\n", map->size);
    hashmap_free(map);

    // Stack
    printf("\nStack:\n");
    struct Stack *stack = stack_create(16);
    stack_push(stack, 10);
    stack_push(stack, 20);
    stack_push(stack, 30);
    printf("  Peek: %d\n", stack_peek(stack));

    int popped;
    while (!stack_empty(stack)) {
        stack_pop(stack, &popped);
        printf("  Pop: %d\n", popped);
    }
    stack_free(stack);

    printf("\n=== Complete ===\n");
    return 0;
}
