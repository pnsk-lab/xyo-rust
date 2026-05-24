#include <stdint.h>

#if defined(_WIN32)
#include <windows.h>
#else
#ifndef _POSIX_C_SOURCE
#define _POSIX_C_SOURCE 200809L
#endif
#include <time.h>
#include <errno.h>
#endif

#if !defined(_WIN32)
static struct timespec xyo_timespec_from_ns(int64_t ns)
{
    struct timespec ts;
    ts.tv_sec = ns / 1000000000LL;
    ts.tv_nsec = ns % 1000000000LL;
    return ts;
}
#endif

int64_t xyo_now_ns(void)
{
#if defined(_WIN32)
    LARGE_INTEGER counter;
    LARGE_INTEGER frequency;
    QueryPerformanceCounter(&counter);
    QueryPerformanceFrequency(&frequency);
    return (int64_t)((long double)counter.QuadPart * 1000000000.0L / (long double)frequency.QuadPart);
#else
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (int64_t)ts.tv_sec * 1000000000LL + ts.tv_nsec;
#endif
}

void sleep_until_ns(int64_t deadline_ns)
{
#if defined(_WIN32)
    int64_t remaining_ns = deadline_ns - xyo_now_ns();
    while (remaining_ns > 0)
    {
        DWORD sleep_ms = remaining_ns > 1000000LL ? (DWORD)(remaining_ns / 1000000LL) : 0;
        Sleep(sleep_ms);
        remaining_ns = deadline_ns - xyo_now_ns();
    }
#elif defined(__APPLE__)
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

void xyo_wait_until_next_frame(double fps)
{
    if (fps <= 0.0)
    {
        return;
    }

    int64_t frame_ns = (int64_t)(1000000000.0 / fps);
    int64_t now_ns = xyo_now_ns();

    int64_t next_frame_ns =
        ((now_ns / frame_ns) + 1) * frame_ns;

    sleep_until_ns(next_frame_ns);
}