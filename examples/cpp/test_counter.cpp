#include <cstdio>

int main() {
    int total = 0;
    int pass = 0;
    
    total = total + 1;
    pass = pass + 1;
    printf("After +1: total=%d pass=%d\n", total, pass);
    
    total = total + 1;
    printf("After +1 again: total=%d\n", total);
    
    if (30 == 30) {
        pass = pass + 1;
    }
    printf("After if: pass=%d\n", pass);
    
    printf("Final: %d/%d\n", pass, total);
    return 0;
}
