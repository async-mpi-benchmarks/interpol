#include "../include/interpol.h"

// TODO: Change calls to `clock_gettime` into `rdtsc` and `sync_rdtscp`
// in `MPI_Init` and `MPI_Finalize`.

int *rank; //pointeur permettant de récupérer le rank du processus

//fonction permettant la récupération des cycles
unsigned long long rdtsc()
{
  unsigned long long a, d;
  
  __asm__ volatile ("rdtsc" : "=a" (a), "=d" (d));
  
  return (d << 32) | a;
}


int MPI_Init(int *argc, char ***argv)
{
  	uint64_t cycles = rdtsc();
    double time = 0.0;
    int ret = PMPI_Init(argc, argv);

    //appel fonction en rust
    register_init(cycles, time);

    rank = -1;

    return ret;
}


