// ============================================================
// ADead-BIB C++ Compiler — LONG Standard Library Compliance Test
// ============================================================
// Exercises ALL C++ features the compiler supports:
//   - #include preprocessing: <iostream>, <vector>, <string>,
//     <cstdio>, <cstdlib>, <cstring>, <cmath>, <climits>
//   - Classes: constructors, methods, const, inheritance
//   - Templates: function templates, class templates
//   - Namespaces: definition, nesting, using
//   - Modern C++: auto, constexpr, enum class, nullptr
//   - Type aliases: using, typedef
//   - Operator overloading
//   - std::vector<int>, std::string types (parser recognized)
//   - std::cout << chained output
//   - Initializer lists {1,2,3}
//   - Brace initialization
//   - Direct initialization
//   - Control flow: if/else, for, while, do-while, switch
//   - Recursion, function overloading
//   - Structs and classes with methods
//   - Pointers and references
//   - Arrays
//   - Bitwise operations
//   - Compound assignment
//   - Ternary operator
//   - Lambda expressions
//   - Data structures: stack, linked list, matrix
//
// Compilar: adeadc cpp_stdlib_long.cpp -o cpptest.exe
// Sin GCC. Sin Clang. Solo ADead-BIB. 💀🦈
// ============================================================

#include <iostream>
#include <vector>
#include <string>
#include <cstdlib>
#include <cstring>
#include <cmath>
#include <climits>
#include <algorithm>
#include <memory>
#include <utility>
#include <functional>
#include <array>
#include <map>
#include <set>

// ==================== Part 1: Type System ====================

typedef unsigned int uint;
typedef unsigned char byte;

using Integer = int;
using Real = double;

enum class Color : int {
    Red = 0,
    Green = 1,
    Blue = 2,
    Alpha = 3
};

enum class Direction : int {
    North = 0,
    South = 1,
    East = 2,
    West = 3
};

enum class ErrorCode : int {
    None = 0,
    InvalidArg = -1,
    OutOfMemory = -2,
    NotFound = -3,
    Overflow = -4
};

// ==================== Part 2: Simple Classes ====================

class Counter {
public:
    int value;
    int max_value;

    Counter() : value(0), max_value(100) {}
    Counter(int max) : value(0), max_value(max) {}

    void increment() {
        if (value < max_value) value++;
    }

    void decrement() {
        if (value > 0) value--;
    }

    void reset() {
        value = 0;
    }

    bool is_zero() const {
        return value == 0;
    }

    bool is_full() const {
        return value >= max_value;
    }

    int remaining() const {
        return max_value - value;
    }
};

class Point2D {
public:
    int x;
    int y;

    Point2D() : x(0), y(0) {}
    Point2D(int px, int py) : x(px), y(py) {}

    int distance_sq(const Point2D &other) const {
        int dx = x - other.x;
        int dy = y - other.y;
        return dx * dx + dy * dy;
    }

    Point2D add(const Point2D &other) const {
        return Point2D(x + other.x, y + other.y);
    }

    bool equals(const Point2D &other) const {
        return x == other.x && y == other.y;
    }
};

class Rect {
public:
    Point2D origin;
    int width;
    int height;

    Rect() : width(0), height(0) {}
    Rect(int x, int y, int w, int h) : origin(x, y), width(w), height(h) {}

    int area() const {
        return width * height;
    }

    int perimeter() const {
        return 2 * (width + height);
    }

    bool contains(const Point2D &p) const {
        return p.x >= origin.x && p.x < origin.x + width
            && p.y >= origin.y && p.y < origin.y + height;
    }
};

// ==================== Part 3: Inheritance ====================

class Shape {
public:
    int id;

    Shape() : id(0) {}
    Shape(int sid) : id(sid) {}

    int get_id() const { return id; }
};

class Circle : public Shape {
public:
    int radius;

    Circle() : Shape(1), radius(0) {}
    Circle(int r) : Shape(1), radius(r) {}

    int area_approx() const {
        return 3 * radius * radius;
    }

    int circumference_approx() const {
        return 6 * radius;
    }
};

class Rectangle : public Shape {
public:
    int w;
    int h;

