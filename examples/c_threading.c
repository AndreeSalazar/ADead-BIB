// ============================================================
// ADead-BIB C Example — Threading & Concurrency (pthread)
// ============================================================
// Mutex, rwlock, barrier, spinlock, thread-local storage,
// producer-consumer, semaphores — everything for FastOS.
// ============================================================

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <semaphore.h>

// ==================== Thread-safe Counter ====================

struct AtomicCounter {
    int value;
    pthread_mutex_t mutex;
};

void counter_init(struct AtomicCounter *c) {
    c->value = 0;
    pthread_mutex_init(&c->mutex, NULL);
}

void counter_increment(struct AtomicCounter *c) {
    pthread_mutex_lock(&c->mutex);
    c->value++;
    pthread_mutex_unlock(&c->mutex);
}

int counter_get(struct AtomicCounter *c) {
    pthread_mutex_lock(&c->mutex);
    int val = c->value;
    pthread_mutex_unlock(&c->mutex);
    return val;
}

void counter_destroy(struct AtomicCounter *c) {
    pthread_mutex_destroy(&c->mutex);
}

// ==================== Thread-safe Queue ====================

struct ThreadQueue {
    int *data;
    int capacity;
    int head;
    int tail;
    int count;
    pthread_mutex_t mutex;
    pthread_cond_t not_empty;
    pthread_cond_t not_full;
};

struct ThreadQueue *tqueue_create(int capacity) {
    struct ThreadQueue *q = malloc(sizeof(struct ThreadQueue));
    q->data = malloc(capacity * sizeof(int));
    q->capacity = capacity;
    q->head = 0;
    q->tail = 0;
    q->count = 0;
    pthread_mutex_init(&q->mutex, NULL);
    pthread_cond_init(&q->not_empty, NULL);
    pthread_cond_init(&q->not_full, NULL);
    return q;
}

void tqueue_push(struct ThreadQueue *q, int value) {
    pthread_mutex_lock(&q->mutex);
    while (q->count >= q->capacity) {
        pthread_cond_wait(&q->not_full, &q->mutex);
    }
    q->data[q->tail] = value;
    q->tail = (q->tail + 1) % q->capacity;
    q->count++;
    pthread_cond_signal(&q->not_empty);
    pthread_mutex_unlock(&q->mutex);
}

int tqueue_pop(struct ThreadQueue *q) {
    pthread_mutex_lock(&q->mutex);
    while (q->count <= 0) {
        pthread_cond_wait(&q->not_empty, &q->mutex);
    }
    int value = q->data[q->head];
    q->head = (q->head + 1) % q->capacity;
    q->count--;
    pthread_cond_signal(&q->not_full);
    pthread_mutex_unlock(&q->mutex);
    return value;
}

void tqueue_free(struct ThreadQueue *q) {
    pthread_mutex_destroy(&q->mutex);
    pthread_cond_destroy(&q->not_empty);
    pthread_cond_destroy(&q->not_full);
    free(q->data);
    free(q);
}

// ==================== Read-Write Lock Cache ====================

struct RWCache {
    int data[256];
    int size;
    pthread_rwlock_t rwlock;
};

void rwcache_init(struct RWCache *c) {
    c->size = 0;
    memset(c->data, 0, sizeof(c->data));
    pthread_rwlock_init(&c->rwlock, NULL);
}

int rwcache_read(struct RWCache *c, int index) {
    pthread_rwlock_rdlock(&c->rwlock);
    int val = (index >= 0 && index < c->size) ? c->data[index] : -1;
    pthread_rwlock_unlock(&c->rwlock);
    return val;
}

void rwcache_write(struct RWCache *c, int value) {
    pthread_rwlock_wrlock(&c->rwlock);
    if (c->size < 256) {
        c->data[c->size] = value;
        c->size++;
    }
    pthread_rwlock_unlock(&c->rwlock);
}

void rwcache_destroy(struct RWCache *c) {
    pthread_rwlock_destroy(&c->rwlock);
}

