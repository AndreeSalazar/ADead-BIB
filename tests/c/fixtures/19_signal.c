// Test: <signal.h> — Signal handling
// Expected: Compile OK — signal macros + function declarations

#include <signal.h>
#include <stdio.h>

volatile int got_signal = 0;

void handler(int sig) {
    got_signal = sig;
}

int main() {
    printf("=== signal.h test ===\n");

    // Signal constants
    int s1 = SIGINT;
    int s2 = SIGTERM;
    int s3 = SIGSEGV;
    int s4 = SIGABRT;
    printf("SIGINT=%d SIGTERM=%d SIGSEGV=%d SIGABRT=%d\n", s1, s2, s3, s4);

    // Register handler
    signal(SIGUSR1, handler);

    // Raise signal
    raise(SIGUSR1);
    printf("got_signal=%d (expected %d)\n", got_signal, SIGUSR1);

    // SIG_DFL / SIG_IGN
    signal(SIGUSR1, SIG_DFL);
    signal(SIGUSR2, SIG_IGN);

    printf("=== signal.h OK ===\n");
    return 0;
}