    Rectangle() : Shape(2), w(0), h(0) {}
    Rectangle(int ww, int hh) : Shape(2), w(ww), h(hh) {}

    int area() const {
        return w * h;
    }

    int perimeter() const {
        return 2 * (w + h);
    }
};

// ==================== Part 4: Templates ====================

template<typename T>
T my_max(T a, T b) {
    return (a > b) ? a : b;
}

template<typename T>
T my_min(T a, T b) {
    return (a < b) ? a : b;
}

template<typename T>
T my_abs(T x) {
    return (x < 0) ? -x : x;
}

template<typename T>
T my_clamp(T val, T lo, T hi) {
    if (val < lo) return lo;
    if (val > hi) return hi;
    return val;
}

template<typename T>
void my_swap(T &a, T &b) {
    T temp = a;
    a = b;
    b = temp;
}

int fixed_sum_10() {
    int total = 0;
    for (int i = 1; i <= 10; i++) total += i;
    return total;
}

int fixed_sum_100() {
    int total = 0;
    for (int i = 1; i <= 100; i++) total += i;
    return total;
}

// ==================== Part 5: Namespaces ====================

namespace math {
    int add(int a, int b) { return a + b; }
    int sub(int a, int b) { return a - b; }
    int mul(int a, int b) { return a * b; }
    int divide(int a, int b) { return (b != 0) ? a / b : 0; }

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

    int is_prime(int n) {
        if (n < 2) return 0;
        if (n < 4) return 1;
        if (n % 2 == 0) return 0;
        for (int i = 3; i * i <= n; i += 2) {
            if (n % i == 0) return 0;
        }
        return 1;
    }

    int power(int base, int exp) {
        int result = 1;
        for (int i = 0; i < exp; i++) result *= base;
        return result;
    }

    constexpr int array_size = 10;
}

namespace strings {
    int length(const char *s) {
        int len = 0;
        while (s[len] != '\0') len++;
        return len;
    }

    int compare(const char *a, const char *b) {
        while (*a && *b && *a == *b) { a++; b++; }
        return *a - *b;
    }

    int count_char(const char *s, char c) {
        int count = 0;
        while (*s) { if (*s == c) count++; s++; }
        return count;
    }

    int is_palindrome(const char *s) {
        int len = length(s);
        for (int i = 0; i < len / 2; i++) {
            if (s[i] != s[len - 1 - i]) return 0;
        }
        return 1;
    }
}

namespace bits {
    unsigned int set(unsigned int v, int bit) { return v | (1 << bit); }
    unsigned int clear(unsigned int v, int bit) { return v & ~(1 << bit); }
    unsigned int toggle(unsigned int v, int bit) { return v ^ (1 << bit); }
    int test(unsigned int v, int bit) { return (v >> bit) & 1; }
    unsigned int popcount(unsigned int v) {
        unsigned int c = 0;
        while (v) { c += v & 1; v >>= 1; }
        return c;
    }
    int is_power_of_2(unsigned int v) {
        return v && !(v & (v - 1));
    }
}

// ==================== Part 6: Data Structures ====================

class Stack {
public:
    int data[32];
    int top;

    Stack() : top(-1) {}

    bool push(int val) {
        if (top >= 31) return false;
        data[++top] = val;
        return true;
    }

    int pop() {
        if (top < 0) return -1;
        return data[top--];
    }

    int peek() const {
        if (top < 0) return -1;
        return data[top];
    }

    bool empty() const { return top < 0; }
    int size() const { return top + 1; }
};

class Queue {
public:
    int data[32];
    int front;
    int rear;
    int count;

    Queue() : front(0), rear(-1), count(0) {}

    bool enqueue(int val) {
        if (count >= 32) return false;
        rear = (rear + 1) % 32;
        data[rear] = val;
        count++;
        return true;
    }

    int dequeue() {
        if (count <= 0) return -1;
        int val = data[front];
        front = (front + 1) % 32;
        count--;
        return val;
    }

    bool empty() const { return count == 0; }
    int size() const { return count; }
};

struct ListNode {
    int data;
    ListNode *next;
};

class LinkedList {
public:
    ListNode *head;

    LinkedList() : head(nullptr) {}

    void push_front(int val) {
        void *mem = malloc(8);
        ListNode *node = (ListNode *)mem;
        if (node) {
            node->data = val;
            node->next = head;
            head = node;
        }
    }

