// ============================================================
// Test 32: Árbol Binario de Búsqueda — insert, search, traversal
// ============================================================
// ADead-BIB Test Canon — BST con punteros recursivos
// Verifica: recursión + punteros + malloc/free + struct self-ref
// ============================================================

#include <stdio.h>
#include <stdlib.h>

struct TreeNode {
    int key;
    struct TreeNode *left;
    struct TreeNode *right;
};

struct TreeNode *tree_new(int key) {
    struct TreeNode *n = (struct TreeNode *)malloc(sizeof(struct TreeNode));
    n->key = key;
    n->left = (struct TreeNode *)0;
    n->right = (struct TreeNode *)0;
    return n;
}

struct TreeNode *tree_insert(struct TreeNode *root, int key) {
    if (root == (struct TreeNode *)0) return tree_new(key);
    if (key < root->key) root->left = tree_insert(root->left, key);
    else if (key > root->key) root->right = tree_insert(root->right, key);
    return root;
}

int tree_search(struct TreeNode *root, int key) {
    if (root == (struct TreeNode *)0) return 0;
    if (key == root->key) return 1;
    if (key < root->key) return tree_search(root->left, key);
    return tree_search(root->right, key);
}

int tree_min(struct TreeNode *root) {
    while (root->left) root = root->left;
    return root->key;
}

int tree_max(struct TreeNode *root) {
    while (root->right) root = root->right;
    return root->key;
}

int tree_height(struct TreeNode *root) {
    if (root == (struct TreeNode *)0) return 0;
    int lh = tree_height(root->left);
    int rh = tree_height(root->right);
    return 1 + ((lh > rh) ? lh : rh);
}

int tree_count(struct TreeNode *root) {
    if (root == (struct TreeNode *)0) return 0;
    return 1 + tree_count(root->left) + tree_count(root->right);
}

void tree_inorder(struct TreeNode *root) {
    if (root == (struct TreeNode *)0) return;
    tree_inorder(root->left);
    printf("%d ", root->key);
    tree_inorder(root->right);
}

void tree_preorder(struct TreeNode *root) {
    if (root == (struct TreeNode *)0) return;
    printf("%d ", root->key);
    tree_preorder(root->left);
    tree_preorder(root->right);
}

void tree_postorder(struct TreeNode *root) {
    if (root == (struct TreeNode *)0) return;
    tree_postorder(root->left);
    tree_postorder(root->right);
    printf("%d ", root->key);
}

void tree_free(struct TreeNode *root) {
    if (root == (struct TreeNode *)0) return;
    tree_free(root->left);
    tree_free(root->right);
    free(root);
}

int main() {
    struct TreeNode *root = (struct TreeNode *)0;

    // --- Insert ---
    root = tree_insert(root, 50);
    root = tree_insert(root, 30);
    root = tree_insert(root, 70);
    root = tree_insert(root, 20);
    root = tree_insert(root, 40);
    root = tree_insert(root, 60);
    root = tree_insert(root, 80);

    // --- Traversals ---
    printf("inorder:   ");
    tree_inorder(root);
    printf("\n");

    printf("preorder:  ");
    tree_preorder(root);
    printf("\n");

    printf("postorder: ");
    tree_postorder(root);
    printf("\n");

    // --- Search ---
    printf("search(40)=%d search(99)=%d\n",
           tree_search(root, 40), tree_search(root, 99));

    // --- Min/Max ---
    printf("min=%d max=%d\n", tree_min(root), tree_max(root));

    // --- Height/Count ---
    printf("height=%d count=%d\n", tree_height(root), tree_count(root));

    tree_free(root);

    // --- Degenerate tree (linked list) ---
    struct TreeNode *degen = (struct TreeNode *)0;
    int i;
    for (i = 1; i <= 5; i++) {
        degen = tree_insert(degen, i);
    }
    printf("degen height=%d count=%d\n", tree_height(degen), tree_count(degen));
    printf("degen inorder: ");
    tree_inorder(degen);
    printf("\n");
    tree_free(degen);

    return 0;
}
// Expected:
// inorder:   20 30 40 50 60 70 80
// preorder:  50 30 20 40 70 60 80
// postorder: 20 40 30 60 80 70 50
// search(40)=1 search(99)=0
// min=20 max=80
// height=3 count=7
// degen height=5 count=5
// degen inorder: 1 2 3 4 5
