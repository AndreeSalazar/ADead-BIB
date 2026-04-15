// ============================================================
// Test 05: Control de Flujo — if, else, while, for, do-while, switch, goto
// ============================================================
// ADead-BIB Test Canon — C99 §6.8
// Verifica: todas las estructuras de control
// ============================================================

#include <stdio.h>

int abs_val(int x) {
    if (x < 0) {
        return -x;
    } else {
        return x;
    }
}

int classify(int x) {
    if (x > 0) {
        return 1;
    } else if (x == 0) {
        return 0;
    } else {
        return -1;
    }
}

int sum_for(int n) {
    int sum = 0;
    int i;
    for (i = 1; i <= n; i++) {
        sum += i;
    }
    return sum;
}

int sum_while(int n) {
    int sum = 0;
    while (n > 0) {
        sum += n;
        n--;
    }
    return sum;
}

int sum_do_while(int n) {
    int sum = 0;
    do {
        sum += n;
        n--;
    } while (n > 0);
    return sum;
}

int switch_test(int x) {
    switch (x) {
        case 0: return 10;
        case 1: return 20;
        case 2: return 30;
        case 3: return 40;
        default: return -1;
    }
}

int switch_fallthrough(int x) {
    int result = 0;
    switch (x) {
        case 1:
        case 2:
        case 3:
            result = 100;
            break;
        case 4:
            result = 200;
            break;
        default:
            result = 0;
    }
    return result;
}

int break_continue() {
    int sum = 0;
    int i;
    for (i = 0; i < 20; i++) {
        if (i % 2 == 0) continue;
        if (i > 10) break;
        sum += i;
    }
    return sum;
}

int goto_loop(int limit) {
    int i = 0;
loop_start:
    if (i >= limit) goto loop_end;
    i++;
    goto loop_start;
loop_end:
    return i;
}

int nested_loops() {
    int count = 0;
    int i, j;
    for (i = 0; i < 5; i++) {
        for (j = 0; j < 5; j++) {
            if (i == j) count++;
        }
    }
    return count;
}

int main() {
    printf("abs(-5)=%d abs(3)=%d\n", abs_val(-5), abs_val(3));
    printf("classify: 5=%d 0=%d -3=%d\n", classify(5), classify(0), classify(-3));
    printf("sum_for(10)=%d\n", sum_for(10));
    printf("sum_while(10)=%d\n", sum_while(10));
    printf("sum_do_while(10)=%d\n", sum_do_while(10));
    printf("switch: 0=%d 2=%d 9=%d\n", switch_test(0), switch_test(2), switch_test(9));
    printf("fallthrough: 2=%d 4=%d\n", switch_fallthrough(2), switch_fallthrough(4));
    printf("break_continue=%d\n", break_continue());
    printf("goto_loop(10)=%d\n", goto_loop(10));
    printf("nested_loops=%d\n", nested_loops());
    return 0;
}
// Expected:
// abs(-5)=5 abs(3)=3
// classify: 5=1 0=0 -3=-1
// sum_for(10)=55
// sum_while(10)=55
// sum_do_while(10)=55
// switch: 0=10 2=30 9=-1
// fallthrough: 2=100 4=200
// break_continue=25
// goto_loop(10)=10
// nested_loops=5
