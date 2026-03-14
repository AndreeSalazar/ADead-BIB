#include <header_main.h>
#include <string>

int main() {
    // Basic string construction
    std::string s1 = "hello";
    std::string s2 = "world";
    printf("s1: %s\n", s1.c_str());
    printf("s2: %s\n", s2.c_str());
    printf("s1 size: %d\n", s1.size());
    printf("s2 size: %d\n", s2.size());

    // Comparison
    std::string a = "abc";
    std::string b = "abc";
    std::string c = "xyz";
    printf("a==b: %d\n", a == b);
    printf("a==c: %d\n", a == c);

    // Empty string
    std::string empty;
    printf("empty: %d\n", empty.empty());

    // String with longer content
    std::string longstr = "this is a longer string for testing";
    printf("long: %s\n", longstr.c_str());
    printf("long size: %d\n", longstr.size());

    // Substr
    std::string sub = longstr.substr(0, 4);
    printf("substr: %s\n", sub.c_str());

    return 0;
}
