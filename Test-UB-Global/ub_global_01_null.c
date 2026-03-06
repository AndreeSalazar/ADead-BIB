#include <stdio.h>
#include <stdlib.h>

void null_literal() {
    int *ptr = 0;
    *ptr = 42;              // UB-001
}

void null_macro() {
    int *ptr = NULL;
    *ptr = 42;              // UB-002
}

void null_via_cast() {
    int *ptr = (int*)0;
    *ptr = 42;              // UB-003
}

typedef struct { int *data; } Box;
void null_struct_member() {
    Box b;
    b.data = NULL;
    *b.data = 42;           // UB-004
}

int* get_null() { return NULL; }
void null_from_function() {
    int *ptr = get_null();
    *ptr = 42;              // UB-005
}

typedef struct { int x; } Point;
void null_arrow() {
    Point *p = NULL;
    p->x = 10;              // UB-006
}

void null_array_ptr() {
    int *arr[3] = {NULL, NULL, NULL};
    *arr[0] = 42;           // UB-007
}

void null_conditional(int flag) {
    int *ptr = flag ? NULL : NULL;
    *ptr = 42;              // UB-008
}

int main() {
    null_literal();
    return 0;
}
