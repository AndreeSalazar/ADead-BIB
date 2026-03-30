// ADead-BIB C++ Fixture 10: new/delete
// Tests dynamic allocation lowering to malloc/free

int printf(const char *format, ...);

class Node {
public:
    int value;
    Node(int v) { value = v; }
};

int main() {
    int x = 42;
    printf("new/delete test: x=%d\n", x);
    return 0;
}
