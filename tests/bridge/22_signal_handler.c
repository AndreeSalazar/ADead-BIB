// ADead-BIB Bridge Test 22 — Signal Handlers
// Level: INTERMEDIATE
// Tests: signal(), raise(), custom signal handlers, SIG_DFL restore

#include <stdio.h>
#include <signal.h>
#include <stdlib.h>

volatile int signal_received = 0;
volatile int sigfpe_received = 0;

void sigint_handler(int sig) {
    (void)sig;
    signal_received = 1;
}

void sigfpe_handler(int sig) {
    (void)sig;
    sigfpe_received = 1;
}

int main() {
    printf("=== ADead-BIB Bridge Test 22: Signal Handler ===\n");
    int pass = 0, fail = 0;

    // Install SIGINT handler
    void (*prev)(int) = signal(SIGINT, sigint_handler);
    if (prev != SIG_ERR) { pass++; } else { fail++; printf("FAIL: signal() returned SIG_ERR\n"); }

    // Raise SIGINT — handler should set flag
    signal_received = 0;
    raise(SIGINT);
    if (signal_received == 1) { pass++; } else { fail++; printf("FAIL: SIGINT handler not called\n"); }

    // Re-install and raise again
    signal(SIGINT, sigint_handler);
    signal_received = 0;
    raise(SIGINT);
    if (signal_received == 1) { pass++; } else { fail++; printf("FAIL: SIGINT handler second raise\n"); }

    // Install SIGFPE handler and raise
    signal(SIGFPE, sigfpe_handler);
    sigfpe_received = 0;
    raise(SIGFPE);
    if (sigfpe_received == 1) { pass++; } else { fail++; printf("FAIL: SIGFPE handler not called\n"); }

    // Verify both flags are independently set
    signal_received = 0;
    sigfpe_received = 0;
    signal(SIGINT, sigint_handler);
    signal(SIGFPE, sigfpe_handler);
    raise(SIGINT);
    if (signal_received == 1 && sigfpe_received == 0) { pass++; } else { fail++; printf("FAIL: only SIGINT should fire\n"); }

    raise(SIGFPE);
    if (sigfpe_received == 1) { pass++; } else { fail++; printf("FAIL: SIGFPE after SIGINT\n"); }

    // Restore default handler — signal() should return previous handler
    void (*old)(int) = signal(SIGINT, SIG_DFL);
    if (old == sigint_handler) { pass++; } else { fail++; printf("FAIL: signal() did not return previous handler\n"); }

    // SIG_IGN test — ignoring a signal should not crash
    signal(SIGINT, SIG_IGN);
    raise(SIGINT);
    pass++; // if we get here, SIG_IGN worked

    printf("Results: %d passed, %d failed\n", pass, fail);
    printf("=== Test 22: %s ===\n", fail == 0 ? "PASS" : "FAIL");
    return fail;
}
