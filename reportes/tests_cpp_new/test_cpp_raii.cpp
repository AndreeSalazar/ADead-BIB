#include <header_main.h>

class Guard {
    const char* name;
public:
    Guard(const char* n) : name(n) {
        printf("+ %s\n", name);
    }
    ~Guard() {
        printf("- %s\n", name);
    }
};

void scoped() {
    Guard a("A");
    Guard b("B");
    {
        Guard c("C");
    }
    Guard d("D");
}

int main() {
    scoped();
    return 0;
}
