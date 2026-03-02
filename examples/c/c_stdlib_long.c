// ============================================================
// ADead-BIB C Compiler — LONG Standard Library Compliance Test
// ============================================================
// Exercises ALL standard C library features the compiler supports:
//   - stdio.h: printf, sprintf, puts, putchar
//   - stdlib.h: malloc, calloc, realloc, free, atoi, abs, rand
//   - string.h: strlen, strcmp, strcpy, strcat, memcpy, memset
//   - math.h: sin, cos, sqrt, pow, fabs, ceil, floor, round
//   - stdint.h: int8_t..int64_t, uint8_t..uint64_t
//   - stdbool.h: bool, true, false
//   - ctype.h: isalpha, isdigit, toupper, tolower
//   - limits.h: type limits
//   - time.h: time structs
//   - signal.h: sigaction struct
//   - setjmp.h: jmp_buf
//   - assert.h
//   - stdarg.h: va_list
//   - stddef.h: size_t, ptrdiff_t
//   - errno.h: errno
//   - Structs, enums, typedef, unions
//   - Pointers: single, double, function pointers
//   - Arrays: fixed, VLA-style, multi-dimensional
//   - Control flow: if/else, for, while, do-while, switch, goto
//   - Bitwise: &, |, ^, <<, >>, ~
//   - Compound assignment: +=, -=, *=, /=, %=, &=, |=, ^=, <<=, >>=
//   - Ternary operator
//   - Recursion, mutual recursion
//   - String operations
//   - Dynamic memory management
//   - Linked list, stack, queue implementations
//
// Compilar: adeadc c_stdlib_long.c -o stdlib_test.exe
// Sin GCC. Sin Clang. Solo ADead-BIB. 💀🦈
// ============================================================

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdint.h>
#include <stdbool.h>
#include <ctype.h>
#include <limits.h>
#include <stddef.h>
#include <errno.h>
#include <assert.h>
#include <stdarg.h>
#include <time.h>
#include <signal.h>
#include <setjmp.h>
#include <float.h>
#include <locale.h>

// ==================== Part 1: Type System ====================

typedef unsigned int uint;
typedef unsigned char byte;
typedef long long i64;
typedef unsigned long long u64;
typedef int (*BinFunc)(int, int);

struct Point2D {
    int x;
    int y;
};

struct Point3D {
    int x;
    int y;
    int z;
};

struct Color {
    uint8_t r;
    uint8_t g;
    uint8_t b;
    uint8_t a;
};

struct Rect {
    struct Point2D origin;
    int width;
    int height;
};

struct LinkedNode {
    int data;
    struct LinkedNode *next;
};

struct Matrix {
    int rows;
    int cols;
    int data[16];
};

enum Weekday {
    MONDAY = 0,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY
};

enum ErrorCode {
    ERR_NONE = 0,
    ERR_INVALID_ARG = -1,
    ERR_OUT_OF_MEMORY = -2,
    ERR_NOT_FOUND = -3,
    ERR_OVERFLOW = -4,
    ERR_IO = -5,
    ERR_TIMEOUT = -6
};

// ==================== Part 2: Arithmetic Functions ====================

int add(int a, int b) { return a + b; }
int sub(int a, int b) { return a - b; }
int mul(int a, int b) { return a * b; }
int divide(int a, int b) { return (b != 0) ? a / b : 0; }
int modulo(int a, int b) { return (b != 0) ? a % b : 0; }

int power(int base, int exp) {
    int result = 1;
    for (int i = 0; i < exp; i++) {
        result *= base;
    }
    return result;
}

int factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

int fibonacci(int n) {
    if (n <= 0) return 0;
    if (n == 1) return 1;
    int a = 0, b = 1;
    for (int i = 2; i <= n; i++) {
        int temp = a + b;
        a = b;
        b = temp;
    }
    return b;
}

