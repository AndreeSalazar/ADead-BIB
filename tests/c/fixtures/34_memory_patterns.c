// ============================================================
// Test 34: Patrones de Memoria Avanzados — pool, arena, ring buffer
// ============================================================
// ADead-BIB Test Canon — Memory management patterns
// Verifica: void* casting, pointer arithmetic, byte-level ops
// ============================================================

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// --- Arena allocator ---
struct Arena {
    char *buffer;
    int capacity;
    int used;
};

struct Arena *arena_create(int size) {
    struct Arena *a = (struct Arena *)malloc(sizeof(struct Arena));
    a->buffer = (char *)malloc(size);
    a->capacity = size;
    a->used = 0;
    return a;
}

void *arena_alloc(struct Arena *a, int size) {
    int aligned = (size + 7) & ~7;
    if (a->used + aligned > a->capacity) return (void *)0;
    void *ptr = a->buffer + a->used;
    a->used += aligned;
    return ptr;
}

void arena_reset(struct Arena *a) {
    a->used = 0;
}

void arena_destroy(struct Arena *a) {
    free(a->buffer);
    free(a);
}

// --- Ring buffer ---
struct RingBuffer {
    int *data;
    int capacity;
    int head;
    int tail;
    int count;
};

struct RingBuffer *ring_create(int capacity) {
    struct RingBuffer *rb = (struct RingBuffer *)malloc(sizeof(struct RingBuffer));
    rb->data = (int *)malloc(sizeof(int) * capacity);
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

int ring_pop(struct RingBuffer *rb, int *value) {
    if (rb->count == 0) return 0;
    *value = rb->data[rb->head];
    rb->head = (rb->head + 1) % rb->capacity;
    rb->count--;
    return 1;
}

void ring_destroy(struct RingBuffer *rb) {
    free(rb->data);
    free(rb);
}

// --- Pool allocator ---
#define POOL_BLOCK_SIZE 32
#define POOL_BLOCKS 16

struct Pool {
    char memory[POOL_BLOCK_SIZE * POOL_BLOCKS];
    int free_list[POOL_BLOCKS];
    int free_count;
};

void pool_init(struct Pool *p) {
    int i;
    for (i = 0; i < POOL_BLOCKS; i++) {
        p->free_list[i] = i;
    }
    p->free_count = POOL_BLOCKS;
}

void *pool_alloc(struct Pool *p) {
    if (p->free_count == 0) return (void *)0;
    p->free_count--;
    int idx = p->free_list[p->free_count];
    return p->memory + idx * POOL_BLOCK_SIZE;
}

void pool_free(struct Pool *p, void *ptr) {
    int offset = (int)((char *)ptr - p->memory);
    int idx = offset / POOL_BLOCK_SIZE;
    p->free_list[p->free_count] = idx;
    p->free_count++;
}

// --- Byte manipulation patterns ---
unsigned int pack_rgba(unsigned char r, unsigned char g, unsigned char b, unsigned char a) {
    return ((unsigned int)a << 24) | ((unsigned int)r << 16) |
           ((unsigned int)g << 8) | (unsigned int)b;
}

void unpack_rgba(unsigned int color, unsigned char *r, unsigned char *g,
                 unsigned char *b, unsigned char *a) {
    *a = (color >> 24) & 0xFF;
    *r = (color >> 16) & 0xFF;
    *g = (color >> 8) & 0xFF;
    *b = color & 0xFF;
}

int main() {
    // --- Arena ---
    struct Arena *arena = arena_create(1024);
    int *a1 = (int *)arena_alloc(arena, sizeof(int) * 10);
    int *a2 = (int *)arena_alloc(arena, sizeof(int) * 5);
    char *a3 = (char *)arena_alloc(arena, 64);

    int i;
    for (i = 0; i < 10; i++) a1[i] = i * i;
    for (i = 0; i < 5; i++) a2[i] = i * 100;
    strcpy(a3, "arena string");

    printf("arena: a1[3]=%d a2[2]=%d a3=%s\n", a1[3], a2[2], a3);
    printf("arena used=%d\n", arena->used);

    arena_reset(arena);
    printf("arena after reset=%d\n", arena->used);
    arena_destroy(arena);

    // --- Ring buffer ---
    struct RingBuffer *rb = ring_create(4);
    ring_push(rb, 10);
    ring_push(rb, 20);
    ring_push(rb, 30);
    ring_push(rb, 40);
    int full = ring_push(rb, 50);
    printf("ring full=%d count=%d\n", !full, rb->count);

    int val;
    ring_pop(rb, &val);
    printf("ring pop=%d count=%d\n", val, rb->count);
    ring_pop(rb, &val);
    printf("ring pop=%d count=%d\n", val, rb->count);

    ring_push(rb, 50);
    ring_push(rb, 60);
    printf("ring count=%d\n", rb->count);

    ring_destroy(rb);

    // --- Pool ---
    struct Pool pool;
    pool_init(&pool);

    void *b1 = pool_alloc(&pool);
    void *b2 = pool_alloc(&pool);
    void *b3 = pool_alloc(&pool);
    printf("pool free=%d\n", pool.free_count);

    strcpy((char *)b1, "block1");
    strcpy((char *)b2, "block2");
    printf("pool: %s %s\n", (char *)b1, (char *)b2);

    pool_free(&pool, b2);
    printf("pool after free=%d\n", pool.free_count);

    void *b4 = pool_alloc(&pool);
    printf("pool reuse=%d\n", pool.free_count);

    // --- RGBA pack/unpack ---
    unsigned int color = pack_rgba(255, 128, 64, 200);
    unsigned char r, g, b, a;
    unpack_rgba(color, &r, &g, &b, &a);
    printf("rgba: r=%d g=%d b=%d a=%d\n", r, g, b, a);
    printf("packed=0x%08X\n", color);

    return 0;
}
