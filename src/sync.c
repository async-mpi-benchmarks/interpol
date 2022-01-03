#include "sync.h"

inline
uint64_t sync_rdtscp()
{
    uint64_t lo, hi;

    __asm__ volatile(
        "rdtscp;\n"
        "cpuid;\n"
        : "=a" (lo), "=d" (hi)
        :
        : "rax", "rbx", "rcx", "rdx");

    return ((hi << 32) | lo);
}

inline
uint64_t rdtsc()
{
    uint64_t lo, hi;

    __asm__ volatile(
        "rdtsc"
        : "=a" (lo), "=d" (hi)
        :
        : "eax", "edx");

    return ((hi << 32) | lo);
}
