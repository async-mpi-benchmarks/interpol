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


int MPI_Send(const void *buf, int count, MPI_Datatype datatype, int dest,
             int tag, MPI_Comm comm)
{
    //récupération du nombre bytes envoyé
    int bytes = MPI_Type_size(datatype, &count);
    //récupération du rank du processus
    int current_rank = MPI_Comm_rank(comm, rank);
    //récuperation de la valeur du comm
    int c = comm;

    //rdtsc et appel de la fonction MPI
  	uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Send(buf, count, datatype, dest, tag, comm);
    uint64_t cycles_hi = rdtsc();

    //appel de fonction rust
    register_send(cycles_lo, cycles_hi, (size_t)bytes, c, rank, dest, tag);

    return ret;
}


int MPI_Recv(void *buf, int count, MPI_Datatype datatype, int source, int tag,
             MPI_Comm comm, MPI_Status *status)
{
    //récupération de nombre de bytes reçu
    int bytes = MPI_Type_size(datatype, &count);
    //récupération du rank du processus
    int current_rank = MPI_Comm_rank(comm, rank);
    //récuperation de la valeur du comm
    int c = comm;

    //rdtsc et appel de la fonction MPI
    uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Recv(buf, count, datatype, source, tag, comm, status);
    uint64_t cycles_hi = rdtsc();

    //appel de la fonction rust
    register_recv(cycles_lo, cycles_hi, (size_t)bytes, c, rank, source, tag);

    return ret;
}

int MPI_Wait(MPI_Request *request, MPI_Status *status)
{
    //récupération du rank du processus
    int current_rank = MPI_Comm_rank(comm, rank);
    int r = request;

    uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Wait(request, status);
    uint64_t cycles_hi = rdtsc();

    //appel de la fonction rust
    register_wait(cycles_lo, cycles_hi, r, current_rank);

    return ret;
}

