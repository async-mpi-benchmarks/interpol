#include "sync.h"

inline
uint64_t sync_rdtscp()
{
    uint64_t lo, hi;

    __asm__ volatile(
        "rdtscp;\n"
        "cpuid;\n"
        : "=r" (lo), "=r" (hi)
        :
        : "rax", "rbx", "rcx", "rdx");

    return ((hi << 32) | lo);
}

inline
uint64_t rdtsc()
{
    uint64_t lo, hi;

    __asm__ volatile(
        "rdtsc;\n"
        : "=r" (lo), "=r" (hi)
        :
        : "rax", "rdx");

    return ((hi << 32) | lo);
}
