// Test 31: Global variable
int global = 42;

int get_global(void) { return global; }

int main() {
    global = global + 8;  // 50
    return get_global() - 50;
}
