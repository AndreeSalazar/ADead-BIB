#include <signal.h>
#include <stdio.h>

void handler(int sig) {
    printf("Caught signal %d\n", sig);
}

int main() {
    signal(SIGINT, handler);
    signal(SIGTERM, handler);
    signal(SIGABRT, handler);
    signal(SIGFPE, handler);
    signal(SIGSEGV, handler);
    int r = raise(SIGABRT);
    printf("raise returned %d\n", r);
    return 0;
}
