// ADead-BIB Test: time.h functions (time, clock, difftime, mktime, localtime, gmtime, asctime, ctime, strftime)
#include <stdio.h>
#include <time.h>
#include <string.h>

int main() {
    int pass = 0, fail = 0;

    // time()
    time_t now = time(0);
    if (now > 0) { pass++; printf("PASS: time()=%ld\n", (long)now); }
    else { fail++; printf("FAIL: time()=%ld\n", (long)now); }

    // clock()
    clock_t cl = clock();
    if (cl >= 0) { pass++; printf("PASS: clock()=%ld\n", (long)cl); }
    else { fail++; printf("FAIL: clock()=%ld\n", (long)cl); }

    // difftime
    time_t t1 = now;
    time_t t2 = now + 100;
    double diff = difftime(t2, t1);
    if (diff > 99.0 && diff < 101.0) { pass++; printf("PASS: difftime=%.0f\n", diff); }
    else { fail++; printf("FAIL: difftime=%.0f\n", diff); }

    // localtime
    struct tm *lt = localtime(&now);
    if (lt) {
        pass++; printf("PASS: localtime year=%d mon=%d day=%d\n",
            lt->tm_year + 1900, lt->tm_mon + 1, lt->tm_mday);
    } else { fail++; printf("FAIL: localtime null\n"); }

    // gmtime
    struct tm *gt = gmtime(&now);
    if (gt) {
        pass++; printf("PASS: gmtime year=%d mon=%d day=%d\n",
            gt->tm_year + 1900, gt->tm_mon + 1, gt->tm_mday);
    } else { fail++; printf("FAIL: gmtime null\n"); }

    // asctime
    if (lt) {
        char *asc = asctime(lt);
        if (asc && strlen(asc) > 5) { pass++; printf("PASS: asctime='%s'", asc); }
        else { fail++; printf("FAIL: asctime null or empty\n"); }
    }

    // ctime
    char *ct = ctime(&now);
    if (ct && strlen(ct) > 5) { pass++; printf("PASS: ctime='%s'", ct); }
    else { fail++; printf("FAIL: ctime null or empty\n"); }

    // strftime
    if (lt) {
        char buf[128];
        int len = strftime(buf, sizeof(buf), "%Y-%m-%d %H:%M:%S", lt);
        if (len > 0) { pass++; printf("PASS: strftime='%s'\n", buf); }
        else { fail++; printf("FAIL: strftime len=%d\n", len); }
    }

    // mktime (roundtrip: time -> localtime -> mktime should give same time)
    if (lt) {
        time_t roundtrip = mktime(lt);
        long delta = (long)(roundtrip - now);
        if (delta >= -1 && delta <= 1) { pass++; printf("PASS: mktime roundtrip delta=%ld\n", delta); }
        else { fail++; printf("FAIL: mktime roundtrip delta=%ld\n", delta); }
    }

    printf("\n=== time_basic: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
