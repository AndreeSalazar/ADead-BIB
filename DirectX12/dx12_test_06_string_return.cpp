// Test 06: Class method returning string literal — the pattern that failed
typedef unsigned int UINT;

class MyString {
public:
    const char* c_str() const { return "hello"; }
};

int main() {
    MyString s;
    const char* p = s.c_str();
    return 0;
}
