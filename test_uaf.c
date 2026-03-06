struct Node {
    int data;
    struct Node *next;
};
void list_free(struct Node *head) {
    while (head != 0) {
        struct Node *next = head->next;
        free(head);
        head = next;
    }
}
int main() {
    struct Node* n = 0;
    list_free(n);
    return 0;
}
