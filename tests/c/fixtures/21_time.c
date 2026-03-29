// Test: <time.h> — Time functions
// Expected: Compile OK — time types + function declarations

#include <time.h>
#include <stdio.h>

int main() {
    printf("=== time.h test ===\n");

    // time_t
    time_t now = time(0);
    printf("time()=%ld\n", (long)now);

    // clock
    clock_t c = clock();
    printf("clock()=%ld CLOCKS_PER_SEC=%ld\n", (long)c, (long)CLOCKS_PER_SEC);

    // struct tm
    struct tm t;
    t.tm_year = 126;  // 2026 - 1900
    t.tm_mon  = 2;    // March (0-indexed)
    t.tm_mday = 29;
    t.tm_hour = 12;
    t.tm_min  = 0;
    t.tm_sec  = 0;
    t.tm_isdst = -1;

    // mktime
    time_t made = mktime(&t);
    printf("mktime: year=%d mon=%d mday=%d\n", t.tm_year, t.tm_mon, t.tm_mday);

    // difftime
    double diff = difftime(now, made);
    printf("difftime result computed\n");

    // strftime
    char buf[64];
    strftime(buf, sizeof(buf), "%Y-%m-%d", &t);
    printf("strftime: %s\n", buf);

    // Clock IDs
    printf("CLOCK_REALTIME=%d CLOCK_MONOTONIC=%d\n", CLOCK_REALTIME, CLOCK_MONOTONIC);

    printf("=== time.h OK ===\n");
    return 0;
}
