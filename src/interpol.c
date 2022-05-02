#include "../include/interpol.h"
#include "../include/tsc.h"

#include <mpi.h>
#include <stdint.h>
#include <stdio.h>
#include <sys/time.h>

/// Global variable that stores the rank of the current process.
static MpiRank current_rank = -1;

/** ------------------------------------------------------------------------ **
 * Management functions.                                                      *
 ** ------------------------------------------------------------------------ **/

int MPI_Init(int* argc, char*** argv)
{
    // Measure the current time and TSC.
    struct timeval timeofday;
    gettimeofday(&timeofday, NULL);
    Tsc const tsc = fenced_rdtscp();
    double const time = timeofday.tv_sec + timeofday.tv_usec / 1e6;

    int ret = PMPI_Init(argc, argv);

    // Set the rank of the current MPI process/thread
    PMPI_Comm_rank(MPI_COMM_WORLD, &current_rank);

    register_init(current_rank, tsc, time);
    return ret;
}

int MPI_Init_thread(int* argc, char*** argv, int required, int* provided)
{
    // Measure the current time and TSC.
    struct timeval timeofday;
    gettimeofday(&timeofday, NULL);
    Tsc const tsc = fenced_rdtscp();
    double const time = timeofday.tv_sec + timeofday.tv_usec / 1e6;

    int ret = PMPI_Init_thread(argc, argv, required, provided);

    // Set the rank of the current MPI process/thread
    PMPI_Comm_rank(MPI_COMM_WORLD, &current_rank);

    register_init_thread(current_rank, required, *provided, tsc, time);
    return ret;
}

int MPI_Finalize()
{
    int ret = PMPI_Finalize();

    // Measure the current time and TSC.
    struct timeval timeofday;
    gettimeofday(&timeofday, NULL);
    Tsc const tsc = fenced_rdtscp();
    double const time = timeofday.tv_sec + timeofday.tv_usec / 1e6;

    register_finalize(current_rank, tsc, time);
    return ret;
}

/** ------------------------------------------------------------------------ **
 * Point-to-point functions.                                                  *
 ** ------------------------------------------------------------------------ **/

int MPI_Send(const void* buf, int count, MPI_Datatype datatype, int dest,
             int tag, MPI_Comm comm)
{
    Tsc const tsc = rdtsc();
    int ret = PMPI_Send(buf, count, datatype, dest, tag, comm);
    Tsc const duration = rdtsc() - tsc;

    int nb_bytes;
    PMPI_Type_size(datatype, &nb_bytes);
    nb_bytes *= count;
    MpiComm const comm_f = PMPI_Comm_c2f(comm);

    register_send(current_rank, dest, (uint32_t)nb_bytes, comm_f, tag, tsc, duration);
    return ret;
}

int MPI_Recv(void* buf, int count, MPI_Datatype datatype, int source, int tag,
             MPI_Comm comm, MPI_Status* status)
{
    Tsc const tsc = rdtsc();
    int ret = PMPI_Recv(buf, count, datatype, source, tag, comm, status);
    Tsc const duration = rdtsc() - tsc;

    int nb_bytes;
    PMPI_Type_size(datatype, &nb_bytes);
    nb_bytes *= count;
    MpiComm const comm_f = PMPI_Comm_c2f(comm);

    register_recv(current_rank, source, (uint32_t)nb_bytes, comm_f, tag, tsc, duration);
    return ret;
}

int MPI_Isend(const void* buf, int count, MPI_Datatype datatype, int dest,
              int tag, MPI_Comm comm, MPI_Request* request)
{
    Tsc const tsc = rdtsc();
    int ret = PMPI_Isend(buf, count, datatype, dest, tag, comm, request);
    Tsc const duration = rdtsc() - tsc;

    int nb_bytes;
    PMPI_Type_size(datatype, &nb_bytes);
    nb_bytes *= count;
    MpiComm const comm_f = PMPI_Comm_c2f(comm);
    MpiReq const req_f = PMPI_Request_c2f(*request);

    register_isend(current_rank, dest, (uint32_t)nb_bytes, comm_f, req_f, tag, tsc,
                   duration);
    return ret;
}

int MPI_Irecv(void* buf, int count, MPI_Datatype datatype, int source, int tag,
              MPI_Comm comm, MPI_Request* request)
{
    Tsc const tsc = rdtsc();
    int ret = PMPI_Irecv(buf, count, datatype, source, tag, comm, request);
    Tsc const duration = rdtsc() - tsc;

    int nb_bytes;
    PMPI_Type_size(datatype, &nb_bytes);
    nb_bytes *= count;
    MpiComm const comm_f = PMPI_Comm_c2f(comm);
    MpiReq const req_f = PMPI_Request_c2f(*request);

    register_irecv(current_rank, source, (uint32_t)nb_bytes, comm_f, req_f, tag, tsc,
                   duration);
    return ret;
}

/** ------------------------------------------------------------------------ **
 * Synchronization functions.                                                 *
 ** ------------------------------------------------------------------------ **/