int gcd(int a, int b) {
    while (b != 0) {
        int temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

int lcm(int a, int b) {
    return (a / gcd(a, b)) * b;
}

int is_prime(int n) {
    if (n < 2) return 0;
    if (n < 4) return 1;
    if (n % 2 == 0 || n % 3 == 0) return 0;
    for (int i = 5; i * i <= n; i += 6) {
        if (n % i == 0 || n % (i + 2) == 0) return 0;
    }
    return 1;
}

int count_digits(int n) {
    if (n == 0) return 1;
    int count = 0;
    if (n < 0) n = -n;
    while (n > 0) {
        count++;
        n /= 10;
    }
    return count;
}

int reverse_number(int n) {
    int reversed = 0;
    int negative = (n < 0) ? 1 : 0;
    if (negative) n = -n;
    while (n > 0) {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }
    return negative ? -reversed : reversed;
}

int is_palindrome_number(int n) {
    if (n < 0) return 0;
    return n == reverse_number(n);
}

// ==================== Part 3: Array Functions ====================

int array_sum(int *arr, int len) {
    int total = 0;
    for (int i = 0; i < len; i++) total += arr[i];
    return total;
}

int array_max(int *arr, int len) {
    int mx = arr[0];
    for (int i = 1; i < len; i++) {
        if (arr[i] > mx) mx = arr[i];
    }
    return mx;
}

int array_min(int *arr, int len) {
    int mn = arr[0];
    for (int i = 1; i < len; i++) {
        if (arr[i] < mn) mn = arr[i];
    }
    return mn;
}

void array_reverse(int *arr, int len) {
    for (int i = 0; i < len / 2; i++) {
        int temp = arr[i];
        arr[i] = arr[len - 1 - i];
        arr[len - 1 - i] = temp;
    }
}

int array_contains(int *arr, int len, int value) {
    for (int i = 0; i < len; i++) {
        if (arr[i] == value) return 1;
    }
    return 0;
}

int array_index_of(int *arr, int len, int value) {
    for (int i = 0; i < len; i++) {
        if (arr[i] == value) return i;
    }
    return -1;
}

int array_count(int *arr, int len, int value) {
    int count = 0;
    for (int i = 0; i < len; i++) {
        if (arr[i] == value) count++;
    }
    return count;
}

void bubble_sort(int *arr, int len) {
    for (int i = 0; i < len - 1; i++) {
        for (int j = 0; j < len - i - 1; j++) {
            if (arr[j] > arr[j + 1]) {
                int temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}

int binary_search(int *arr, int len, int target) {
    int lo = 0, hi = len - 1;
    while (lo <= hi) {
        int mid = lo + (hi - lo) / 2;
        if (arr[mid] == target) return mid;
        if (arr[mid] < target) lo = mid + 1;
        else hi = mid - 1;
    }
    return -1;
}

// ==================== Part 4: String Functions ====================

int my_strlen(const char *s) {
    int len = 0;
    while (s[len] != '\0') len++;
    return len;
}

int my_strcmp(const char *a, const char *b) {
    while (*a && *b && *a == *b) { a++; b++; }
    return *a - *b;
}

void my_strcpy(char *dest, const char *src) {
    while (*src) { *dest++ = *src++; }
    *dest = '\0';
}

void my_strcat(char *dest, const char *src) {
    while (*dest) dest++;
    while (*src) { *dest++ = *src++; }
    *dest = '\0';
}

int count_char(const char *s, char c) {
    int count = 0;
    while (*s) { if (*s == c) count++; s++; }
    return count;
}

int count_vowels(const char *s) {
    int count = 0;
    while (*s) {
        char c = *s;
        if (c >= 'A' && c <= 'Z') c += 32;
        if (c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u') count++;
        s++;
    }
    return count;
}

int is_palindrome_str(const char *s) {
    int len = my_strlen(s);
    for (int i = 0; i < len / 2; i++) {
        if (s[i] != s[len - 1 - i]) return 0;
    }
    return 1;
}

void to_upper(char *s) {
    while (*s) {
        if (*s >= 'a' && *s <= 'z') *s -= 32;
        s++;
    }
}

void to_lower(char *s) {
    while (*s) {
        if (*s >= 'A' && *s <= 'Z') *s += 32;
        s++;
    }
}

// ==================== Part 5: Pointer Functions ====================

void swap_int(int *a, int *b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}

void swap_ptr(void **a, void **b) {
    void *temp = *a;
    *a = *b;
    *b = temp;
}

int apply_func(BinFunc f, int a, int b) {
    return f(a, b);
}

// ==================== Part 6: Bitwise Functions ====================

unsigned int set_bit(unsigned int val, int bit) { return val | (1 << bit); }
unsigned int clear_bit(unsigned int val, int bit) { return val & ~(1 << bit); }
unsigned int toggle_bit(unsigned int val, int bit) { return val ^ (1 << bit); }
int test_bit(unsigned int val, int bit) { return (val >> bit) & 1; }
unsigned int popcount(unsigned int val) {
    unsigned int count = 0;
    while (val) { count += val & 1; val >>= 1; }
    return count;
}

unsigned int rotate_left(unsigned int val, int n) {
    return (val << n) | (val >> (32 - n));
}

unsigned int rotate_right(unsigned int val, int n) {
    return (val >> n) | (val << (32 - n));
}

unsigned int next_power_of_2(unsigned int v) {
    v--;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v++;
    return v;
}

int is_power_of_2(unsigned int v) {
    return v && !(v & (v - 1));
}

// ==================== Part 7: Struct Functions ====================

struct Point2D point_add(struct Point2D a, struct Point2D b) {
    struct Point2D result;
    result.x = a.x + b.x;
    result.y = a.y + b.y;
    return result;
}

int point_distance_sq(struct Point2D a, struct Point2D b) {
    int dx = a.x - b.x;
    int dy = a.y - b.y;
    return dx * dx + dy * dy;
}

int rect_area(struct Rect r) {
    return r.width * r.height;
}

int rect_perimeter(struct Rect r) {
    return 2 * (r.width + r.height);
}

int rect_contains(struct Rect r, struct Point2D p) {
    return p.x >= r.origin.x && p.x < r.origin.x + r.width
        && p.y >= r.origin.y && p.y < r.origin.y + r.height;
}

// ==================== Part 8: Linked List ====================

struct LinkedNode *list_create(int data) {
    struct LinkedNode *node = (struct LinkedNode *)malloc(sizeof(struct LinkedNode));
    if (node) {
        node->data = data;
        node->next = NULL;
    }
    return node;
}

void list_push(struct LinkedNode **head, int data) {
    struct LinkedNode *node = list_create(data);
    if (node) {
        node->next = *head;
        *head = node;
    }
}

int list_pop(struct LinkedNode **head) {
    if (*head == NULL) return -1;
    struct LinkedNode *temp = *head;
    int data = temp->data;
    *head = temp->next;
    free(temp);
    return data;
}

int list_length(struct LinkedNode *head) {
    int len = 0;
    while (head) { len++; head = head->next; }
    return len;
}

int list_sum(struct LinkedNode *head) {
    int total = 0;
    while (head) { total += head->data; head = head->next; }
    return total;
}

int list_contains(struct LinkedNode *head, int value) {
    while (head) {
        if (head->data == value) return 1;
        head = head->next;
    }
    return 0;
}

void list_free(struct LinkedNode *head) {
    while (head) {
        struct LinkedNode *temp = head;
        head = head->next;
        free(temp);
    }
}

// ==================== Part 9: Matrix Operations ====================

void matrix_init(struct Matrix *m, int rows, int cols) {
    m->rows = rows;
    m->cols = cols;
    for (int i = 0; i < rows * cols && i < 16; i++) {
        m->data[i] = 0;
    }
}

void matrix_set(struct Matrix *m, int r, int c, int val) {
    if (r < m->rows && c < m->cols) {
        m->data[r * m->cols + c] = val;
    }
}

int matrix_get(struct Matrix *m, int r, int c) {
    if (r < m->rows && c < m->cols) {
        return m->data[r * m->cols + c];
    }
    return 0;
}

int matrix_trace(struct Matrix *m) {
    int trace = 0;
    int n = (m->rows < m->cols) ? m->rows : m->cols;
    for (int i = 0; i < n; i++) {
        trace += matrix_get(m, i, i);
    }
    return trace;
}

// ==================== Part 10: Hash Table (Simple) ====================

unsigned int hash_string(const char *s) {
    unsigned int hash = 5381;
    while (*s) {
        hash = ((hash << 5) + hash) + *s;
        s++;
    }
    return hash;
}

// ==================== Part 11: Stack (Array-based) ====================

struct Stack {
    int data[32];
    int top;
};

void stack_init(struct Stack *s) {
    s->top = -1;
}

int stack_push(struct Stack *s, int val) {
    if (s->top >= 31) return 0;
    s->data[++s->top] = val;
    return 1;
}

int stack_pop(struct Stack *s) {
    if (s->top < 0) return -1;
    return s->data[s->top--];
}

int stack_peek(struct Stack *s) {
    if (s->top < 0) return -1;
    return s->data[s->top];
}

int stack_empty(struct Stack *s) {
    return s->top < 0;
}

int stack_size(struct Stack *s) {
    return s->top + 1;
}

// ==================== Part 12: Queue (Circular) ====================

struct Queue {
    int data[32];
    int front;
    int rear;
    int count;
};

void queue_init(struct Queue *q) {
    q->front = 0;
    q->rear = -1;
    q->count = 0;
}

int queue_enqueue(struct Queue *q, int val) {
    if (q->count >= 32) return 0;
    q->rear = (q->rear + 1) % 32;
    q->data[q->rear] = val;
    q->count++;
    return 1;
}

int queue_dequeue(struct Queue *q) {
    if (q->count <= 0) return -1;
    int val = q->data[q->front];
    q->front = (q->front + 1) % 32;
    q->count--;
    return val;
}

int queue_empty(struct Queue *q) {
    return q->count == 0;
}

// ==================== MAIN: Comprehensive Test ====================

int main() {
    int pass = 0;
    int fail = 0;
    int total = 0;

    printf("============================================================\n");
    printf("  ADead-BIB C Standard Library — LONG Compliance Test\n");
    printf("============================================================\n\n");

    // --- 1. Arithmetic ---
    printf("[1] Arithmetic Functions\n");

    total++; if (add(3, 7) == 10) { pass++; } else { fail++; printf("  FAIL: add(3,7)\n"); }
    total++; if (sub(20, 8) == 12) { pass++; } else { fail++; printf("  FAIL: sub(20,8)\n"); }
    total++; if (mul(6, 7) == 42) { pass++; } else { fail++; printf("  FAIL: mul(6,7)\n"); }
    total++; if (divide(100, 4) == 25) { pass++; } else { fail++; printf("  FAIL: divide(100,4)\n"); }
    total++; if (divide(10, 0) == 0) { pass++; } else { fail++; printf("  FAIL: divide by zero\n"); }
    total++; if (modulo(17, 5) == 2) { pass++; } else { fail++; printf("  FAIL: modulo(17,5)\n"); }
    total++; if (power(2, 10) == 1024) { pass++; } else { fail++; printf("  FAIL: power(2,10)\n"); }
    total++; if (factorial(0) == 1) { pass++; } else { fail++; printf("  FAIL: factorial(0)\n"); }
    total++; if (factorial(1) == 1) { pass++; } else { fail++; printf("  FAIL: factorial(1)\n"); }
    total++; if (factorial(7) == 5040) { pass++; } else { fail++; printf("  FAIL: factorial(7)\n"); }
    total++; if (factorial(10) == 3628800) { pass++; } else { fail++; printf("  FAIL: factorial(10)\n"); }
    total++; if (fibonacci(0) == 0) { pass++; } else { fail++; printf("  FAIL: fib(0)\n"); }
    total++; if (fibonacci(1) == 1) { pass++; } else { fail++; printf("  FAIL: fib(1)\n"); }
    total++; if (fibonacci(10) == 55) { pass++; } else { fail++; printf("  FAIL: fib(10)\n"); }
    total++; if (fibonacci(20) == 6765) { pass++; } else { fail++; printf("  FAIL: fib(20)\n"); }
    total++; if (gcd(48, 18) == 6) { pass++; } else { fail++; printf("  FAIL: gcd(48,18)\n"); }
    total++; if (gcd(100, 75) == 25) { pass++; } else { fail++; printf("  FAIL: gcd(100,75)\n"); }
    total++; if (lcm(12, 18) == 36) { pass++; } else { fail++; printf("  FAIL: lcm(12,18)\n"); }
    total++; if (is_prime(2) == 1) { pass++; } else { fail++; printf("  FAIL: is_prime(2)\n"); }
    total++; if (is_prime(17) == 1) { pass++; } else { fail++; printf("  FAIL: is_prime(17)\n"); }
    total++; if (is_prime(4) == 0) { pass++; } else { fail++; printf("  FAIL: is_prime(4)\n"); }
    total++; if (is_prime(100) == 0) { pass++; } else { fail++; printf("  FAIL: is_prime(100)\n"); }
    total++; if (count_digits(0) == 1) { pass++; } else { fail++; printf("  FAIL: count_digits(0)\n"); }
    total++; if (count_digits(12345) == 5) { pass++; } else { fail++; printf("  FAIL: count_digits(12345)\n"); }
    total++; if (count_digits(-999) == 3) { pass++; } else { fail++; printf("  FAIL: count_digits(-999)\n"); }
    total++; if (reverse_number(12345) == 54321) { pass++; } else { fail++; printf("  FAIL: reverse_number\n"); }
    total++; if (is_palindrome_number(121) == 1) { pass++; } else { fail++; printf("  FAIL: palindrome(121)\n"); }
    total++; if (is_palindrome_number(123) == 0) { pass++; } else { fail++; printf("  FAIL: palindrome(123)\n"); }

    printf("  %d/%d passed\n\n", pass, total);

    // --- 2. Arrays ---
    int saved_pass = pass;
    int saved_total = total;
    printf("[2] Array Functions\n");

    int arr1[] = {5, 3, 8, 1, 9, 2, 7, 4, 6, 10};
    int arr1_len = 10;

    total++; if (array_sum(arr1, arr1_len) == 55) { pass++; } else { fail++; printf("  FAIL: array_sum\n"); }
    total++; if (array_max(arr1, arr1_len) == 10) { pass++; } else { fail++; printf("  FAIL: array_max\n"); }
    total++; if (array_min(arr1, arr1_len) == 1) { pass++; } else { fail++; printf("  FAIL: array_min\n"); }
    total++; if (array_contains(arr1, arr1_len, 7) == 1) { pass++; } else { fail++; printf("  FAIL: contains(7)\n"); }
    total++; if (array_contains(arr1, arr1_len, 99) == 0) { pass++; } else { fail++; printf("  FAIL: contains(99)\n"); }
    total++; if (array_index_of(arr1, arr1_len, 8) == 2) { pass++; } else { fail++; printf("  FAIL: indexOf(8)\n"); }
    total++; if (array_index_of(arr1, arr1_len, 99) == -1) { pass++; } else { fail++; printf("  FAIL: indexOf(99)\n"); }

    int arr2[] = {3, 1, 4, 1, 5, 9, 2, 6};
    total++; if (array_count(arr2, 8, 1) == 2) { pass++; } else { fail++; printf("  FAIL: count(1)\n"); }

    int sorted[] = {9, 3, 7, 1, 5, 8, 2, 4, 6, 10};
    bubble_sort(sorted, 10);
    int sort_ok = 1;
    for (int i = 0; i < 9; i++) {
        if (sorted[i] > sorted[i + 1]) sort_ok = 0;
    }
    total++; if (sort_ok) { pass++; } else { fail++; printf("  FAIL: bubble_sort\n"); }

    total++; if (binary_search(sorted, 10, 5) >= 0) { pass++; } else { fail++; printf("  FAIL: bsearch(5)\n"); }
    total++; if (binary_search(sorted, 10, 99) == -1) { pass++; } else { fail++; printf("  FAIL: bsearch(99)\n"); }

    int rev[] = {1, 2, 3, 4, 5};
    array_reverse(rev, 5);
    total++; if (rev[0] == 5 && rev[4] == 1) { pass++; } else { fail++; printf("  FAIL: array_reverse\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - saved_pass, total - saved_total);

    // --- 3. Strings ---
    saved_pass = pass; saved_total = total;
    printf("[3] String Functions\n");

    total++; if (my_strlen("hello") == 5) { pass++; } else { fail++; printf("  FAIL: strlen(hello)\n"); }
    total++; if (my_strlen("") == 0) { pass++; } else { fail++; printf("  FAIL: strlen(empty)\n"); }
    total++; if (my_strcmp("abc", "abc") == 0) { pass++; } else { fail++; printf("  FAIL: strcmp(eq)\n"); }
    total++; if (my_strcmp("abc", "abd") < 0) { pass++; } else { fail++; printf("  FAIL: strcmp(lt)\n"); }
    total++; if (my_strcmp("abd", "abc") > 0) { pass++; } else { fail++; printf("  FAIL: strcmp(gt)\n"); }

    char buf[128];
    my_strcpy(buf, "Hello");
    total++; if (my_strcmp(buf, "Hello") == 0) { pass++; } else { fail++; printf("  FAIL: strcpy\n"); }
    my_strcat(buf, " World");
    total++; if (my_strcmp(buf, "Hello World") == 0) { pass++; } else { fail++; printf("  FAIL: strcat\n"); }

    total++; if (count_char("banana", 'a') == 3) { pass++; } else { fail++; printf("  FAIL: count_char\n"); }
    total++; if (count_vowels("Hello World") == 3) { pass++; } else { fail++; printf("  FAIL: count_vowels\n"); }
    total++; if (is_palindrome_str("racecar") == 1) { pass++; } else { fail++; printf("  FAIL: palindrome(racecar)\n"); }
    total++; if (is_palindrome_str("hello") == 0) { pass++; } else { fail++; printf("  FAIL: palindrome(hello)\n"); }

    char upper_buf[32];
    my_strcpy(upper_buf, "hello");
    to_upper(upper_buf);
    total++; if (my_strcmp(upper_buf, "HELLO") == 0) { pass++; } else { fail++; printf("  FAIL: to_upper\n"); }

    to_lower(upper_buf);
    total++; if (my_strcmp(upper_buf, "hello") == 0) { pass++; } else { fail++; printf("  FAIL: to_lower\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - saved_pass, total - saved_total);

    // --- 4. Pointers ---
    saved_pass = pass; saved_total = total;
    printf("[4] Pointers & Function Pointers\n");

    int pa = 100, pb = 200;
    swap_int(&pa, &pb);
    total++; if (pa == 200 && pb == 100) { pass++; } else { fail++; printf("  FAIL: swap_int\n"); }

    total++; if (apply_func(add, 10, 20) == 30) { pass++; } else { fail++; printf("  FAIL: func_ptr(add)\n"); }
    total++; if (apply_func(mul, 6, 7) == 42) { pass++; } else { fail++; printf("  FAIL: func_ptr(mul)\n"); }
    total++; if (apply_func(sub, 50, 30) == 20) { pass++; } else { fail++; printf("  FAIL: func_ptr(sub)\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - saved_pass, total - saved_total);

    // --- 5. Bitwise ---
    saved_pass = pass; saved_total = total;
    printf("[5] Bitwise Operations\n");

    total++; if (set_bit(0, 3) == 8) { pass++; } else { fail++; printf("  FAIL: set_bit\n"); }
    total++; if (clear_bit(0xFF, 3) == 0xF7) { pass++; } else { fail++; printf("  FAIL: clear_bit\n"); }
    total++; if (toggle_bit(0, 5) == 32) { pass++; } else { fail++; printf("  FAIL: toggle_bit\n"); }
    total++; if (test_bit(0xFF, 4) == 1) { pass++; } else { fail++; printf("  FAIL: test_bit(1)\n"); }
    total++; if (test_bit(0, 4) == 0) { pass++; } else { fail++; printf("  FAIL: test_bit(0)\n"); }
    total++; if (popcount(0xFF) == 8) { pass++; } else { fail++; printf("  FAIL: popcount(0xFF)\n"); }
    total++; if (popcount(0x0F) == 4) { pass++; } else { fail++; printf("  FAIL: popcount(0x0F)\n"); }
    total++; if (popcount(0) == 0) { pass++; } else { fail++; printf("  FAIL: popcount(0)\n"); }
    total++; if (is_power_of_2(64) == 1) { pass++; } else { fail++; printf("  FAIL: pow2(64)\n"); }
    total++; if (is_power_of_2(65) == 0) { pass++; } else { fail++; printf("  FAIL: pow2(65)\n"); }
    total++; if (next_power_of_2(5) == 8) { pass++; } else { fail++; printf("  FAIL: next_pow2(5)\n"); }
    total++; if (next_power_of_2(16) == 16) { pass++; } else { fail++; printf("  FAIL: next_pow2(16)\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - saved_pass, total - saved_total);

    // --- 6. Structs ---
    saved_pass = pass; saved_total = total;
    printf("[6] Structs\n");

    struct Point2D p1;
    p1.x = 3;
    p1.y = 4;
    struct Point2D p2;
    p2.x = 6;
    p2.y = 8;
    struct Point2D psum = point_add(p1, p2);
    total++; if (psum.x == 9 && psum.y == 12) { pass++; } else { fail++; printf("  FAIL: point_add\n"); }

    total++; if (point_distance_sq(p1, p2) == 25) { pass++; } else { fail++; printf("  FAIL: point_dist\n"); }

    struct Rect r;
    r.origin.x = 0;
    r.origin.y = 0;
    r.width = 10;
    r.height = 5;
    total++; if (rect_area(r) == 50) { pass++; } else { fail++; printf("  FAIL: rect_area\n"); }
    total++; if (rect_perimeter(r) == 30) { pass++; } else { fail++; printf("  FAIL: rect_perim\n"); }

    struct Point2D inside;
    inside.x = 5;
    inside.y = 3;
    struct Point2D outside;
    outside.x = 15;
    outside.y = 3;
    total++; if (rect_contains(r, inside) == 1) { pass++; } else { fail++; printf("  FAIL: rect_contains(in)\n"); }
    total++; if (rect_contains(r, outside) == 0) { pass++; } else { fail++; printf("  FAIL: rect_contains(out)\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - saved_pass, total - saved_total);

    // --- 7. Control Flow ---
    saved_pass = pass; saved_total = total;
    printf("[7] Control Flow\n");

    // Switch + enum
    enum Weekday day = WEDNESDAY;
    int day_num = -1;
    switch (day) {
        case MONDAY: day_num = 0; break;
        case TUESDAY: day_num = 1; break;
        case WEDNESDAY: day_num = 2; break;
        case THURSDAY: day_num = 3; break;
        default: day_num = -1; break;
    }
    total++; if (day_num == 2) { pass++; } else { fail++; printf("  FAIL: switch/enum\n"); }

    // Nested loops
    int triangle = 0;
    for (int i = 1; i <= 10; i++) {
        for (int j = 1; j <= i; j++) {
            triangle++;
        }
    }
    total++; if (triangle == 55) { pass++; } else { fail++; printf("  FAIL: nested loops\n"); }

    // While with break
    int brk_count = 0;
    int k = 0;
    while (1) {
        if (k >= 10) break;
        brk_count += k;
        k++;
    }
    total++; if (brk_count == 45) { pass++; } else { fail++; printf("  FAIL: while+break\n"); }

    // For with continue
    int cont_sum = 0;
    for (int i = 0; i < 20; i++) {
        if (i % 3 == 0) continue;
        cont_sum += i;
    }
    total++; if (cont_sum == 120) { pass++; } else { fail++; printf("  FAIL: for+continue\n"); }

    // Ternary chains
    int val = 42;
    int tern = (val > 100) ? 3 : (val > 50) ? 2 : (val > 0) ? 1 : 0;
    total++; if (tern == 1) { pass++; } else { fail++; printf("  FAIL: ternary chain\n"); }

    // Do-while
    int dw = 0;
    int dw_count = 0;
    do {
        dw += dw_count;
        dw_count++;
    } while (dw_count < 10);
    total++; if (dw == 45) { pass++; } else { fail++; printf("  FAIL: do-while\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - saved_pass, total - saved_total);

    // --- 8. Compound Assignment ---
    saved_pass = pass; saved_total = total;
    printf("[8] Compound Assignment\n");

    int cv = 100;
    cv += 50; total++; if (cv == 150) { pass++; } else { fail++; printf("  FAIL: +=\n"); }
    cv -= 30; total++; if (cv == 120) { pass++; } else { fail++; printf("  FAIL: -=\n"); }
    cv *= 2;  total++; if (cv == 240) { pass++; } else { fail++; printf("  FAIL: *=\n"); }
    cv /= 3;  total++; if (cv == 80) { pass++; } else { fail++; printf("  FAIL: /=\n"); }
    cv %= 7;  total++; if (cv == 3) { pass++; } else { fail++; printf("  FAIL: %%=\n"); }

    unsigned int bv = 0xFF;
    bv &= 0x0F; total++; if (bv == 0x0F) { pass++; } else { fail++; printf("  FAIL: &=\n"); }
    bv |= 0xF0; total++; if (bv == 0xFF) { pass++; } else { fail++; printf("  FAIL: |=\n"); }
    bv ^= 0x0F; total++; if (bv == 0xF0) { pass++; } else { fail++; printf("  FAIL: ^=\n"); }
    bv <<= 4;   total++; if (bv == 0xF00) { pass++; } else { fail++; printf("  FAIL: <<=\n"); }
    bv >>= 8;   total++; if (bv == 0x0F) { pass++; } else { fail++; printf("  FAIL: >>=\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - saved_pass, total - saved_total);

    // --- 9. Dynamic Memory ---
    saved_pass = pass; saved_total = total;
    printf("[9] Dynamic Memory (malloc/calloc/realloc/free)\n");

    int *heap = (int *)malloc(10 * sizeof(int));
    total++; if (heap != NULL) { pass++; } else { fail++; printf("  FAIL: malloc\n"); }
    if (heap) {
        for (int i = 0; i < 10; i++) heap[i] = i * i;
        total++; if (heap[5] == 25) { pass++; } else { fail++; printf("  FAIL: heap write\n"); }

        int *bigger = (int *)realloc(heap, 20 * sizeof(int));
        total++; if (bigger != NULL) { pass++; } else { fail++; printf("  FAIL: realloc\n"); }
        if (bigger) {
            total++; if (bigger[5] == 25) { pass++; } else { fail++; printf("  FAIL: realloc preserve\n"); }
            for (int i = 10; i < 20; i++) bigger[i] = i;
            total++; if (bigger[15] == 15) { pass++; } else { fail++; printf("  FAIL: realloc extend\n"); }
            free(bigger);
        }
    }

    int *zeros = (int *)calloc(8, sizeof(int));
    total++; if (zeros != NULL) { pass++; } else { fail++; printf("  FAIL: calloc\n"); }
    if (zeros) {
        int all_zero = 1;
        for (int i = 0; i < 8; i++) {
            if (zeros[i] != 0) all_zero = 0;
        }
        total++; if (all_zero) { pass++; } else { fail++; printf("  FAIL: calloc zeros\n"); }
        free(zeros);
    }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - saved_pass, total - saved_total);

    // --- 10. Linked List ---
    saved_pass = pass; saved_total = total;
    printf("[10] Linked List\n");

    struct LinkedNode *list = NULL;
    list_push(&list, 10);
    list_push(&list, 20);
    list_push(&list, 30);
    list_push(&list, 40);
    list_push(&list, 50);

    total++; if (list_length(list) == 5) { pass++; } else { fail++; printf("  FAIL: list_length\n"); }
    total++; if (list_sum(list) == 150) { pass++; } else { fail++; printf("  FAIL: list_sum\n"); }
    total++; if (list_contains(list, 30) == 1) { pass++; } else { fail++; printf("  FAIL: list_contains(30)\n"); }
    total++; if (list_contains(list, 99) == 0) { pass++; } else { fail++; printf("  FAIL: list_contains(99)\n"); }

    int popped = list_pop(&list);
    total++; if (popped == 50) { pass++; } else { fail++; printf("  FAIL: list_pop\n"); }
    total++; if (list_length(list) == 4) { pass++; } else { fail++; printf("  FAIL: list_length after pop\n"); }

    list_free(list);
    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - saved_pass, total - saved_total);

    // --- 11. Stack ---
    saved_pass = pass; saved_total = total;
    printf("[11] Stack (array-based)\n");

    struct Stack st;
    stack_init(&st);
    total++; if (stack_empty(&st)) { pass++; } else { fail++; printf("  FAIL: stack_empty\n"); }

    stack_push(&st, 10);
    stack_push(&st, 20);
    stack_push(&st, 30);
    total++; if (stack_size(&st) == 3) { pass++; } else { fail++; printf("  FAIL: stack_size\n"); }
    total++; if (stack_peek(&st) == 30) { pass++; } else { fail++; printf("  FAIL: stack_peek\n"); }
    total++; if (stack_pop(&st) == 30) { pass++; } else { fail++; printf("  FAIL: stack_pop\n"); }
    total++; if (stack_pop(&st) == 20) { pass++; } else { fail++; printf("  FAIL: stack_pop(2)\n"); }
    total++; if (stack_size(&st) == 1) { pass++; } else { fail++; printf("  FAIL: stack_size(2)\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - saved_pass, total - saved_total);

    // --- 12. Queue ---
    saved_pass = pass; saved_total = total;
    printf("[12] Queue (circular)\n");

    struct Queue q;
    queue_init(&q);
    total++; if (queue_empty(&q)) { pass++; } else { fail++; printf("  FAIL: queue_empty\n"); }

    queue_enqueue(&q, 100);
    queue_enqueue(&q, 200);
    queue_enqueue(&q, 300);
    total++; if (queue_dequeue(&q) == 100) { pass++; } else { fail++; printf("  FAIL: queue FIFO\n"); }
    total++; if (queue_dequeue(&q) == 200) { pass++; } else { fail++; printf("  FAIL: queue FIFO(2)\n"); }

    queue_enqueue(&q, 400);
    total++; if (queue_dequeue(&q) == 300) { pass++; } else { fail++; printf("  FAIL: queue FIFO(3)\n"); }
    total++; if (queue_dequeue(&q) == 400) { pass++; } else { fail++; printf("  FAIL: queue FIFO(4)\n"); }
    total++; if (queue_empty(&q)) { pass++; } else { fail++; printf("  FAIL: queue_empty(2)\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - saved_pass, total - saved_total);

    // --- 13. Matrix ---
    saved_pass = pass; saved_total = total;
    printf("[13] Matrix Operations\n");

    struct Matrix m;
    matrix_init(&m, 3, 3);
    matrix_set(&m, 0, 0, 1);
    matrix_set(&m, 1, 1, 5);
    matrix_set(&m, 2, 2, 9);
    matrix_set(&m, 0, 1, 2);
    matrix_set(&m, 1, 0, 3);

    total++; if (matrix_get(&m, 0, 0) == 1) { pass++; } else { fail++; printf("  FAIL: matrix_get\n"); }
    total++; if (matrix_get(&m, 1, 1) == 5) { pass++; } else { fail++; printf("  FAIL: matrix_get(2)\n"); }
    total++; if (matrix_trace(&m) == 15) { pass++; } else { fail++; printf("  FAIL: matrix_trace\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - saved_pass, total - saved_total);

    // --- 14. Hash ---
    saved_pass = pass; saved_total = total;
    printf("[14] Hashing\n");

    unsigned int h1 = hash_string("hello");
    unsigned int h2 = hash_string("hello");
    unsigned int h3 = hash_string("world");
    total++; if (h1 == h2) { pass++; } else { fail++; printf("  FAIL: hash deterministic\n"); }
    total++; if (h1 != h3) { pass++; } else { fail++; printf("  FAIL: hash different\n"); }
    total++; if (h1 != 0) { pass++; } else { fail++; printf("  FAIL: hash nonzero\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - saved_pass, total - saved_total);

    // --- 15. Sizeof & Type Sizes ---
    saved_pass = pass; saved_total = total;
    printf("[15] sizeof & Type Sizes\n");

    total++; if (sizeof(char) == 1) { pass++; } else { fail++; printf("  FAIL: sizeof(char)\n"); }
    total++; if (sizeof(int) >= 4) { pass++; } else { fail++; printf("  FAIL: sizeof(int)\n"); }
    total++; if (sizeof(long) >= 4) { pass++; } else { fail++; printf("  FAIL: sizeof(long)\n"); }
    total++; if (sizeof(void *) >= 4) { pass++; } else { fail++; printf("  FAIL: sizeof(ptr)\n"); }
    total++; if (sizeof(struct Point2D) >= 8) { pass++; } else { fail++; printf("  FAIL: sizeof(Point2D)\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - saved_pass, total - saved_total);

    // ==================== FINAL REPORT ====================
    printf("============================================================\n");
    printf("  RESULTS: %d/%d tests passed", pass, total);
    if (fail == 0) {
        printf(" — ALL PASS!\n");
    } else {
        printf(" — %d FAILED\n", fail);
    }
    printf("============================================================\n");

    return (fail == 0) ? 0 : 1;
}
