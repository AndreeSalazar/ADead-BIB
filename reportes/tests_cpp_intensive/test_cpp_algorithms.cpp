// ============================================================
// ADead-BIB C++ Algorithms Intensive Test
// "Respetar Bits" — Type Strictness ULTRA
// ============================================================
// Ejecutar: adb cpp reportes/tests_cpp_intensive/test_cpp_algorithms.cpp
// ============================================================

#include <stdio.h>
#include <stdlib.h>

// ============================================================
// Test 1: Sorting Algorithms
// ============================================================
void bubble_sort(int* arr, int n) {
    for (int i = 0; i < n - 1; i++) {
        for (int j = 0; j < n - i - 1; j++) {
            if (arr[j] > arr[j + 1]) {
                int temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}

void quick_sort(int* arr, int low, int high) {
    if (low < high) {
        int pivot = arr[high];
        int i = low - 1;
        
        for (int j = low; j < high; j++) {
            if (arr[j] < pivot) {
                i++;
                int temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
        }
        
        int temp = arr[i + 1];
        arr[i + 1] = arr[high];
        arr[high] = temp;
        
        int pi = i + 1;
        quick_sort(arr, low, pi - 1);
        quick_sort(arr, pi + 1, high);
    }
}

void merge(int* arr, int l, int m, int r) {
    int n1 = m - l + 1;
    int n2 = r - m;
    
    int* L = (int*)malloc(sizeof(int) * n1);
    int* R = (int*)malloc(sizeof(int) * n2);
    
    for (int i = 0; i < n1; i++) L[i] = arr[l + i];
    for (int j = 0; j < n2; j++) R[j] = arr[m + 1 + j];
    
    int i = 0, j = 0, k = l;
    while (i < n1 && j < n2) {
        if (L[i] <= R[j]) {
            arr[k] = L[i];
            i++;
        } else {
            arr[k] = R[j];
            j++;
        }
        k++;
    }
    
    while (i < n1) { arr[k] = L[i]; i++; k++; }
    while (j < n2) { arr[k] = R[j]; j++; k++; }
    
    free(L);
    free(R);
}

void merge_sort(int* arr, int l, int r) {
    if (l < r) {
        int m = l + (r - l) / 2;
        merge_sort(arr, l, m);
        merge_sort(arr, m + 1, r);
        merge(arr, l, m, r);
    }
}

void print_array(const char* label, int* arr, int n) {
    printf("  %s: ", label);
    for (int i = 0; i < n; i++) printf("%d ", arr[i]);
    printf("\n");
}

void test_sorting() {
    printf("\n=== TEST 1: Sorting Algorithms ===\n");
    
    int arr1[] = {64, 34, 25, 12, 22, 11, 90};
    int n = 7;
    print_array("Original", arr1, n);
    bubble_sort(arr1, n);
    print_array("Bubble Sort", arr1, n);
    
    int arr2[] = {64, 34, 25, 12, 22, 11, 90};
    quick_sort(arr2, 0, n - 1);
    print_array("Quick Sort", arr2, n);
    
    int arr3[] = {64, 34, 25, 12, 22, 11, 90};
    merge_sort(arr3, 0, n - 1);
    print_array("Merge Sort", arr3, n);
}

// ============================================================
// Test 2: Search Algorithms
// ============================================================
int linear_search(int* arr, int n, int target) {
    for (int i = 0; i < n; i++) {
        if (arr[i] == target) return i;
    }
    return -1;
}

int binary_search(int* arr, int n, int target) {
    int left = 0, right = n - 1;
    while (left <= right) {
        int mid = left + (right - left) / 2;
        if (arr[mid] == target) return mid;
        if (arr[mid] < target) left = mid + 1;
        else right = mid - 1;
    }
    return -1;
}

int binary_search_recursive(int* arr, int left, int right, int target) {
    if (left > right) return -1;
    int mid = left + (right - left) / 2;
    if (arr[mid] == target) return mid;
    if (arr[mid] < target) return binary_search_recursive(arr, mid + 1, right, target);
    return binary_search_recursive(arr, left, mid - 1, target);
}

void test_searching() {
    printf("\n=== TEST 2: Search Algorithms ===\n");
    
    int arr[] = {2, 3, 4, 10, 40, 50, 60, 70, 80, 90};
    int n = 10;
    print_array("Array", arr, n);
    
    int target = 40;
    int idx = linear_search(arr, n, target);
    printf("  Linear search for %d: index %d\n", target, idx);
    
    idx = binary_search(arr, n, target);
    printf("  Binary search for %d: index %d\n", target, idx);
    
    idx = binary_search_recursive(arr, 0, n - 1, target);
    printf("  Binary search (recursive) for %d: index %d\n", target, idx);
    
    target = 100;
    idx = binary_search(arr, n, target);
    printf("  Binary search for %d (not found): index %d\n", target, idx);
}

// ============================================================
// Test 3: Mathematical Algorithms
// ============================================================
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

bool is_prime(int n) {
    if (n <= 1) return false;
    if (n <= 3) return true;
    if (n % 2 == 0 || n % 3 == 0) return false;
    for (int i = 5; i * i <= n; i += 6) {
        if (n % i == 0 || n % (i + 2) == 0) return false;
    }
    return true;
}

int fibonacci(int n) {
    if (n <= 1) return n;
    int a = 0, b = 1;
    for (int i = 2; i <= n; i++) {
        int temp = a + b;
        a = b;
        b = temp;
    }
    return b;
}

int factorial(int n) {
    int result = 1;
    for (int i = 2; i <= n; i++) {
        result = result * i;
    }
    return result;
}

int power(int base, int exp) {
    int result = 1;
    while (exp > 0) {
        if (exp % 2 == 1) result = result * base;
        base = base * base;
        exp = exp / 2;
    }
    return result;
}

void test_math_algorithms() {
    printf("\n=== TEST 3: Mathematical Algorithms ===\n");
    
    printf("  gcd(48, 18) = %d\n", gcd(48, 18));
    printf("  lcm(12, 18) = %d\n", lcm(12, 18));
    
    printf("  Primes up to 30: ");
    for (int i = 2; i <= 30; i++) {
        if (is_prime(i)) printf("%d ", i);
    }
    printf("\n");
    
    printf("  Fibonacci(10) = %d\n", fibonacci(10));
    printf("  Factorial(7) = %d\n", factorial(7));
    printf("  Power(2, 10) = %d\n", power(2, 10));
}

// ============================================================
// Test 4: String Algorithms
// ============================================================
int string_length(const char* str) {
    int len = 0;
    while (str[len] != '\0') len++;
    return len;
}

void string_reverse(char* str) {
    int len = string_length(str);
    for (int i = 0; i < len / 2; i++) {
        char temp = str[i];
        str[i] = str[len - 1 - i];
        str[len - 1 - i] = temp;
    }
}

bool is_palindrome(const char* str) {
    int len = string_length(str);
    for (int i = 0; i < len / 2; i++) {
        if (str[i] != str[len - 1 - i]) return false;
    }
    return true;
}

int string_compare(const char* s1, const char* s2) {
    while (*s1 && *s2 && *s1 == *s2) {
        s1++;
        s2++;
    }
    return *s1 - *s2;
}

int count_char(const char* str, char c) {
    int count = 0;
    while (*str) {
        if (*str == c) count++;
        str++;
    }
    return count;
}

void test_string_algorithms() {
    printf("\n=== TEST 4: String Algorithms ===\n");
    
    const char* str1 = "Hello, World!";
    printf("  Length of \"%s\" = %d\n", str1, string_length(str1));
    
    char str2[] = "Hello";
    printf("  Original: \"%s\"\n", str2);
    string_reverse(str2);
    printf("  Reversed: \"%s\"\n", str2);
    
    printf("  Is \"radar\" palindrome? %s\n", is_palindrome("radar") ? "yes" : "no");
    printf("  Is \"hello\" palindrome? %s\n", is_palindrome("hello") ? "yes" : "no");
    
    printf("  Compare \"abc\" vs \"abd\": %d\n", string_compare("abc", "abd"));
    printf("  Compare \"abc\" vs \"abc\": %d\n", string_compare("abc", "abc"));
    
    printf("  Count 'l' in \"Hello, World!\": %d\n", count_char("Hello, World!", 'l'));
}

// ============================================================
// Test 5: Graph Algorithms (simplified)
// ============================================================
const int MAX_VERTICES = 10;

class Graph {
private:
    int adj[MAX_VERTICES][MAX_VERTICES];
    int vertices;
    
public:
    Graph(int v) : vertices(v) {
        for (int i = 0; i < v; i++) {
            for (int j = 0; j < v; j++) {
                adj[i][j] = 0;
            }
        }
        printf("[Graph] Created with %d vertices\n", v);
    }
    
    void add_edge(int u, int v) {
        adj[u][v] = 1;
        adj[v][u] = 1;  // Undirected
    }
    
    void bfs(int start) {
        bool visited[MAX_VERTICES] = {false};
        int queue[MAX_VERTICES];
        int front = 0, rear = 0;
        
        visited[start] = true;
        queue[rear++] = start;
        
        printf("  BFS from %d: ", start);
        while (front < rear) {
            int current = queue[front++];
            printf("%d ", current);
            
            for (int i = 0; i < vertices; i++) {
                if (adj[current][i] && !visited[i]) {
                    visited[i] = true;
                    queue[rear++] = i;
                }
            }
        }
        printf("\n");
    }
    
    void dfs_util(int v, bool* visited) {
        visited[v] = true;
        printf("%d ", v);
        
        for (int i = 0; i < vertices; i++) {
            if (adj[v][i] && !visited[i]) {
                dfs_util(i, visited);
            }
        }
    }
    
    void dfs(int start) {
        bool visited[MAX_VERTICES] = {false};
        printf("  DFS from %d: ", start);
        dfs_util(start, visited);
        printf("\n");
    }
};

void test_graph_algorithms() {
    printf("\n=== TEST 5: Graph Algorithms ===\n");
    
    Graph g(6);
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(1, 3);
    g.add_edge(1, 4);
    g.add_edge(2, 4);
    g.add_edge(3, 5);
    g.add_edge(4, 5);
    
    g.bfs(0);
    g.dfs(0);
}

// ============================================================
// Test 6: Dynamic Programming
// ============================================================
int knapsack(int* weights, int* values, int n, int capacity) {
    int** dp = (int**)malloc(sizeof(int*) * (n + 1));
    for (int i = 0; i <= n; i++) {
        dp[i] = (int*)malloc(sizeof(int) * (capacity + 1));
    }
    
    for (int i = 0; i <= n; i++) {
        for (int w = 0; w <= capacity; w++) {
            if (i == 0 || w == 0) {
                dp[i][w] = 0;
            } else if (weights[i - 1] <= w) {
                int include = values[i - 1] + dp[i - 1][w - weights[i - 1]];
                int exclude = dp[i - 1][w];
                dp[i][w] = (include > exclude) ? include : exclude;
            } else {
                dp[i][w] = dp[i - 1][w];
            }
        }
    }
    
    int result = dp[n][capacity];
    
    for (int i = 0; i <= n; i++) free(dp[i]);
    free(dp);
    
    return result;
}

int longest_common_subsequence(const char* s1, const char* s2) {
    int m = string_length(s1);
    int n = string_length(s2);
    
    int** dp = (int**)malloc(sizeof(int*) * (m + 1));
    for (int i = 0; i <= m; i++) {
        dp[i] = (int*)malloc(sizeof(int) * (n + 1));
    }
    
    for (int i = 0; i <= m; i++) {
        for (int j = 0; j <= n; j++) {
            if (i == 0 || j == 0) {
                dp[i][j] = 0;
            } else if (s1[i - 1] == s2[j - 1]) {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = (dp[i - 1][j] > dp[i][j - 1]) ? dp[i - 1][j] : dp[i][j - 1];
            }
        }
    }
    
    int result = dp[m][n];
    
    for (int i = 0; i <= m; i++) free(dp[i]);
    free(dp);
    
    return result;
}

void test_dynamic_programming() {
    printf("\n=== TEST 6: Dynamic Programming ===\n");
    
    int weights[] = {10, 20, 30};
    int values[] = {60, 100, 120};
    int capacity = 50;
    printf("  Knapsack (capacity=%d): max value = %d\n", 
           capacity, knapsack(weights, values, 3, capacity));
    
    const char* s1 = "AGGTAB";
    const char* s2 = "GXTXAYB";
    printf("  LCS of \"%s\" and \"%s\" = %d\n", 
           s1, s2, longest_common_subsequence(s1, s2));
}

// ============================================================
// Main — Run all tests
// ============================================================
int main() {
    printf("============================================================\n");
    printf("ADead-BIB C++ Algorithms Intensive Test\n");
    printf("============================================================\n");
    
    test_sorting();
    test_searching();
    test_math_algorithms();
    test_string_algorithms();
    test_graph_algorithms();
    test_dynamic_programming();
    
    printf("\n============================================================\n");
    printf("All Algorithm tests completed!\n");
    printf("==============================================================\n");
    
    return 0;
}
