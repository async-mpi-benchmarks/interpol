#include <stdint.h>
#include <stdio.h>
#include <sys/time.h>

#include "mpi.h"
#include "interpol.h"
#include "tsc.h"

// TODO: Change calls to `clock_gettime` into `rdtsc` and `fenced_rdtscp`
// in `MPI_Init` and `MPI_Finalize`.

int proc_rank = -1;

int MPI_Init(int *argc, char ***argv)
{
    // Récupération du temps (on prendra la valeur en microsecondes)
    struct timeval time;
    gettimeofday(&time, NULL);
    // Récupération des cycles
  	uint64_t cycles = fenced_rdtscp();

    int ret = PMPI_Init(argc, argv);

    MPI_Comm_rank(MPI_COMM_WORLD, &proc_rank);

    // Appel fonction en Rust
    register_init(cycles, time.tv_usec);

    return ret;
}

int MPI_Send(const void *buf, int count, MPI_Datatype datatype, int dest,
             int tag, MPI_Comm comm)
{
    // Récupération du nombre bytes envoyés
    int bytes = MPI_Type_size(datatype, &count);
    // Récuperation de la valeur du Comm
    int fcomm = PMPI_Comm_c2f(comm);

    // `rdtsc` et appel de la fonction MPI
  	uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Send(buf, count, datatype, dest, tag, comm);
    uint64_t cycles_hi = rdtsc();

    // Appel de fonction Rust
    register_send(cycles_lo, cycles_hi, (size_t)bytes,
                  fcomm, proc_rank, dest, tag);

    return ret;
}


int MPI_Recv(void *buf, int count, MPI_Datatype datatype, int source, int tag,
             MPI_Comm comm, MPI_Status *status)
{
    // Récupération de nombre de bytes reçus
    int bytes = MPI_Type_size(datatype, &count);
    // Récuperation de la valeur du Comm
    int fcomm = PMPI_Comm_c2f(comm);

    // `rdtsc` et appel de la fonction MPI
    uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Recv(buf, count, datatype, source, tag, comm, status);
    uint64_t cycles_hi = rdtsc();

    // Appel de la fonction Rust
    register_recv(cycles_lo, cycles_hi, (size_t)bytes,
                  fcomm, proc_rank, source, tag);

    return ret;
}

int MPI_Wait(MPI_Request *request, MPI_Status *status)
{
    // Récupération du rank du processus
    // current_rank = MPI_Comm_rank(comm, rank);
    // Récupération de la valeur de la Req
    int req_f = PMPI_Request_c2f(*request);

    uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Wait(request, status);
    uint64_t cycles_hi = rdtsc();

    // Appel de la fonction Rust
    register_wait(cycles_lo, cycles_hi, req_f, proc_rank);

    return ret;
}

int MPI_Isend(const void *buf, int count, MPI_Datatype datatype, int dest,
              int tag, MPI_Comm comm, MPI_Request *request)
{
    // Récupération de nombre de bytes reçus
    int bytes = MPI_Type_size(datatype, &count);
    // Récuperation de la valeur du Comm
    int fcomm = PMPI_Comm_c2f(comm);
    // Récupération de la valeur de la Req
    int req_f = PMPI_Request_c2f(*request);

    uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Isend(buf, count, datatype, dest, tag, comm, request);
    uint64_t cycles_hi = rdtsc();

    // Appel de la fonction Rust
    register_isend(cycles_lo, cycles_hi, (size_t)bytes,
                   fcomm, req_f, proc_rank, dest, tag);

    return ret;
}

int MPI_Irecv(void *buf, int count, MPI_Datatype datatype, int source,
              int tag, MPI_Comm comm, MPI_Request *request)
{
    // Récupération de nombre de bytes reçus
    int bytes = MPI_Type_size(datatype, &count);
    // Récuperation de la valeur du Comm
    int fcomm = PMPI_Comm_c2f(comm);
    // Récupération de la valeur de la Req
    int req_f = PMPI_Request_c2f(*request);

    uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Irecv(buf, count, datatype, source, tag, comm, request);
    uint64_t cycles_hi = rdtsc();

    register_irecv(cycles_lo, cycles_hi, (size_t)bytes, fcomm,
                   req_f, proc_rank, source, tag);

    return ret;
}

int MPI_Finalize()
{
    // Appel de la fonction MPI
    int ret = PMPI_Finalize();

    // Récupération du temps (on prendra la valeur en microsecondes)
    struct timeval time;
    gettimeofday(&time, NULL);

    // Récupération des cycles
    uint64_t cycles = fenced_rdtscp();

    register_finalize(cycles, time.tv_usec, proc_rank);

    return ret;
}