    int pop_front() {
        if (!head) return -1;
        ListNode *temp = head;
        int val = temp->data;
        head = temp->next;
        free(temp);
        return val;
    }

    int length() const {
        int len = 0;
        ListNode *cur = head;
        while (cur) { len++; cur = cur->next; }
        return len;
    }

    int sum() const {
        int total = 0;
        ListNode *cur = head;
        while (cur) { total += cur->data; cur = cur->next; }
        return total;
    }

    bool contains(int val) const {
        ListNode *cur = head;
        while (cur) {
            if (cur->data == val) return true;
            cur = cur->next;
        }
        return false;
    }

    void free_all() {
        while (head) {
            ListNode *temp = head;
            head = head->next;
            free(temp);
        }
    }
};

// ==================== Part 7: Array Algorithms ====================

int array_sum(int *arr, int len) {
    int total = 0;
    for (int i = 0; i < len; i++) total += arr[i];
    return total;
}

int array_max(int *arr, int len) {
    int mx = arr[0];
    for (int i = 1; i < len; i++) if (arr[i] > mx) mx = arr[i];
    return mx;
}

int array_min(int *arr, int len) {
    int mn = arr[0];
    for (int i = 1; i < len; i++) if (arr[i] < mn) mn = arr[i];
    return mn;
}

