#include <header_main.h>

struct Node {
    int val;
    Node(int v) : val(v) {
        printf("Node(%d) created\n", val);
    }
    ~Node() {
        printf("Node(%d) destroyed\n", val);
    }
};

int main() {
    // new/delete basic int
    int* p = new int(42);
    printf("*p = %d\n", *p);
    delete p;

    // new/delete with class
    Node* n = new Node(10);
    printf("n->val = %d\n", n->val);
    delete n;

    // new[] / delete[]
    int* arr = new int[5];
    for (int i = 0; i < 5; i++) arr[i] = i + 1;
    for (int i = 0; i < 5; i++) printf("%d ", arr[i]);
    printf("\n");
    delete[] arr;

    // linked list with new/delete
    Node* a = new Node(1);
    Node* b = new Node(2);
    Node* c = new Node(3);
    printf("nodes: %d %d %d\n", a->val, b->val, c->val);
    delete c;
    delete b;
    delete a;

    return 0;
}
