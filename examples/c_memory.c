// ============================================================
// ADead-BIB C Example — Punteros y Memoria Dinámica
// ============================================================
// Demuestra punteros, malloc/free, arrays dinámicos,
// manejo de memoria estilo musl libc compilado por ADead-BIB.
// ============================================================

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// ==================== Dynamic Array ====================

struct DynArray {
    int *data;
    int size;
    int capacity;
};

struct DynArray *dynarray_create(int initial_cap) {
    struct DynArray *arr = malloc(sizeof(struct DynArray));
    if (arr == NULL) return NULL;
    arr->data = malloc(initial_cap * sizeof(int));
    arr->size = 0;
    arr->capacity = initial_cap;
    return arr;
}

void dynarray_push(struct DynArray *arr, int value) {
    if (arr->size >= arr->capacity) {
        arr->capacity *= 2;
        arr->data = realloc(arr->data, arr->capacity * sizeof(int));
    }
    arr->data[arr->size] = value;
    arr->size++;
}

int dynarray_pop(struct DynArray *arr) {
    if (arr->size <= 0) return -1;
    arr->size--;
    return arr->data[arr->size];
}

int dynarray_get(struct DynArray *arr, int index) {
    if (index < 0 || index >= arr->size) return -1;
    return arr->data[index];
}

void dynarray_set(struct DynArray *arr, int index, int value) {
    if (index >= 0 && index < arr->size) {
        arr->data[index] = value;
    }
}

void dynarray_print(struct DynArray *arr) {
    printf("[");
    for (int i = 0; i < arr->size; i++) {
        if (i > 0) printf(", ");
        printf("%d", arr->data[i]);
    }
    printf("] (size=%d, cap=%d)\n", arr->size, arr->capacity);
}

void dynarray_free(struct DynArray *arr) {
    if (arr != NULL) {
        free(arr->data);
        free(arr);
    }
}

// ==================== Ring Buffer ====================

struct RingBuffer {
    int *data;
    int capacity;
    int head;
    int tail;
    int count;
};

struct RingBuffer *ring_create(int capacity) {
    struct RingBuffer *rb = malloc(sizeof(struct RingBuffer));
    if (rb == NULL) return NULL;
    rb->data = malloc(capacity * sizeof(int));
    rb->capacity = capacity;
    rb->head = 0;
    rb->tail = 0;
    rb->count = 0;
    return rb;
}

int ring_push(struct RingBuffer *rb, int value) {
    if (rb->count >= rb->capacity) return 0;
    rb->data[rb->tail] = value;
    rb->tail = (rb->tail + 1) % rb->capacity;
    rb->count++;
    return 1;
}

int ring_pop(struct RingBuffer *rb, int *out) {
    if (rb->count <= 0) return 0;
    *out = rb->data[rb->head];
    rb->head = (rb->head + 1) % rb->capacity;
    rb->count--;
    return 1;
}

void ring_free(struct RingBuffer *rb) {
    if (rb != NULL) {
        free(rb->data);
        free(rb);
    }
}

// ==================== Memory Pool ====================

struct MemPool {
    char *block;
    int block_size;
    int offset;
};

struct MemPool *pool_create(int size) {
    struct MemPool *pool = malloc(sizeof(struct MemPool));
    if (pool == NULL) return NULL;
    pool->block = malloc(size);
    pool->block_size = size;
    pool->offset = 0;
    memset(pool->block, 0, size);
    return pool;
}

void *pool_alloc(struct MemPool *pool, int size) {
    int aligned = (size + 7) & ~7;
    if (pool->offset + aligned > pool->block_size) return NULL;
    void *ptr = &pool->block[pool->offset];
    pool->offset += aligned;
    return ptr;
}

void pool_reset(struct MemPool *pool) {
    pool->offset = 0;
}

void pool_free(struct MemPool *pool) {
    if (pool != NULL) {
        free(pool->block);
        free(pool);
    }
}

// ==================== Pointer Arithmetic ====================

void fill_pattern(int *buf, int count, int start) {
    int *ptr = buf;
    for (int i = 0; i < count; i++) {
        *ptr = start + i;
        ptr++;
    }
}

int find_value(int *buf, int count, int target) {
    for (int i = 0; i < count; i++) {
        if (buf[i] == target) return i;
    }
    return -1;
}

void swap_bytes(void *a, void *b, int size) {
    char *pa = (char *)a;
    char *pb = (char *)b;
    for (int i = 0; i < size; i++) {
        char temp = pa[i];
        pa[i] = pb[i];
        pb[i] = temp;
    }
}

// ==================== Main ====================

int main() {
    printf("=== ADead-BIB: Pointers & Memory ===\n\n");

    // --- Dynamic Array ---
    printf("Dynamic Array:\n");
    struct DynArray *arr = dynarray_create(4);
    for (int i = 0; i < 10; i++) {
        dynarray_push(arr, i * 10);
    }
    printf("  ");
    dynarray_print(arr);

    dynarray_set(arr, 5, 999);
    printf("  After set[5]=999: ");
    dynarray_print(arr);

    int popped = dynarray_pop(arr);
    printf("  Popped: %d\n", popped);
    printf("  After pop: ");
    dynarray_print(arr);
    dynarray_free(arr);

    // --- Ring Buffer ---
    printf("\nRing Buffer:\n");
    struct RingBuffer *rb = ring_create(4);
    ring_push(rb, 10);
    ring_push(rb, 20);
    ring_push(rb, 30);
    ring_push(rb, 40);
    int overflow = ring_push(rb, 50);
    printf("  Push 50 (overflow): %s\n", overflow ? "ok" : "full");

    int val;
    ring_pop(rb, &val);
    printf("  Pop: %d\n", val);
    ring_pop(rb, &val);
    printf("  Pop: %d\n", val);

    ring_push(rb, 60);
    ring_push(rb, 70);
    printf("  After push 60, 70 — count: %d\n", rb->count);

    while (rb->count > 0) {
        ring_pop(rb, &val);
        printf("  Drain: %d\n", val);
    }
    ring_free(rb);

    // --- Memory Pool ---
    printf("\nMemory Pool:\n");
    struct MemPool *pool = pool_create(1024);
    int *a = pool_alloc(pool, sizeof(int));
    int *b = pool_alloc(pool, sizeof(int));
    int *c = pool_alloc(pool, sizeof(int));
    *a = 111;
    *b = 222;
    *c = 333;
    printf("  Pool allocs: a=%d, b=%d, c=%d\n", *a, *b, *c);
    printf("  Pool used: %d / %d bytes\n", pool->offset, pool->block_size);
    pool_reset(pool);
    printf("  After reset: %d / %d bytes\n", pool->offset, pool->block_size);
    pool_free(pool);

    // --- Pointer arithmetic ---
    printf("\nPointer Arithmetic:\n");
    int buf[10];
    fill_pattern(buf, 10, 100);
    printf("  Pattern: ");
    for (int i = 0; i < 10; i++) {
        printf("%d ", buf[i]);
    }
    printf("\n");
    int idx = find_value(buf, 10, 105);
    printf("  Find 105: index %d\n", idx);

    // --- Swap via void* ---
    int x = 42;
    int y = 99;
    printf("  Before swap: x=%d, y=%d\n", x, y);
    swap_bytes(&x, &y, sizeof(int));
    printf("  After swap:  x=%d, y=%d\n", x, y);

    printf("\n=== Complete ===\n");
    return 0;
}
