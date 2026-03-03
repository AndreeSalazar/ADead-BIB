/*
 * ADead-BIB Standard Library
 * time.h - Date and Time Functions
 * 
 * Based on: C99/C11 standard, POSIX
 */

#ifndef _ADEAD_TIME_H
#define _ADEAD_TIME_H

#include "stddef.h"

/* Clock types */
typedef long clock_t;
typedef long long time_t;

/* Clocks per second */
#define CLOCKS_PER_SEC 1000000

/* Time structure */
struct tm {
    int tm_sec;     /* Seconds (0-60) */
    int tm_min;     /* Minutes (0-59) */
    int tm_hour;    /* Hours (0-23) */
    int tm_mday;    /* Day of month (1-31) */
    int tm_mon;     /* Month (0-11) */
    int tm_year;    /* Year - 1900 */
    int tm_wday;    /* Day of week (0-6, Sunday = 0) */
    int tm_yday;    /* Day of year (0-365) */
    int tm_isdst;   /* Daylight saving time flag */
};

/* POSIX timespec */
struct timespec {
    time_t tv_sec;  /* Seconds */
    long tv_nsec;   /* Nanoseconds */
};

/* POSIX timeval */
struct timeval {
    time_t tv_sec;  /* Seconds */
    long tv_usec;   /* Microseconds */
};

/* Time manipulation */
clock_t clock(void);
time_t time(time_t* timer);
double difftime(time_t time1, time_t time0);
time_t mktime(struct tm* timeptr);

/* Time conversion */
char* asctime(const struct tm* timeptr);
char* ctime(const time_t* timer);
struct tm* gmtime(const time_t* timer);
struct tm* localtime(const time_t* timer);
size_t strftime(char* s, size_t maxsize, const char* format, const struct tm* timeptr);

/* Thread-safe versions (C11) */
struct tm* gmtime_r(const time_t* timer, struct tm* result);
struct tm* localtime_r(const time_t* timer, struct tm* result);
char* asctime_r(const struct tm* timeptr, char* buf);
char* ctime_r(const time_t* timer, char* buf);

/* POSIX clock functions */
#define CLOCK_REALTIME           0
#define CLOCK_MONOTONIC          1
#define CLOCK_PROCESS_CPUTIME_ID 2
#define CLOCK_THREAD_CPUTIME_ID  3

int clock_gettime(int clk_id, struct timespec* tp);
int clock_settime(int clk_id, const struct timespec* tp);
int clock_getres(int clk_id, struct timespec* res);

/* POSIX nanosleep */
int nanosleep(const struct timespec* req, struct timespec* rem);

/* Timezone */
extern char* tzname[2];
extern long timezone;
extern int daylight;
void tzset(void);

#endif /* _ADEAD_TIME_H */
