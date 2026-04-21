#ifndef _POSIX_C_SOURCE
#define _POSIX_C_SOURCE 200809L
#endif

#include <time.h>
#include <errno.h>
#include <stdint.h>

int64_t xyo_now_ns(void)
{
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (int64_t)ts.tv_sec * 1000000000LL + ts.tv_nsec;
}

void xyo_sleep_until_ns(int64_t deadline_ns)
{
    struct timespec ts;
    ts.tv_sec = deadline_ns / 1000000000LL;
    ts.tv_nsec = deadline_ns % 1000000000LL;

    while (clock_nanosleep(CLOCK_MONOTONIC, TIMER_ABSTIME, &ts, NULL) == EINTR)
    {
    }
}
