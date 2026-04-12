// ADead-BIB Test: signal.h + locale.h functions
#include <stdio.h>
#include <signal.h>
#include <locale.h>
#include <string.h>

int signal_received = 0;

void my_handler(int sig) {
    signal_received = sig;
}

int main() {
    int pass = 0, fail = 0;

    // signal + raise
    signal(SIGUSR1, my_handler);
    // Note: SIGUSR1 may not exist on Windows msvcrt, use SIGINT instead
    // But we test with raise(SIGINT) which is safe
    
    // Test signal registration (doesn't crash)
    void (*prev)(int) = signal(SIGFPE, my_handler);
    if (prev != (void(*)(int))-1) { pass++; printf("PASS: signal() registered handler\n"); }
    else { fail++; printf("FAIL: signal() returned SIG_ERR\n"); }

    // setlocale
    char *loc = setlocale(0, 0); // LC_ALL=0, query current
    if (loc) { pass++; printf("PASS: setlocale query='%s'\n", loc); }
    else { fail++; printf("FAIL: setlocale query null\n"); }

    // Set to C locale
    loc = setlocale(0, "C");
    if (loc) { pass++; printf("PASS: setlocale set C='%s'\n", loc); }
    else { fail++; printf("FAIL: setlocale set C\n"); }

    // localeconv
    struct lconv *lc = localeconv();
    if (lc) {
        pass++; printf("PASS: localeconv decimal='%s'\n", lc->decimal_point);
    } else { fail++; printf("FAIL: localeconv null\n"); }

    printf("\n=== signal_locale: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
