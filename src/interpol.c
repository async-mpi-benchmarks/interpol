#include <stdint.h>
#include <stdio.h>
#include <sys/time.h>

#include "mpi.h"
#include "interpol.h"
#include "tsc.h"

/// Global variable that stores the rank of the current process.
int proc_rank = -1;

int MPI_Init(int *argc, char ***argv)
{
    struct timeval timeofday;

    // Measure the current time and TSC.
    gettimeofday(&timeofday, NULL);
  	uint64_t cycles = fenced_rdtscp();
  	double time = timeofday.tv_sec + timeofday.tv_usec / 1e6;

    int ret = PMPI_Init(argc, argv);

    // Set the rank of the current process 
    MPI_Comm_rank(MPI_COMM_WORLD, &proc_rank);

    register_init(cycles, time);
    return ret;
}

int MPI_Send(const void *buf, int count, MPI_Datatype datatype, int dest,
             int tag, MPI_Comm comm)
{
    size_t bytes = sizeof(datatype) * count;
    int comm_f = PMPI_Comm_c2f(comm);

  	uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Send(buf, count, datatype, dest, tag, comm);
    uint64_t cycles_hi = rdtsc();

    register_send(cycles_lo, cycles_hi, bytes,
                  comm_f, proc_rank, dest, tag);
    return ret;
}


int MPI_Recv(void *buf, int count, MPI_Datatype datatype, int source, int tag,
             MPI_Comm comm, MPI_Status *status)
{
    size_t bytes = sizeof(datatype) * count;
    int comm_f = PMPI_Comm_c2f(comm);

    uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Recv(buf, count, datatype, source, tag, comm, status);
    uint64_t cycles_hi = rdtsc();

    register_recv(cycles_lo, cycles_hi, bytes,
                  comm_f, proc_rank, source, tag);
    return ret;
}

int MPI_Isend(const void *buf, int count, MPI_Datatype datatype, int dest,
              int tag, MPI_Comm comm, MPI_Request *request)
{
    size_t bytes = sizeof(datatype) * count;
    int comm_f = PMPI_Comm_c2f(comm);
    int req_f = PMPI_Request_c2f(*request);

    uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Isend(buf, count, datatype, dest, tag, comm, request);
    uint64_t cycles_hi = rdtsc();

    register_isend(cycles_lo, cycles_hi, bytes,
                   comm_f, req_f, proc_rank, dest, tag);
    return ret;
}

int MPI_Irecv(void *buf, int count, MPI_Datatype datatype, int source,
              int tag, MPI_Comm comm, MPI_Request *request)
{
    size_t bytes = sizeof(datatype) * count;
    int comm_f = PMPI_Comm_c2f(comm);
    int req_f = PMPI_Request_c2f(*request);

    uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Irecv(buf, count, datatype, source, tag, comm, request);
    uint64_t cycles_hi = rdtsc();

    register_irecv(cycles_lo, cycles_hi, bytes, comm_f,
                   req_f, proc_rank, source, tag);
    return ret;
}

int MPI_Wait(MPI_Request *request, MPI_Status *status)
{
    int req_f = PMPI_Request_c2f(*request);

    uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Wait(request, status);
    uint64_t cycles_hi = rdtsc();

    register_wait(cycles_lo, cycles_hi, req_f, proc_rank);
    return ret;
}

int MPI_Finalize()
{
    struct timeval timeofday;

    int ret = PMPI_Finalize();

    // Measure the current time and TSC.
    gettimeofday(&timeofday, NULL);
    uint64_t cycles = fenced_rdtscp();
  	double time = timeofday.tv_sec + timeofday.tv_usec / 1e6;

    register_finalize(cycles, time, proc_rank);
    return ret;
}