int MPI_Barrier(MPI_Comm comm)
{
    Tsc const tsc = rdtsc();
    int ret = PMPI_Barrier(comm);
    Tsc const duration = rdtsc() - tsc;

    MpiComm const comm_f = PMPI_Comm_c2f(comm);

    register_barrier(current_rank, comm_f, tsc, duration);
    return ret;
}

int MPI_Ibarrier(MPI_Comm comm, MPI_Request* request)
{
    Tsc const tsc = rdtsc();
    int ret = PMPI_Barrier(comm);
    Tsc const duration = rdtsc() - tsc;

    MpiComm const comm_f = PMPI_Comm_c2f(comm);
    MpiReq const req_f = PMPI_Request_c2f(*request);

    register_ibarrier(current_rank, comm_f, req_f, tsc, duration);
    return ret;
}

int MPI_Test(MPI_Request* request, int* flag, MPI_Status* status)
{
    Tsc const tsc = rdtsc();
    int ret = PMPI_Test(request, flag, status);
    Tsc const duration = rdtsc() - tsc;

    MpiReq const req_f = PMPI_Request_c2f(*request);
    bool const finished = flag != 0 ? true : false;

    register_test(current_rank, req_f, finished, tsc, duration);
    return ret;
}

int MPI_Wait(MPI_Request* request, MPI_Status* status)
{
    Tsc const tsc = rdtsc();
    int ret = PMPI_Wait(request, status);
    Tsc const duration = rdtsc() - tsc;

    MpiReq const req_f = PMPI_Request_c2f(*request);

    register_wait(current_rank, req_f, tsc, duration);
    return ret;
}

/** ------------------------------------------------------------------------ **
 * Collective functions.                                                      *
 ** ------------------------------------------------------------------------ **/

// int MPI_Bcast(void* buf, int count, MPI_Datatype datatype, int source,
//               MPI_Comm comm)
// {
//     uint64_t tsc_before = rdtsc();
//     int ret = PMPI_Bcast(buf, count, datatype, source, comm);
//     uint64_t tsc_after = rdtsc();

//     ssize_t bytes;
//     PMPI_Type_size(datatype, &bytes);
//     bytes *= count;
//     int comm_f = PMPI_Comm_c2f(comm);

//     // register_bcast(tsc_before, tsc_after, (size_t) bytes, comm_f,
//     //                proc_current_rank, source);

//     return ret;
// }

// int MPI_Ibcast(void* buf, int count, MPI_Datatype datatype, int source,
//                MPI_Comm comm, MPI_Request* request)
// {
//     uint64_t tsc_before = rdtsc();
//     int ret = PMPI_Ibcast(buf, count, datatype, source, comm, request);
//     uint64_t tsc_after = rdtsc();

//     ssize_t bytes;
//     PMPI_Type_size(datatype, &bytes);
//     bytes *= count;
//     int comm_f = PMPI_Comm_c2f(comm);
//     uint32_t req_f =
//         jenkins_one_at_a_time_hash((char*)request, sizeof(MPI_Request));

//     // register_ibcast(tsc_before, tsc_after, (size_t) bytes, comm_f,
//     //                 req_f, proc_current_rank, source);

//     return ret;
// }

// int MPI_Gather(const void* buf_s, int count_s, MPI_Datatype datatype_s,
//                void* buf_r, int count_r, MPI_Datatype datatype_r, int source,
//                MPI_Comm comm)
// {
//     uint64_t tsc_before = rdtsc();
//     int ret = PMPI_Gather(buf_s, count_s, datatype_s, buf_r, count_r,
//                           datatype_r, source, comm);
//     uint64_t tsc_after = rdtsc();

//     ssize_t bytes_s, bytes_r;
//     PMPI_Type_size(datatype_s, &bytes_s);
//     bytes_s *= count_s;
//     PMPI_Type_size(datatype_r, &bytes_r);
//     bytes_r *= count_r;
//     int comm_f = PMPI_Comm_c2f(comm);

//     // register_gather(tsc_before, tsc_after, (size_t) bytes_s, (size_t)
//     // bytes_r,
//     //                 comm_f, proc_current_rank, source);

//     return ret;
// }

// int MPI_Igather(const void* buf_s, int count_s, MPI_Datatype datatype_s,
//                 void* buf_r, int count_r, MPI_Datatype datatype_r, int
//                 source, MPI_Comm comm, MPI_Request* request)
// {
//     uint64_t tsc_before = rdtsc();
//     int ret = PMPI_Gather(buf_s, count_s, datatype_s, buf_r, count_r,
//                           datatype_r, source, comm);
//     uint64_t tsc_after = rdtsc();

//     ssize_t bytes_s, bytes_r;
//     PMPI_Type_size(datatype_s, &bytes_s);
//     bytes_s *= count_s;
//     PMPI_Type_size(datatype_r, &bytes_r);
//     bytes_r *= count_r;
//     int comm_f = PMPI_Comm_c2f(comm);
//     uint32_t req_f =
//         jenkins_one_at_a_time_hash((char*)request, sizeof(MPI_Request));

//     // register_igather(tsc_before, tsc_after, (size_t) bytes_s, (size_t)
//     // bytes_r,
//     //                 comm_f, req_f, proc_current_rank, source);

//     return ret;
// }
