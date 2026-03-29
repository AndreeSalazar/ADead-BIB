// Test: Control flow — if/else, while, do-while, for, switch, goto/label
// Expected: All parse + lower correctly

int abs_val(int x) {
    if (x < 0) {
        return -x;
    } else {
        return x;
    }
}

int sum_to_n(int n) {
    int sum = 0;
    int i;
    for (i = 1; i <= n; i++) {
        sum += i;
    }
    return sum;
}

int while_countdown(int n) {
    int count = 0;
    while (n > 0) {
        count++;
        n--;
    }
    return count;
}

int do_while_example(int n) {
    int sum = 0;
    do {
        sum += n;
        n--;
    } while (n > 0);
    return sum;
}

int classify(int x) {
    switch (x) {
        case 0: return 0;
        case 1: return 1;
        case 2: return 4;
        case 3: return 9;
        default: return -1;
    }
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

int nested_if(int a, int b) {
    if (a > 0) {
        if (b > 0) {
            return 1;
        } else {
            return 2;
        }
    } else {
        if (b > 0) {
            return 3;
        } else {
            return 4;
        }
    }
}

int main() {
    int r1 = abs_val(-5);
    int r2 = sum_to_n(10);
    int r3 = while_countdown(5);
    int r4 = do_while_example(5);
    int r5 = classify(2);
    int r6 = goto_loop(10);
    int r7 = nested_if(1, -1);
    return 0;
}
