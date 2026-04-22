#ifndef _POSIX_C_SOURCE
#define _POSIX_C_SOURCE 200809L
#endif

#include <time.h>
#include <errno.h>
#include <stdint.h>

static struct timespec xyo_timespec_from_ns(int64_t ns)
{
    struct timespec ts;
    ts.tv_sec = ns / 1000000000LL;
    ts.tv_nsec = ns % 1000000000LL;
    return ts;
}

int64_t xyo_now_ns(void)
{
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (int64_t)ts.tv_sec * 1000000000LL + ts.tv_nsec;
}

void xyo_sleep_until_ns(int64_t deadline_ns)
{
#if defined(__APPLE__)
    int64_t remaining_ns = deadline_ns - xyo_now_ns();
    if (remaining_ns <= 0)
    {
        return;
    }

    struct timespec ts = xyo_timespec_from_ns(remaining_ns);
    while (nanosleep(&ts, &ts) == -1 && errno == EINTR)
    {
    }
#else
    struct timespec ts = xyo_timespec_from_ns(deadline_ns);
    while (clock_nanosleep(CLOCK_MONOTONIC, TIMER_ABSTIME, &ts, NULL) == EINTR)
    {
    }
#endif
}