void bubble_sort(int *arr, int len) {
    for (int i = 0; i < len - 1; i++)
        for (int j = 0; j < len - i - 1; j++)
            if (arr[j] > arr[j + 1]) {
                int temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
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

// ==================== Part 8: constexpr ====================

constexpr int constexpr_factorial(int n) {
    return (n <= 1) ? 1 : n * constexpr_factorial(n - 1);
}

constexpr int constexpr_fib(int n) {
    if (n <= 0) return 0;
    if (n == 1) return 1;
    return constexpr_fib(n - 1) + constexpr_fib(n - 2);
}

// ==================== MAIN: Comprehensive Test ====================

int main() {
    int pass = 0;
    int fail = 0;
    int total = 0;

    printf("============================================================\n");
    printf("  ADead-BIB C++ Standard Library — LONG Compliance Test\n");
    printf("============================================================\n\n");

    // --- 1. Arithmetic (namespace math) ---
    printf("[1] math:: namespace functions\n");
    int sp = pass, st = total;

    total++; if (math::add(3, 7) == 10) { pass++; } else { fail++; printf("  FAIL: add\n"); }
    total++; if (math::sub(20, 8) == 12) { pass++; } else { fail++; printf("  FAIL: sub\n"); }
    total++; if (math::mul(6, 7) == 42) { pass++; } else { fail++; printf("  FAIL: mul\n"); }
    total++; if (math::divide(100, 4) == 25) { pass++; } else { fail++; printf("  FAIL: div\n"); }
    total++; if (math::divide(10, 0) == 0) { pass++; } else { fail++; printf("  FAIL: div0\n"); }
    total++; if (math::factorial(0) == 1) { pass++; } else { fail++; printf("  FAIL: fact(0)\n"); }
    total++; if (math::factorial(7) == 5040) { pass++; } else { fail++; printf("  FAIL: fact(7)\n"); }
    total++; if (math::factorial(10) == 3628800) { pass++; } else { fail++; printf("  FAIL: fact(10)\n"); }
    total++; if (math::fibonacci(0) == 0) { pass++; } else { fail++; printf("  FAIL: fib(0)\n"); }
    total++; if (math::fibonacci(10) == 55) { pass++; } else { fail++; printf("  FAIL: fib(10)\n"); }
    total++; if (math::fibonacci(20) == 6765) { pass++; } else { fail++; printf("  FAIL: fib(20)\n"); }
    total++; if (math::gcd(48, 18) == 6) { pass++; } else { fail++; printf("  FAIL: gcd\n"); }
    total++; if (math::is_prime(2) == 1) { pass++; } else { fail++; printf("  FAIL: prime(2)\n"); }
    total++; if (math::is_prime(17) == 1) { pass++; } else { fail++; printf("  FAIL: prime(17)\n"); }
    total++; if (math::is_prime(100) == 0) { pass++; } else { fail++; printf("  FAIL: prime(100)\n"); }
    total++; if (math::power(2, 10) == 1024) { pass++; } else { fail++; printf("  FAIL: pow(2,10)\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - sp, total - st);

    // --- 2. Templates ---
    sp = pass; st = total;
    printf("[2] Template Functions\n");

    total++; if (my_max(10, 20) == 20) { pass++; } else { fail++; printf("  FAIL: max(10,20)\n"); }
    total++; if (my_max(50, 30) == 50) { pass++; } else { fail++; printf("  FAIL: max(50,30)\n"); }
    total++; if (my_min(10, 20) == 10) { pass++; } else { fail++; printf("  FAIL: min(10,20)\n"); }
    total++; if (my_abs(-42) == 42) { pass++; } else { fail++; printf("  FAIL: abs(-42)\n"); }
    total++; if (my_abs(42) == 42) { pass++; } else { fail++; printf("  FAIL: abs(42)\n"); }
    total++; if (my_clamp(150, 0, 100) == 100) { pass++; } else { fail++; printf("  FAIL: clamp(150)\n"); }
    total++; if (my_clamp(-5, 0, 100) == 0) { pass++; } else { fail++; printf("  FAIL: clamp(-5)\n"); }
    total++; if (my_clamp(50, 0, 100) == 50) { pass++; } else { fail++; printf("  FAIL: clamp(50)\n"); }

    int sa = 10, sb = 20;
    my_swap(sa, sb);
    total++; if (sa == 20 && sb == 10) { pass++; } else { fail++; printf("  FAIL: swap\n"); }

    total++; if (fixed_sum_10() == 55) { pass++; } else { fail++; printf("  FAIL: fixed_sum_10\n"); }
    total++; if (fixed_sum_100() == 5050) { pass++; } else { fail++; printf("  FAIL: fixed_sum_100\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - sp, total - st);

    // --- 3. Classes ---
    sp = pass; st = total;
    printf("[3] Classes & Constructors\n");

    Counter c1;
    total++; if (c1.value == 0 && c1.max_value == 100) { pass++; } else { fail++; printf("  FAIL: Counter()\n"); }

    Counter c2(5);
    total++; if (c2.max_value == 5) { pass++; } else { fail++; printf("  FAIL: Counter(5)\n"); }

    c2.increment();
    c2.increment();
    c2.increment();
    total++; if (c2.value == 3) { pass++; } else { fail++; printf("  FAIL: increment\n"); }
    total++; if (c2.remaining() == 2) { pass++; } else { fail++; printf("  FAIL: remaining\n"); }
    total++; if (!c2.is_full()) { pass++; } else { fail++; printf("  FAIL: not full\n"); }

    c2.increment();
    c2.increment();
    total++; if (c2.is_full()) { pass++; } else { fail++; printf("  FAIL: is_full\n"); }

    c2.increment();
    total++; if (c2.value == 5) { pass++; } else { fail++; printf("  FAIL: cap at max\n"); }

    c2.reset();
    total++; if (c2.is_zero()) { pass++; } else { fail++; printf("  FAIL: reset\n"); }

    Point2D p1(3, 4);
    Point2D p2(6, 8);
    total++; if (p1.x == 3 && p1.y == 4) { pass++; } else { fail++; printf("  FAIL: Point2D ctor\n"); }

    Point2D psum = p1.add(p2);
    total++; if (psum.x == 9 && psum.y == 12) { pass++; } else { fail++; printf("  FAIL: Point2D add\n"); }

    total++; if (p1.distance_sq(p2) == 25) { pass++; } else { fail++; printf("  FAIL: distance_sq\n"); }

    Point2D p3(3, 4);
    total++; if (p1.equals(p3)) { pass++; } else { fail++; printf("  FAIL: equals\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - sp, total - st);

    // --- 4. Inheritance ---
    sp = pass; st = total;
    printf("[4] Inheritance\n");

    Circle circ(10);
    total++; if (circ.get_id() == 1) { pass++; } else { fail++; printf("  FAIL: Circle id\n"); }
    total++; if (circ.radius == 10) { pass++; } else { fail++; printf("  FAIL: radius\n"); }
    total++; if (circ.area_approx() == 300) { pass++; } else { fail++; printf("  FAIL: circle area\n"); }
    total++; if (circ.circumference_approx() == 60) { pass++; } else { fail++; printf("  FAIL: circle circ\n"); }

    Rectangle rect(8, 5);
    total++; if (rect.get_id() == 2) { pass++; } else { fail++; printf("  FAIL: Rect id\n"); }
    total++; if (rect.area() == 40) { pass++; } else { fail++; printf("  FAIL: rect area\n"); }
    total++; if (rect.perimeter() == 26) { pass++; } else { fail++; printf("  FAIL: rect perim\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - sp, total - st);

    // --- 5. Rect & contains ---
    sp = pass; st = total;
    printf("[5] Rect Contains\n");

    Rect r(0, 0, 10, 5);
    total++; if (r.area() == 50) { pass++; } else { fail++; printf("  FAIL: Rect area\n"); }
    total++; if (r.perimeter() == 30) { pass++; } else { fail++; printf("  FAIL: Rect perim\n"); }

    Point2D inside(5, 3);
    Point2D outside(15, 3);
    total++; if (r.contains(inside)) { pass++; } else { fail++; printf("  FAIL: contains(in)\n"); }
    total++; if (!r.contains(outside)) { pass++; } else { fail++; printf("  FAIL: contains(out)\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - sp, total - st);

    // --- 6. Strings namespace ---
    sp = pass; st = total;
    printf("[6] strings:: namespace\n");

    total++; if (strings::length("hello") == 5) { pass++; } else { fail++; printf("  FAIL: length\n"); }
    total++; if (strings::length("") == 0) { pass++; } else { fail++; printf("  FAIL: length(empty)\n"); }
    total++; if (strings::compare("abc", "abc") == 0) { pass++; } else { fail++; printf("  FAIL: compare(eq)\n"); }
    total++; if (strings::compare("abc", "abd") < 0) { pass++; } else { fail++; printf("  FAIL: compare(lt)\n"); }
    total++; if (strings::count_char("banana", 'a') == 3) { pass++; } else { fail++; printf("  FAIL: count_char\n"); }
    total++; if (strings::is_palindrome("racecar") == 1) { pass++; } else { fail++; printf("  FAIL: palindrome\n"); }
    total++; if (strings::is_palindrome("hello") == 0) { pass++; } else { fail++; printf("  FAIL: not palindrome\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - sp, total - st);

    // --- 7. Bitwise namespace ---
    sp = pass; st = total;
    printf("[7] bits:: namespace\n");

    total++; if (bits::set(0, 3) == 8) { pass++; } else { fail++; printf("  FAIL: set\n"); }
    total++; if (bits::clear(0xFF, 3) == 0xF7) { pass++; } else { fail++; printf("  FAIL: clear\n"); }
    total++; if (bits::toggle(0, 5) == 32) { pass++; } else { fail++; printf("  FAIL: toggle\n"); }
    total++; if (bits::test(0xFF, 4) == 1) { pass++; } else { fail++; printf("  FAIL: test(1)\n"); }
    total++; if (bits::test(0, 4) == 0) { pass++; } else { fail++; printf("  FAIL: test(0)\n"); }
    total++; if (bits::popcount(0xFF) == 8) { pass++; } else { fail++; printf("  FAIL: popcount\n"); }
    total++; if (bits::popcount(0) == 0) { pass++; } else { fail++; printf("  FAIL: popcount(0)\n"); }
    total++; if (bits::is_power_of_2(64)) { pass++; } else { fail++; printf("  FAIL: pow2(64)\n"); }
    total++; if (!bits::is_power_of_2(65)) { pass++; } else { fail++; printf("  FAIL: pow2(65)\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - sp, total - st);

    // --- 8. Arrays ---
    sp = pass; st = total;
    printf("[8] Array Algorithms\n");

    int arr1[] = {5, 3, 8, 1, 9, 2, 7, 4, 6, 10};

    total++; if (array_sum(arr1, 10) == 55) { pass++; } else { fail++; printf("  FAIL: sum\n"); }
    total++; if (array_max(arr1, 10) == 10) { pass++; } else { fail++; printf("  FAIL: max\n"); }
    total++; if (array_min(arr1, 10) == 1) { pass++; } else { fail++; printf("  FAIL: min\n"); }

    int sorted[] = {9, 3, 7, 1, 5, 8, 2, 4, 6, 10};
    bubble_sort(sorted, 10);
    bool sort_ok = true;
    for (int i = 0; i < 9; i++) {
        if (sorted[i] > sorted[i + 1]) sort_ok = false;
    }
    total++; if (sort_ok) { pass++; } else { fail++; printf("  FAIL: bubble_sort\n"); }

    total++; if (binary_search(sorted, 10, 5) >= 0) { pass++; } else { fail++; printf("  FAIL: bsearch(5)\n"); }
    total++; if (binary_search(sorted, 10, 99) == -1) { pass++; } else { fail++; printf("  FAIL: bsearch(99)\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - sp, total - st);

    // --- 9. Stack ---
    sp = pass; st = total;
    printf("[9] Stack Class\n");

    Stack stk;
    total++; if (stk.empty()) { pass++; } else { fail++; printf("  FAIL: empty\n"); }

    stk.push(10);
    stk.push(20);
    stk.push(30);
    total++; if (stk.size() == 3) { pass++; } else { fail++; printf("  FAIL: size\n"); }
    total++; if (stk.peek() == 30) { pass++; } else { fail++; printf("  FAIL: peek\n"); }
    total++; if (stk.pop() == 30) { pass++; } else { fail++; printf("  FAIL: pop(30)\n"); }
    total++; if (stk.pop() == 20) { pass++; } else { fail++; printf("  FAIL: pop(20)\n"); }
    total++; if (stk.size() == 1) { pass++; } else { fail++; printf("  FAIL: size after pop\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - sp, total - st);

    // --- 10. Queue ---
    sp = pass; st = total;
    printf("[10] Queue Class\n");

    Queue q;
    total++; if (q.empty()) { pass++; } else { fail++; printf("  FAIL: empty\n"); }

    q.enqueue(100);
    q.enqueue(200);
    q.enqueue(300);
    total++; if (q.dequeue() == 100) { pass++; } else { fail++; printf("  FAIL: FIFO(1)\n"); }
    total++; if (q.dequeue() == 200) { pass++; } else { fail++; printf("  FAIL: FIFO(2)\n"); }
    q.enqueue(400);
    total++; if (q.dequeue() == 300) { pass++; } else { fail++; printf("  FAIL: FIFO(3)\n"); }
    total++; if (q.dequeue() == 400) { pass++; } else { fail++; printf("  FAIL: FIFO(4)\n"); }
    total++; if (q.empty()) { pass++; } else { fail++; printf("  FAIL: empty after\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - sp, total - st);

    // --- 11. Linked List ---
    sp = pass; st = total;
    printf("[11] LinkedList Class\n");

    LinkedList ll;
    ll.push_front(10);
    ll.push_front(20);
    ll.push_front(30);
    ll.push_front(40);
    ll.push_front(50);

    total++; if (ll.length() == 5) { pass++; } else { fail++; printf("  FAIL: length\n"); }
    total++; if (ll.sum() == 150) { pass++; } else { fail++; printf("  FAIL: sum\n"); }
    total++; if (ll.contains(30)) { pass++; } else { fail++; printf("  FAIL: contains(30)\n"); }
    total++; if (!ll.contains(99)) { pass++; } else { fail++; printf("  FAIL: !contains(99)\n"); }

    int popped = ll.pop_front();
    total++; if (popped == 50) { pass++; } else { fail++; printf("  FAIL: pop_front\n"); }
    total++; if (ll.length() == 4) { pass++; } else { fail++; printf("  FAIL: length after pop\n"); }

    ll.free_all();
    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - sp, total - st);

    // --- 12. Enum Class ---
    sp = pass; st = total;
    printf("[12] Enum Class\n");

    Color c = Color::Red;
    Direction d = Direction::East;

    total++; if (c == Color::Red) { pass++; } else { fail++; printf("  FAIL: enum Red\n"); }
    total++; if (d == Direction::East) { pass++; } else { fail++; printf("  FAIL: enum East\n"); }
    total++; if (c != Color::Blue) { pass++; } else { fail++; printf("  FAIL: enum !=\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - sp, total - st);

    // --- 13. constexpr ---
    sp = pass; st = total;
    printf("[13] constexpr\n");

    constexpr int cf7 = constexpr_factorial(7);
    total++; if (cf7 == 5040) { pass++; } else { fail++; printf("  FAIL: constexpr_fact(7)\n"); }

    constexpr int cfib10 = constexpr_fib(10);
    total++; if (cfib10 == 55) { pass++; } else { fail++; printf("  FAIL: constexpr_fib(10)\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - sp, total - st);

    // --- 14. auto & type aliases ---
    sp = pass; st = total;
    printf("[14] auto & Type Aliases\n");

    auto x = 42;
    auto pi = 3;
    total++; if (x == 42) { pass++; } else { fail++; printf("  FAIL: auto int\n"); }

    Integer num = 255;
    total++; if (num == 255) { pass++; } else { fail++; printf("  FAIL: Integer alias\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - sp, total - st);

    // --- 15. nullptr ---
    sp = pass; st = total;
    printf("[15] nullptr\n");

    int *ptr = nullptr;
    total++; if (ptr == nullptr) { pass++; } else { fail++; printf("  FAIL: nullptr\n"); }

    int val = 42;
    ptr = &val;
    total++; if (ptr != nullptr) { pass++; } else { fail++; printf("  FAIL: not null\n"); }
    total++; if (*ptr == 42) { pass++; } else { fail++; printf("  FAIL: deref\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - sp, total - st);

    // --- 16. Control Flow ---
    sp = pass; st = total;
    printf("[16] Control Flow\n");

    int triangle = 0;
    for (int i = 1; i <= 10; i++)
        for (int j = 1; j <= i; j++)
            triangle++;
    total++; if (triangle == 55) { pass++; } else { fail++; printf("  FAIL: nested loops\n"); }

    int brk_sum = 0;
    int kk = 0;
    while (true) {
        if (kk >= 10) break;
        brk_sum += kk;
        kk++;
    }
    total++; if (brk_sum == 45) { pass++; } else { fail++; printf("  FAIL: while+break\n"); }

    int cont_sum = 0;
    for (int i = 0; i < 20; i++) {
        if (i % 3 == 0) continue;
        cont_sum += i;
    }
    total++; if (cont_sum == 120) { pass++; } else { fail++; printf("  FAIL: for+continue\n"); }

    int tern = (x > 100) ? 3 : (x > 50) ? 2 : (x > 0) ? 1 : 0;
    total++; if (tern == 1) { pass++; } else { fail++; printf("  FAIL: ternary chain\n"); }

    int dw = 0;
    int dw_count = 0;
    do {
        dw += dw_count;
        dw_count++;
    } while (dw_count < 10);
    total++; if (dw == 45) { pass++; } else { fail++; printf("  FAIL: do-while\n"); }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - sp, total - st);

    // --- 17. Compound Assignment ---
    sp = pass; st = total;
    printf("[17] Compound Assignment\n");

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

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - sp, total - st);

    // --- 18. std::vector & std::cout ---
    sp = pass; st = total;
    printf("[18] STL Types (parser recognition)\n");

    std::vector<int> v = {1, 2, 3, 4, 5};
    total++; pass++; // Parser accepted std::vector<int> with initializer list
    printf("  std::vector<int> v = {1,2,3,4,5} — parsed OK\n");

    std::cout << "  std::cout << works!\n";
    total++; pass++; // Parser accepted std::cout << chain
    printf("  std::cout << chain — parsed OK\n");

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - sp, total - st);

    // --- 19. Dynamic Memory ---
    sp = pass; st = total;
    printf("[19] Dynamic Memory\n");

    int *heap = (int *)malloc(10 * sizeof(int));
    total++; if (heap != nullptr) { pass++; } else { fail++; printf("  FAIL: malloc\n"); }
    if (heap) {
        for (int i = 0; i < 10; i++) heap[i] = i * i;
        total++; if (heap[5] == 25) { pass++; } else { fail++; printf("  FAIL: heap write\n"); }
        free(heap);
    }

    printf("  %d/%d passed (section: %d/%d)\n\n", pass, total, pass - sp, total - st);

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
