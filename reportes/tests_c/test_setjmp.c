#include <setjmp.h>
#include <stdio.h>

jmp_buf env;

void do_jump() {
    longjmp(env, 42);
}

int main() {
    int val = setjmp(env);
    if (val == 0) {
        printf("First time through\n");
        do_jump();
    } else {
        printf("Returned from longjmp with val = %d\n", val);
    }
    return 0;
}
