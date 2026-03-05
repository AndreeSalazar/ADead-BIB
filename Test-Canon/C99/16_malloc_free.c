// ============================================================
// Canon C99 — §7.20.3 malloc/free — Memoria Dinámica
// ============================================================
// Intención: malloc() pide memoria al sistema. free() la
// devuelve. El programador es 100% responsable.
// No hay garbage collector, no hay RAII, no hay smart ptrs.
//
// C99 §7.20.3.3: "The malloc function allocates space for
// an object whose size is specified by size."
// C99 §7.20.3.2: "The free function causes the space pointed
// to by ptr to be deallocated."
// ============================================================

#include <stdio.h>
#include <stdlib.h>

// --- Linked list con malloc/free ---
struct Node {
    int data;
    struct Node *next;
};

struct Node *node_new(int value) {
    struct Node *n = malloc(sizeof(struct Node));
    if (n != 0) {
        n->data = value;
        n->next = 0;
    }
    return n;
}

void list_push(struct Node **head, int value) {
    struct Node *n = node_new(value);
    n->next = *head;
    *head = n;
}

int list_pop(struct Node **head) {
    if (*head == 0) return -1;
    struct Node *top = *head;
    int value = top->data;
    *head = top->next;
    free(top);
    return value;
}

int list_length(struct Node *head) {
    int len = 0;
    while (head != 0) {
        len++;
        head = head->next;
    }
    return len;
}

int list_sum(struct Node *head) {
    int total = 0;
    while (head != 0) {
        total = total + head->data;
        head = head->next;
    }
    return total;
}

void list_print(struct Node *head) {
    printf("  [");
    while (head != 0) {
        printf("%d", head->data);
        if (head->next != 0) printf(" → ");
        head = head->next;
    }
    printf("]\n");
}

void list_free(struct Node *head) {
    while (head != 0) {
        struct Node *next = head->next;
        free(head);
        head = next;
    }
}

// --- Dynamic array ---
int *array_create(int size) {
    int *arr = malloc(size * sizeof(int));
    int i;
    for (i = 0; i < size; i++) {
        arr[i] = 0;
    }
    return arr;
}

void array_fill(int *arr, int size, int value) {
    int i;
    for (i = 0; i < size; i++) {
        arr[i] = value;
    }
}

int array_sum(int *arr, int size) {
    int total = 0;
    int i;
    for (i = 0; i < size; i++) {
        total = total + arr[i];
    }
    return total;
}

int main() {
    printf("=== Canon C99: malloc/free ===\n\n");

    // --- Linked list ---
    printf("Linked List:\n");
    struct Node *list = 0;

    list_push(&list, 10);
    list_push(&list, 20);
    list_push(&list, 30);
    list_push(&list, 40);
    list_push(&list, 50);

    list_print(list);
    printf("  length = %d\n", list_length(list));
    printf("  sum = %d\n", list_sum(list));

    // --- Pop ---
    int popped = list_pop(&list);
    printf("  pop = %d\n", popped);
    printf("  length after pop = %d\n", list_length(list));
    list_print(list);

    // --- Free all ---
    list_free(list);
    printf("  freed OK\n");

    // --- Dynamic array ---
    printf("\nDynamic Array:\n");
    int size = 10;
    int *arr = array_create(size);
    printf("  created size=%d\n", size);

    int i;
    for (i = 0; i < size; i++) {
        arr[i] = (i + 1) * (i + 1);
    }

    printf("  contents: ");
    for (i = 0; i < size; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");

    printf("  sum = %d\n", array_sum(arr, size));

    free(arr);
    printf("  freed OK\n");

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (popped == 50) { pass++; } else { printf("FAIL: pop\n"); }

    struct Node *test_list = 0;
    list_push(&test_list, 1);
    list_push(&test_list, 2);
    list_push(&test_list, 3);
    total++; if (list_length(test_list) == 3)  { pass++; } else { printf("FAIL: length\n"); }
    total++; if (list_sum(test_list) == 6)     { pass++; } else { printf("FAIL: sum\n"); }
    total++; if (list_pop(&test_list) == 3)    { pass++; } else { printf("FAIL: pop2\n"); }
    list_free(test_list);

    int *test_arr = array_create(5);
    array_fill(test_arr, 5, 7);
    total++; if (array_sum(test_arr, 5) == 35) { pass++; } else { printf("FAIL: array sum\n"); }
    free(test_arr);

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
