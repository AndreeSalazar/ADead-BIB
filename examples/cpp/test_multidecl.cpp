#include <cstdio>

int main() {
    int pass = 5;
    int total = 10;
    int sp = pass, st = total;
    printf("sp=%d st=%d\n", sp, st);
    
    total++;
    if (10 == 10) { pass++; } else { printf("FAIL\n"); }
    printf("pass=%d total=%d\n", pass, total);
    printf("section: %d/%d\n", pass - sp, total - st);
    return 0;
}
