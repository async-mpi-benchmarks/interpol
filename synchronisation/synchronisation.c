#include "synchronisation.h"

inline
uint64_t cycles()
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
