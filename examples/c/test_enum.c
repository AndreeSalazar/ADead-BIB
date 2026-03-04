#include <stdio.h>

enum Color { RED = 0, GREEN = 1, BLUE = 2, ALPHA = 3 };
enum Status { OK, ERROR, PENDING };

int main() {
    enum Color c = GREEN;
    printf("GREEN=%d\n", c);
    
    enum Status s = OK;
    if (s == OK) {
        printf("status=OK\n");
    }
    
    int val = BLUE + ALPHA;
    printf("BLUE+ALPHA=%d\n", val);
    return 0;
}
