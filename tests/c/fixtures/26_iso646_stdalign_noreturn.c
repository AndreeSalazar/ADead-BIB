// Test: <iso646.h> + <stdalign.h> + <stdnoreturn.h> — C99/C11 utility headers
// Expected: Compile OK — operator macros + alignment + noreturn

#include <iso646.h>
#include <stdalign.h>
#include <stdnoreturn.h>
#include <stdio.h>

int main() {
    printf("=== iso646/stdalign/noreturn test ===\n");

    // iso646.h operator macros
    int a = 1, b = 0;
    int r1 = a and b;       // &&
    int r2 = a or b;        // ||
    int r3 = not a;          // !
    int r4 = a bitand b;    // &
    int r5 = a bitor b;     // |
    int r6 = a xor b;       // ^
    int r7 = compl 0;       // ~
    printf("and=%d or=%d not=%d bitand=%d bitor=%d xor=%d compl=%d\n",
           r1, r2, r3, r4, r5, r6, r7);

    // stdalign.h
    int al = alignof(int);
    printf("alignof(int)=%d\n", al);
    printf("__alignas_is_defined=%d __alignof_is_defined=%d\n",
           __alignas_is_defined, __alignof_is_defined);

    printf("=== iso646/stdalign/noreturn OK ===\n");
    return 0;
}
