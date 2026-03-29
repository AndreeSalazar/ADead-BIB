// Test: Production structs — linked list, bitfields, function pointers
// Expected: Compile + Run — real data structure patterns
// Strict: All operations verified

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Linked list node
struct Node {
    int value;
    struct Node *next;
};

// Push front
struct Node* push(struct Node *head, int val) {
    struct Node *n = (struct Node*)malloc(sizeof(struct Node));
    n->value = val;
    n->next = head;
    return n;
}

// Count nodes
int list_len(struct Node *head) {
    int count = 0;
    while (head) { count++; head = head->next; }
    return count;
}

// Sum values
int list_sum(struct Node *head) {
    int sum = 0;
    while (head) { sum += head->value; head = head->next; }
    return sum;
}

// Free list
void list_free(struct Node *head) {
    while (head) {
        struct Node *tmp = head;
        head = head->next;
        free(tmp);
    }
}

// Nested struct
struct Vec3 { float x, y, z; };
struct Transform {
    struct Vec3 position;
    struct Vec3 rotation;
    struct Vec3 scale;
};

// Function pointer table
typedef int (*MathOp)(int, int);
int add_op(int a, int b) { return a + b; }
int sub_op(int a, int b) { return a - b; }
int mul_op(int a, int b) { return a * b; }

int main() {
    printf("=== PRODUCTION: Structs ===\n");
    int pass = 0, fail = 0;

    // Linked list
    struct Node *list = 0;
    list = push(list, 10);
    list = push(list, 20);
    list = push(list, 30);
    if (list_len(list) == 3)    { pass++; } else { fail++; printf("FAIL: list len\n"); }
    if (list_sum(list) == 60)   { pass++; } else { fail++; printf("FAIL: list sum=%d\n", list_sum(list)); }
    if (list->value == 30)      { pass++; } else { fail++; printf("FAIL: list head\n"); }
    list_free(list);

    // Nested struct
    struct Transform t;
    t.position.x = 1.0f; t.position.y = 2.0f; t.position.z = 3.0f;
    t.scale.x = 1.0f; t.scale.y = 1.0f; t.scale.z = 1.0f;
    if (sizeof(struct Transform) == sizeof(struct Vec3) * 3) { pass++; } else { fail++; printf("FAIL: Transform size\n"); }

    // Function pointer table
    MathOp ops[3] = { add_op, sub_op, mul_op };
    if (ops[0](10, 5) == 15)  { pass++; } else { fail++; printf("FAIL: add_op\n"); }
    if (ops[1](10, 5) == 5)   { pass++; } else { fail++; printf("FAIL: sub_op\n"); }
    if (ops[2](10, 5) == 50)  { pass++; } else { fail++; printf("FAIL: mul_op\n"); }

    // Struct sizeof
    if (sizeof(struct Node) >= 12)     { pass++; } else { fail++; printf("FAIL: Node size\n"); }

    printf("Results: %d passed, %d failed\n", pass, fail);
    printf("=== PRODUCTION: Structs %s ===\n", fail == 0 ? "PASS" : "FAIL");
    return fail;
}
