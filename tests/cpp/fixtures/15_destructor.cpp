// ADead-BIB C++ Fixture 15: Destructor & delete lowering
// Tests destructor emission and delete → dtor + free

int printf(const char *format, ...);

class Resource {
public:
    int handle;
    Resource(int h) { handle = h; }
    ~Resource() { handle = 0; }
};

int main() {
    int x = 42;
    printf("destructor test: %d\n", x);
    return 0;
}
