// ADead-BIB Bridge Test 21 — Linked List
// Level: INTERMEDIATE
// Tests: Singly linked list with malloc, pointer chasing, insert, delete, search

#include <stdio.h>
#include <stdlib.h>

struct Node {
    int data;
    struct Node* next;
};

struct Node* list_push(struct Node* head, int data) {
    struct Node* n = (struct Node*)malloc(sizeof(struct Node));
    n->data = data;
    n->next = head;
    return n;
}

int list_pop(struct Node** head) {
    if (*head == NULL) return -1;
    struct Node* top = *head;
    int val = top->data;
    *head = top->next;
    free(top);
    return val;
}

struct Node* list_find(struct Node* head, int data) {
    while (head) {
        if (head->data == data) return head;
        head = head->next;
    }
    return NULL;
}

int list_length(struct Node* head) {
    int len = 0;
    while (head) { len++; head = head->next; }
    return len;
}

struct Node* list_reverse(struct Node* head) {
    struct Node* prev = NULL;
    struct Node* curr = head;
    while (curr) {
        struct Node* next = curr->next;
        curr->next = prev;
        prev = curr;
        curr = next;
    }
    return prev;
}

void list_free(struct Node* head) {
    while (head) {
        struct Node* next = head->next;
        free(head);
        head = next;
    }
}

int main() {
    printf("=== ADead-BIB Bridge Test 21: Linked List ===\n");
    int pass = 0, fail = 0;

    // Build a list of 10 elements: push 0..9
    struct Node* list = NULL;
    for (int i = 0; i < 10; i++) {
        list = list_push(list, i);
    }

    // Verify length == 10
    if (list_length(list) == 10) { pass++; } else { fail++; printf("FAIL: length=%d\n", list_length(list)); }

    // Head should be 9 (last pushed)
    if (list->data == 9) { pass++; } else { fail++; printf("FAIL: head=%d\n", list->data); }

    // Find element 5
    struct Node* found = list_find(list, 5);
    if (found != NULL && found->data == 5) { pass++; } else { fail++; printf("FAIL: find 5\n"); }

    // Find element that doesn't exist
    if (list_find(list, 99) == NULL) { pass++; } else { fail++; printf("FAIL: find 99 should be NULL\n"); }

    // Reverse the list: order should become 0..9
    list = list_reverse(list);
    if (list->data == 0) { pass++; } else { fail++; printf("FAIL: reverse head=%d\n", list->data); }

    // Walk reversed list, verify ascending order
    int ordered = 1;
    struct Node* cur = list;
    for (int i = 0; i < 10 && cur; i++, cur = cur->next) {
        if (cur->data != i) { ordered = 0; break; }
    }
    if (ordered) { pass++; } else { fail++; printf("FAIL: reverse order\n"); }

    // Pop first element (should be 0)
    int val = list_pop(&list);
    if (val == 0) { pass++; } else { fail++; printf("FAIL: pop=%d\n", val); }

    // Length should now be 9
    if (list_length(list) == 9) { pass++; } else { fail++; printf("FAIL: length after pop=%d\n", list_length(list)); }

    // Pop next (should be 1)
    val = list_pop(&list);
    if (val == 1) { pass++; } else { fail++; printf("FAIL: pop2=%d\n", val); }

    // Pop all remaining, verify last pop is 9
    int last = -1;
    while (list) {
        last = list_pop(&list);
    }
    if (last == 9) { pass++; } else { fail++; printf("FAIL: last pop=%d\n", last); }

    // List should be empty
    if (list == NULL && list_length(list) == 0) { pass++; } else { fail++; printf("FAIL: list not empty\n"); }

    // Rebuild and free (no crash)
    for (int i = 0; i < 5; i++) list = list_push(list, i * 10);
    list_free(list);
    list = NULL;
    pass++;

    printf("Results: %d passed, %d failed\n", pass, fail);
    printf("=== Test 21: %s ===\n", fail == 0 ? "PASS" : "FAIL");
    return fail;
}
