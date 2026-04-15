// ============================================================
// Test 31: Lista Enlazada Completa — insert, delete, reverse, sort
// ============================================================
// ADead-BIB Test Canon — Data structures con punteros
// Verifica: manejo completo de punteros, malloc/free, struct self-ref
// ============================================================

#include <stdio.h>
#include <stdlib.h>

struct Node {
    int data;
    struct Node *next;
};

struct Node *node_new(int data) {
    struct Node *n = (struct Node *)malloc(sizeof(struct Node));
    n->data = data;
    n->next = (struct Node *)0;
    return n;
}

void list_push_front(struct Node **head, int data) {
    struct Node *n = node_new(data);
    n->next = *head;
    *head = n;
}

void list_push_back(struct Node **head, int data) {
    struct Node *n = node_new(data);
    if (*head == (struct Node *)0) {
        *head = n;
        return;
    }
    struct Node *cur = *head;
    while (cur->next) cur = cur->next;
    cur->next = n;
}

int list_pop_front(struct Node **head) {
    struct Node *n = *head;
    int data = n->data;
    *head = n->next;
    free(n);
    return data;
}

void list_insert_at(struct Node **head, int index, int data) {
    if (index == 0) {
        list_push_front(head, data);
        return;
    }
    struct Node *cur = *head;
    int i;
    for (i = 0; i < index - 1 && cur; i++) {
        cur = cur->next;
    }
    if (cur) {
        struct Node *n = node_new(data);
        n->next = cur->next;
        cur->next = n;
    }
}

int list_delete(struct Node **head, int data) {
    struct Node *prev = (struct Node *)0;
    struct Node *cur = *head;
    while (cur) {
        if (cur->data == data) {
            if (prev) prev->next = cur->next;
            else *head = cur->next;
            free(cur);
            return 1;
        }
        prev = cur;
        cur = cur->next;
    }
    return 0;
}

int list_find(struct Node *head, int data) {
    int index = 0;
    while (head) {
        if (head->data == data) return index;
        head = head->next;
        index++;
    }
    return -1;
}

int list_length(struct Node *head) {
    int len = 0;
    while (head) { len++; head = head->next; }
    return len;
}

void list_reverse(struct Node **head) {
    struct Node *prev = (struct Node *)0;
    struct Node *cur = *head;
    struct Node *next;
    while (cur) {
        next = cur->next;
        cur->next = prev;
        prev = cur;
        cur = next;
    }
    *head = prev;
}

void list_print(struct Node *head) {
    printf("[");
    while (head) {
        printf("%d", head->data);
        if (head->next) printf(", ");
        head = head->next;
    }
    printf("]\n");
}

void list_free(struct Node *head) {
    while (head) {
        struct Node *next = head->next;
        free(head);
        head = next;
    }
}

// --- Merge sort para lista enlazada ---
struct Node *list_merge(struct Node *a, struct Node *b) {
    struct Node dummy;
    struct Node *tail = &dummy;
    dummy.next = (struct Node *)0;
    while (a && b) {
        if (a->data <= b->data) { tail->next = a; a = a->next; }
        else { tail->next = b; b = b->next; }
        tail = tail->next;
    }
    tail->next = a ? a : b;
    return dummy.next;
}

void list_split(struct Node *head, struct Node **front, struct Node **back) {
    struct Node *slow = head;
    struct Node *fast = head->next;
    while (fast) {
        fast = fast->next;
        if (fast) { slow = slow->next; fast = fast->next; }
    }
    *front = head;
    *back = slow->next;
    slow->next = (struct Node *)0;
}

void list_sort(struct Node **head) {
    if (*head == (struct Node *)0 || (*head)->next == (struct Node *)0) return;
    struct Node *a, *b;
    list_split(*head, &a, &b);
    list_sort(&a);
    list_sort(&b);
    *head = list_merge(a, b);
}

int main() {
    struct Node *list = (struct Node *)0;

    // --- Build list ---
    list_push_back(&list, 10);
    list_push_back(&list, 20);
    list_push_back(&list, 30);
    list_push_front(&list, 5);
    printf("built: ");
    list_print(list);
    printf("len=%d\n", list_length(list));

    // --- Insert at ---
    list_insert_at(&list, 2, 15);
    printf("insert@2: ");
    list_print(list);

    // --- Find ---
    printf("find(15)=%d find(99)=%d\n", list_find(list, 15), list_find(list, 99));

    // --- Delete ---
    list_delete(&list, 15);
    printf("delete(15): ");
    list_print(list);

    // --- Pop front ---
    int popped = list_pop_front(&list);
    printf("popped=%d remaining: ", popped);
    list_print(list);

    // --- Reverse ---
    list_reverse(&list);
    printf("reversed: ");
    list_print(list);

    list_free(list);

    // --- Sort ---
    struct Node *unsorted = (struct Node *)0;
    list_push_front(&unsorted, 30);
    list_push_front(&unsorted, 10);
    list_push_front(&unsorted, 50);
    list_push_front(&unsorted, 20);
    list_push_front(&unsorted, 40);
    printf("unsorted: ");
    list_print(unsorted);
    list_sort(&unsorted);
    printf("sorted:   ");
    list_print(unsorted);
    list_free(unsorted);

    return 0;
}
// Expected:
// built: [5, 10, 20, 30]
// len=4
// insert@2: [5, 10, 15, 20, 30]
// find(15)=2 find(99)=-1
// delete(15): [5, 10, 20, 30]
// popped=5 remaining: [10, 20, 30]
// reversed: [30, 20, 10]
// unsorted: [40, 20, 50, 10, 30]
// sorted:   [10, 20, 30, 40, 50]
