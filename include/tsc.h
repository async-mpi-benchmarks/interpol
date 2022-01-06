#pragma once

#include <stdint.h>

static inline uint64_t fenced_rdtscp()
{
    uint64_t tsc;
    asm volatile(
        // `LFENCE`: Serializes all load (read) operations that ocurred prior
        // to the `LFENCE` instruction in the program instruction stream, but
        // does not affect store operations.
        // If software requires `RDTSCP` to be executed prior to execution of
        // any subsequent instruction (including any memory accesses), it can
        // execute `LFENCE` immediately after `RDTSCP`.
        //
        // Referenced from:
        // Intel 64 and IA-32 Architectures software developer's manual
        // Volume 3, section 8.2.5
        "rdtscp                  \n\t"
        "lfence                  \n\t"
        "shl     $0x20, %%rdx    \n\t"
        "or      %%rdx, %%rax    \n\t"
        : "=a" (tsc)
        :
        : "rdx", "rcx");
    return tsc;
}

static inline uint64_t rdtsc()
{
    uint64_t tsc;
    asm volatile(
        "rdtsc                  \n\t"
        "shl    $0x20, %%rdx    \n\t"
        "or     %%rdx, %%rax    \n\t"
        : "=a" (tsc)
        :
        : "rdx");
    return tsc;
}
