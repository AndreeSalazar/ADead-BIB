// ADead-BIB C++ Fixture 13: Static Methods & Initializer Lists
// Tests static method (no this), member initializer list lowering

int printf(const char *format, ...);

class Config {
public:
    int width;
    int height;

    Config(int w, int h) : width(w), height(h) {}

    static int default_width() { return 800; }
    static int default_height() { return 600; }
};

int main() {
    printf("static method test\n");
    return 0;
}
