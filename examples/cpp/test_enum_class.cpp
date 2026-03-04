int printf(const char *format, ...);

enum class Color : int {
    Red = 0,
    Green = 1,
    Blue = 2,
    Alpha = 3
};

enum class Status : int {
    OK = 0,
    Error = 1,
    Pending = 2
};

int main() {
    Color c = Color::Green;
    printf("Green=%d\n", c);
    Status s = Status::OK;
    printf("OK=%d\n", s);
    return 0;
}
