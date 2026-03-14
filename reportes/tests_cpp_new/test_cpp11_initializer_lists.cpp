#include <header_main.h>
#include <vector>
#include <string>

struct Point {
    int x;
    int y;
};

// Test return with initializer list
Point make_origin() {
    return {};
}

Point make_point(int a, int b) {
    return {a, b};
}

int main() {
    // Brace initialization in variable declarations
    std::vector<int> v = {1, 2, 3, 4, 5};
    std::vector<std::string> names = {"Alice", "Bob", "Carol"};

    printf("v size: %d\n", (int)v.size());
    for (int i = 0; i < (int)v.size(); i++) {
        printf("v[%d] = %d\n", i, v[i]);
    }

    for (int i = 0; i < (int)names.size(); i++) {
        printf("name: %s\n", names[i].c_str());
    }

    // Return with initializer list
    Point origin = make_origin();
    Point p = make_point(10, 20);
    printf("origin: (%d, %d)\n", origin.x, origin.y);
    printf("point: (%d, %d)\n", p.x, p.y);

    return 0;
}
