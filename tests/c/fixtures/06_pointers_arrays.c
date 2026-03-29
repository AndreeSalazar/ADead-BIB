// Test: Pointers and arrays — pointer arithmetic, arrays, multi-dim, string ops
// Expected: All parse + lower correctly

int sum_array(int *arr, int n) {
    int sum = 0;
    int i;
    for (i = 0; i < n; i++) {
        sum += arr[i];
    }
    return sum;
}

int pointer_arithmetic() {
    int arr[5] = {10, 20, 30, 40, 50};
    int *p = arr;
    int val = *(p + 2);
    return val;
}

void swap(int *a, int *b) {
    int tmp = *a;
    *a = *b;
    *b = tmp;
}

int matrix_trace() {
    int mat[3][3] = {{1,0,0}, {0,2,0}, {0,0,3}};
    return mat[0][0] + mat[1][1] + mat[2][2];
}

int string_length(const char *s) {
    int len = 0;
    while (s[len] != '\0') {
        len++;
    }
    return len;
}

int main() {
    int data[4] = {1, 2, 3, 4};
    int s = sum_array(data, 4);
    int p = pointer_arithmetic();
    int a = 10, b = 20;
    swap(&a, &b);
    int t = matrix_trace();
    const char *msg = "hello";
    int len = string_length(msg);
    return 0;
}
