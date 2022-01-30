#include <stdint.h>
#include <stdio.h>
#include <sys/time.h>

#include "mpi.h"
#include "interpol.h"
#include "tsc.h"

/// Global variable that stores the rank of the current process.
int proc_rank = -1;

/// Hashing function to generate a unique value for each `MPI_Request` object
uint32_t jenkins_one_at_a_time_hash(char *key, size_t len)
{
    uint32_t hash = 0;

    for (size_t i = 0; i < len; i++) {
        hash += key[i];
        hash += (hash << 10);
        hash ^= (hash >> 6);
    }

    hash += (hash << 3);
    hash ^= (hash >> 11);
    hash += (hash << 15);

    return hash;
}

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
  	uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Send(buf, count, datatype, dest, tag, comm);
    uint64_t cycles_hi = rdtsc();

    ssize_t bytes;
    PMPI_Type_size(datatype, &bytes);
    bytes *= count;
    int comm_f = PMPI_Comm_c2f(comm);

    register_send(cycles_lo, cycles_hi, (size_t)bytes,
                  comm_f, proc_rank, dest, tag);
    return ret;
}

int MPI_Recv(void *buf, int count, MPI_Datatype datatype, int source, int tag,
             MPI_Comm comm, MPI_Status *status)
{
    uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Recv(buf, count, datatype, source, tag, comm, status);
    uint64_t cycles_hi = rdtsc();

    ssize_t bytes;
    MPI_Type_size(datatype, &bytes);
    bytes *= count;
    int comm_f = PMPI_Comm_c2f(comm);

    register_recv(cycles_lo, cycles_hi, (size_t)bytes,
                  comm_f, proc_rank, source, tag);
    return ret;
}

int MPI_Isend(const void *buf, int count, MPI_Datatype datatype, int dest,
              int tag, MPI_Comm comm, MPI_Request *request)
{
    uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Isend(buf, count, datatype, dest, tag, comm, request);
    uint64_t cycles_hi = rdtsc();

    ssize_t bytes;
    PMPI_Type_size(datatype, &bytes);
    bytes *= count;
    int comm_f = PMPI_Comm_c2f(comm);
    uint32_t req_f = jenkins_one_at_a_time_hash((char *)request, sizeof(MPI_Request));

    register_isend(cycles_lo, cycles_hi, (size_t)bytes,
                   comm_f, req_f, proc_rank, dest, tag);
    return ret;
}

int MPI_Irecv(void *buf, int count, MPI_Datatype datatype, int source,
              int tag, MPI_Comm comm, MPI_Request *request)
{
    uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Irecv(buf, count, datatype, source, tag, comm, request);
    uint64_t cycles_hi = rdtsc();

    ssize_t bytes;
    PMPI_Type_size(datatype, &bytes);
    bytes *= count;
    int comm_f = PMPI_Comm_c2f(comm);
    uint32_t req_f = jenkins_one_at_a_time_hash((char *)request, sizeof(MPI_Request));

    register_irecv(cycles_lo, cycles_hi, (size_t)bytes, comm_f,
                   req_f, proc_rank, source, tag);
    return ret;
}

int MPI_Wait(MPI_Request *request, MPI_Status *status)
{
    uint32_t req_f = jenkins_one_at_a_time_hash((char *)request, sizeof(MPI_Request));

    uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Wait(request, status);
    uint64_t cycles_hi = rdtsc();

    register_wait(cycles_lo, cycles_hi, req_f, proc_rank);
    return ret;
}

int MPI_Bcast(void* buf, int count, MPI_Datatype datatype, int source, MPI_Comm comm)
{
    uint64_t cycles_lo = rdtsc();
    int ret = PMPI_Bcast(buf, count, datatype, source, comm);
    uint64_t cycles_hi = rdtsc();

    ssize_t bytes;
    PMPI_Type_size(datatype, &bytes);
    bytes *= count;
    int comm_f = PMPI_Comm_c2f(comm);

    register_bcast(cycles_lo, cycles_hi, (size_t) bytes, comm_f, 
                   proc_rank, source);

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
