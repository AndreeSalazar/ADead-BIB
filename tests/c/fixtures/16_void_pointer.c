// ============================================================
// Test 16: Void Pointer — casting genérico, contenedores, type erasure
// ============================================================
// ADead-BIB Test Canon — C99 §6.3.2.3
// Verifica: void* cast, generic swap, generic array, opaque handles
// ============================================================

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// --- Generic swap ---
void generic_swap(void *a, void *b, int size) {
    char *ca = (char *)a;
    char *cb = (char *)b;
    int i;
    for (i = 0; i < size; i++) {
        char tmp = ca[i];
        ca[i] = cb[i];
        cb[i] = tmp;
    }
}

// --- Generic print (tipo tag) ---
enum TypeTag { TYPE_INT, TYPE_FLOAT, TYPE_STRING };

struct TaggedValue {
    enum TypeTag type;
    void *data;
};

void print_tagged(struct TaggedValue *v) {
    switch (v->type) {
        case TYPE_INT:
            printf("int:%d", *(int *)v->data);
            break;
        case TYPE_FLOAT:
            printf("float:%.2f", *(float *)v->data);
            break;
        case TYPE_STRING:
            printf("str:%s", (char *)v->data);
            break;
    }
}

// --- Generic min ---
void *generic_min(void *arr, int count, int elem_size,
                  int (*cmp)(const void *, const void *)) {
    char *base = (char *)arr;
    char *min_elem = base;
    int i;
    for (i = 1; i < count; i++) {
        char *current = base + i * elem_size;
        if (cmp(current, min_elem) < 0) {
            min_elem = current;
        }
    }
    return min_elem;
}

int int_compare(const void *a, const void *b) {
    return *(const int *)a - *(const int *)b;
}

// --- Opaque handle pattern ---
typedef void *Handle;

struct InternalState {
    int id;
    int counter;
};

Handle create_handle(int id) {
    struct InternalState *s = (struct InternalState *)malloc(sizeof(struct InternalState));
    s->id = id;
    s->counter = 0;
    return (Handle)s;
}

int handle_increment(Handle h) {
    struct InternalState *s = (struct InternalState *)h;
    s->counter++;
    return s->counter;
}

int handle_get_id(Handle h) {
    struct InternalState *s = (struct InternalState *)h;
    return s->id;
}

void handle_destroy(Handle h) {
    free(h);
}

int main() {
    // --- void* cast básico ---
    int x = 42;
    void *vp = &x;
    int *ip = (int *)vp;
    printf("void* cast: %d\n", *ip);

    // --- void* con diferentes tipos ---
    float f = 3.14f;
    void *fvp = &f;
    float *fp = (float *)fvp;
    printf("float via void*: %.2f\n", *fp);

    // --- Generic swap int ---
    int a = 10, b = 20;
    generic_swap(&a, &b, sizeof(int));
    printf("swap int: a=%d b=%d\n", a, b);

    // --- Generic swap float ---
    float fa = 1.5f, fb = 2.5f;
    generic_swap(&fa, &fb, sizeof(float));
    printf("swap float: a=%.1f b=%.1f\n", fa, fb);

    // --- Tagged values ---
    int ival = 42;
    float fval = 3.14f;
    char sval[] = "hello";

    struct TaggedValue values[3];
    values[0].type = TYPE_INT;    values[0].data = &ival;
    values[1].type = TYPE_FLOAT;  values[1].data = &fval;
    values[2].type = TYPE_STRING; values[2].data = sval;

    int i;
    for (i = 0; i < 3; i++) {
        print_tagged(&values[i]);
        printf(" ");
    }
    printf("\n");

    // --- Generic min ---
    int nums[] = {5, 3, 8, 1, 9, 2};
    int *min = (int *)generic_min(nums, 6, sizeof(int), int_compare);
    printf("generic_min=%d\n", *min);

    // --- Opaque handle ---
    Handle h = create_handle(42);
    printf("handle id=%d\n", handle_get_id(h));
    printf("counter=%d\n", handle_increment(h));
    printf("counter=%d\n", handle_increment(h));
    printf("counter=%d\n", handle_increment(h));
    handle_destroy(h);

    // --- malloc retorna void* ---
    int *heap = (int *)malloc(sizeof(int) * 5);
    for (i = 0; i < 5; i++) {
        heap[i] = (i + 1) * 100;
    }
    printf("heap: %d %d %d %d %d\n", heap[0], heap[1], heap[2], heap[3], heap[4]);
    free(heap);

    return 0;
}