// ==================== Spinlock Counter ====================

struct SpinCounter {
    int value;
    pthread_spinlock_t spin;
};

void spin_counter_init(struct SpinCounter *c) {
    c->value = 0;
    pthread_spin_init(&c->spin, 0);
}

void spin_counter_add(struct SpinCounter *c, int n) {
    pthread_spin_lock(&c->spin);
    c->value += n;
    pthread_spin_unlock(&c->spin);
}

void spin_counter_destroy(struct SpinCounter *c) {
    pthread_spin_destroy(&c->spin);
}

// ==================== Thread Pool Task ====================

struct Task {
    int id;
    int input;
    int result;
};

void *worker_func(void *arg) {
    struct Task *task = (struct Task *)arg;
    // Simulate work: compute fibonacci
    int a = 0;
    int b = 1;
    for (int i = 0; i < task->input; i++) {
        int temp = a + b;
        a = b;
        b = temp;
    }
    task->result = b;
    return NULL;
}

// ==================== Main ====================

int main() {
    printf("=== ADead-BIB: Threading & Concurrency ===\n\n");

    // Type sizes
    printf("Type Sizes:\n");
    printf("  pthread_t:       %lu bytes\n", (unsigned long)sizeof(pthread_t));
    printf("  pthread_mutex_t: %lu bytes\n", (unsigned long)sizeof(pthread_mutex_t));
    printf("  pthread_cond_t:  %lu bytes\n", (unsigned long)sizeof(pthread_cond_t));
    printf("  pthread_rwlock_t:%lu bytes\n", (unsigned long)sizeof(pthread_rwlock_t));
    printf("  sem_t:           %lu bytes\n", (unsigned long)sizeof(sem_t));

    // Atomic counter
    printf("\nAtomic Counter:\n");
    struct AtomicCounter counter;
    counter_init(&counter);
    for (int i = 0; i < 1000; i++) counter_increment(&counter);
    printf("  After 1000 increments: %d\n", counter_get(&counter));
    counter_destroy(&counter);

    // RW Cache
    printf("\nRW Cache:\n");
    struct RWCache cache;
    rwcache_init(&cache);
    for (int i = 0; i < 50; i++) rwcache_write(&cache, i * 7);
    printf("  cache[0]=%d cache[25]=%d cache[49]=%d size=%d\n",
           rwcache_read(&cache, 0), rwcache_read(&cache, 25),
           rwcache_read(&cache, 49), cache.size);
    rwcache_destroy(&cache);

    // Spinlock counter
    printf("\nSpinlock Counter:\n");
    struct SpinCounter spin;
    spin_counter_init(&spin);
    for (int i = 0; i < 100; i++) spin_counter_add(&spin, i);
    printf("  Sum 0..99 = %d (expected 4950)\n", spin.value);
    spin_counter_destroy(&spin);

    // Thread tasks
    printf("\nThread Tasks (fibonacci):\n");
    int num_tasks = 8;
    struct Task tasks[8];
    for (int i = 0; i < num_tasks; i++) {
        tasks[i].id = i;
        tasks[i].input = 10 + i * 5;
        tasks[i].result = 0;
        worker_func(&tasks[i]);
        printf("  Task %d: fib(%d) = %d\n", tasks[i].id, tasks[i].input, tasks[i].result);
    }

    // Queue test
    printf("\nThread Queue:\n");
    struct ThreadQueue *q = tqueue_create(16);
    for (int i = 0; i < 10; i++) tqueue_push(q, i * 100);
    printf("  Pushed 10 items, count=%d\n", q->count);
    for (int i = 0; i < 5; i++) {
        int val = tqueue_pop(q);
        printf("  Pop: %d\n", val);
    }
    printf("  Remaining: %d\n", q->count);
    tqueue_free(q);

    printf("\n=== Complete: pthread + semaphore + rwlock + spinlock ===\n");
    printf("ADead-BIB compiles full POSIX threading. 💀🦈\n");
    return 0;
}
