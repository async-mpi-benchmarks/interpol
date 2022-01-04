#include <stdint.h>
#include <stdio.h>
#include <sys/time.h>

#include "mpi.h"
#include "interpol.h"
#include "sync.h"

// TODO: Change calls to `clock_gettime` into `rdtsc` and `sync_rdtscp`
// in `MPI_Init` and `MPI_Finalize`.

// Valeur du rank
int current_rank;
// Pointeur permettant de récupérer le rank du processus
int *rank;

int MPI_Init(int *argc, char ***argv)
{
    // Récupération du temps (on prendra la valeur en microsecondes)
    struct timeval time;
    gettimeofday(&time, NULL);
    // Récupération des cycles
  	uint64_t cycles = sync_rdtscp();

    int ret = PMPI_Init(argc, argv);

    // Appel fonction en Rust
    register_init(cycles, time.tv_usec);

    current_rank = -1;

    return ret;
}

int MPI_Send(const void *buf, int count, MPI_Datatype datatype, int dest,
             int tag, MPI_Comm comm)
{
    // Récupération du nombre bytes envoyés
    int bytes = MPI_Type_size(datatype, &count);
    // Récupération du rank du processus
    current_rank = MPI_Comm_rank(comm, rank);
    // Récuperation de la valeur du Comm
    int fcomm = PMPI_Comm_c2f(comm);

    // `rdtsc` et appel de la fonction MPI
  	uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Send(buf, count, datatype, dest, tag, comm);
    uint64_t cycles_hi = rdtsc();

    // Appel de fonction Rust
    register_send(cycles_lo, cycles_hi, (size_t)bytes,
                  fcomm, current_rank, dest, tag);

    return ret;
}


int MPI_Recv(void *buf, int count, MPI_Datatype datatype, int source, int tag,
             MPI_Comm comm, MPI_Status *status)
{
    // Récupération de nombre de bytes reçus
    int bytes = MPI_Type_size(datatype, &count);
    // Récupération du rank du processus
    current_rank = MPI_Comm_rank(comm, rank);
    // Récuperation de la valeur du Comm
    int fcomm = PMPI_Comm_c2f(comm);

    // `rdtsc` et appel de la fonction MPI
    uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Recv(buf, count, datatype, source, tag, comm, status);
    uint64_t cycles_hi = rdtsc();

    // Appel de la fonction Rust
    register_recv(cycles_lo, cycles_hi, (size_t)bytes,
                  fcomm, current_rank, source, tag);

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
    register_wait(cycles_lo, cycles_hi, req_f, current_rank);

    return ret;
}

int MPI_Isend(const void *buf, int count, MPI_Datatype datatype, int dest,
              int tag, MPI_Comm comm, MPI_Request *request)
{
    // Récupération de nombre de bytes reçus
    int bytes = MPI_Type_size(datatype, &count);
    // Récupération du rank du processus
    current_rank = MPI_Comm_rank(comm, rank);
    // Récuperation de la valeur du Comm
    int fcomm = PMPI_Comm_c2f(comm);
    // Récupération de la valeur de la Req
    int req_f = PMPI_Request_c2f(*request);

    uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Isend(buf, count, datatype, dest, tag, comm, request);
    uint64_t cycles_hi = rdtsc();

    // Appel de la fonction Rust
    register_isend(cycles_lo, cycles_hi, (size_t)bytes,
                   fcomm, req_f, current_rank, dest, tag);

    return ret;
}

int MPI_Irecv(void *buf, int count, MPI_Datatype datatype, int source,
              int tag, MPI_Comm comm, MPI_Request *request)
{
    // Récupération de nombre de bytes reçus
    int bytes = MPI_Type_size(datatype, &count);
    // Récupération du rank du processus
    current_rank = MPI_Comm_rank(comm, rank);
    // Récuperation de la valeur du Comm
    int fcomm = PMPI_Comm_c2f(comm);
    // Récupération de la valeur de la Req
    int req_f = PMPI_Request_c2f(*request);

    uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Irecv(buf, count, datatype, source, tag, comm, request);
    uint64_t cycles_hi = rdtsc();

    register_irecv(cycles_lo, cycles_hi, (size_t)bytes, fcomm,
                   req_f, current_rank, source, tag);

    return ret;
}

int MPI_Finalize()
{
    // Appel de la fonction MPI
    int ret = PMPI_Finalize();

    // Vérification qu'un appel a été fait entre le init et le finalize
    if (*rank != -1) {
        // Récupération du temps (on prendra la valeur en microsecondes)
        struct timeval time;
        gettimeofday(&time, NULL);

        // Récupération des cycles
        uint64_t cycles = rdtsc();

        register_finalize(cycles, time.tv_usec, current_rank);

        return ret;
    } else {
        fprintf(stderr, "Aucun appel entre `MPI_Init` et `MPI_Finalize`\n");
        return ret;
    }
    return ret;
}
