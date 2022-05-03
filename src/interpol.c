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

    MpiCall init;
    init.call = Init;

    gettimeofday(&timeofday, NULL);

    init.tsc = fenced_rdtscp();
    init.time = timeofday.tv_sec + timeofday.tv_usec / 1e6;

    int ret = PMPI_Init(argc, argv);

    // Set the rank of the current MPI process/thread
    PMPI_Comm_rank(MPI_COMM_WORLD, &current_rank);
    
    
    init.duration = 0;
    init.nb_bytes = 0;
    init.comm = 0;
    init.req = 0;
    init.current_rank = current_rank;
    init.partner_rank = NULL;
    init.tag = -1;
    init.required_thread_lvl = 0;
    init.provided_thread_lvl = 0;
    init.finished = false;

    register_mpi_call(init);
    return ret;
}

int MPI_Init_thread(int* argc, char*** argv, int required, int* provided)
{
    // Measure the current time and TSC.
    struct timeval timeofday;

    MpiCall initthread;
    initthread.call = Initthread;

    gettimeofday(&timeofday, NULL);

    initthread.tsc = fenced_rdtscp();
    initthread.time = timeofday.tv_sec + timeofday.tv_usec / 1e6;

    int ret = PMPI_Init_thread(argc, argv, required, provided);

    // Set the rank of the current MPI process/thread
    PMPI_Comm_rank(MPI_COMM_WORLD, &current_rank);
 
    initthread.duration = 0;
    initthread.nb_bytes = 0;
    initthread.comm = 0;
    initthread.req = 0;
    initthread.current_rank = current_rank;
    initthread.partner_rank = NULL;
    initthread.tag = -1;
    initthread.required_thread_lvl = required;
    initthread.provided_thread_lvl = *provided;
    initthread.finished = false;

    register_mpi_call(initthread);
    return ret;
}

int MPI_Finalize()
{
    MpiCall finalize;
    
    finalize.call = Finalize;

    int ret = PMPI_Finalize();

    // Measure the current time and TSC.
    struct timeval timeofday;
    gettimeofday(&timeofday, NULL);
    finalize.tsc = fenced_rdtscp();
    finalize.time = timeofday.tv_sec + timeofday.tv_usec / 1e6;

    finalize.duration = 0;
    finalize.nb_bytes = 0;
    finalize.comm = 0;
    finalize.req = 0;
    finalize.current_rank = current_rank;
    finalize.partner_rank = NULL;
    finalize.tag = -1;
    finalize.required_thread_lvl = 0;
    finalize.provided_thread_lvl = 0;
    finalize.finished = false;

    register_mpi_call(finalize);
    return ret;
}

/** ------------------------------------------------------------------------ **
 * Point-to-point functions.                                                  *
 ** ------------------------------------------------------------------------ **/

int MPI_Send(const void* buf, int count, MPI_Datatype datatype, int dest,
             int tag, MPI_Comm comm)
{
    MpiCall send;
    
    send.call = Send;
    send.tsc = rdtsc();

    int ret = PMPI_Send(buf, count, datatype, dest, tag, comm);

    send.duration = rdtsc() - send.tsc;

    int nb_bytes;
    PMPI_Type_size(datatype, &nb_bytes);
    send.nb_bytes = nb_bytes * count;
    send.comm = PMPI_Comm_c2f(comm);

    
    send.time = 0.0;
    send.req = 0;
    send.current_rank = current_rank;
    send.partner_rank = &dest;
    send.tag = tag;
    send.required_thread_lvl = 0;
    send.provided_thread_lvl = 0;
    send.finished = false;

    register_mpi_call(send);
    return ret;
}

int MPI_Recv(void* buf, int count, MPI_Datatype datatype, int source, int tag,
             MPI_Comm comm, MPI_Status* status)
{
    MpiCall recv;
    
    recv.call = Recv;
    recv.tsc = rdtsc();

    int ret = PMPI_Recv(buf, count, datatype, source, tag, comm, status);
    
    recv.duration = rdtsc() - recv.tsc;

    int nb_bytes;
    PMPI_Type_size(datatype, &nb_bytes);
    recv.nb_bytes = nb_bytes * count;
    recv.comm = PMPI_Comm_c2f(comm);

    recv.time = 0.0;
    recv.req = 0;
    recv.current_rank = current_rank;
    recv.partner_rank = &source;
    recv.tag = tag;
    recv.required_thread_lvl = 0;
    recv.provided_thread_lvl = 0;
    recv.finished = false;

    register_mpi_call(recv);
    return ret;
}

int MPI_Isend(const void* buf, int count, MPI_Datatype datatype, int dest,
              int tag, MPI_Comm comm, MPI_Request* request)
{
    MpiCall isend;
    
    isend.call = Isend;
    isend.tsc = rdtsc();

    int ret = PMPI_Isend(buf, count, datatype, dest, tag, comm, request);

    isend.duration = rdtsc() - isend.tsc;

    int nb_bytes;
    PMPI_Type_size(datatype, &nb_bytes);
    isend.nb_bytes = nb_bytes*count;
    isend.comm = PMPI_Comm_c2f(comm);
    isend.req = PMPI_Request_c2f(*request);

    isend.time = 0.0;
    isend.current_rank = current_rank;
    isend.partner_rank = &dest;
    isend.tag = tag;
    isend.required_thread_lvl = 0;
    isend.provided_thread_lvl = 0;
    isend.finished = false;

    register_mpi_call(isend);
    return ret;
}

int MPI_Irecv(void* buf, int count, MPI_Datatype datatype, int source, int tag,
              MPI_Comm comm, MPI_Request* request)
{
    MpiCall irecv;
    
    irecv.call = Irecv;
    irecv.tsc = rdtsc();

    int ret = PMPI_Irecv(buf, count, datatype, source, tag, comm, request);

    irecv.duration = rdtsc() - irecv.tsc;

    int nb_bytes;
    PMPI_Type_size(datatype, &nb_bytes);
    irecv.nb_bytes = nb_bytes*count;
    irecv.comm = PMPI_Comm_c2f(comm);
    irecv.req = PMPI_Request_c2f(*request);

    irecv.time = 0.0;
    irecv.current_rank = current_rank;
    irecv.partner_rank = &source;
    irecv.tag = tag;
    irecv.required_thread_lvl = 0;
    irecv.provided_thread_lvl = 0;
    irecv.finished = false;

    register_mpi_call(irecv);
    return ret;
}

/** ------------------------------------------------------------------------ **
 * Synchronization functions.                                                 *
 ** ------------------------------------------------------------------------ **/

int MPI_Barrier(MPI_Comm comm)
{
    MpiCall barrier;
    
    barrier.call = Barrier;
    barrier.tsc = rdtsc();

    int ret = PMPI_Barrier(comm);

    barrier.duration = rdtsc() - barrier.tsc;

    barrier.comm = PMPI_Comm_c2f(comm);

    barrier.req = 0;
    barrier.nb_bytes = 0;
    barrier.time = 0.0;
    barrier.current_rank = current_rank;
    barrier.partner_rank = NULL;
    barrier.tag = -1;
    barrier.required_thread_lvl = 0;
    barrier.provided_thread_lvl = 0;
    barrier.finished = false;

    register_mpi_call(barrier);
    return ret;
}

int MPI_Test(MPI_Request* request, int* flag, MPI_Status* status)
{
    MpiCall test;
    
    test.call = Test;
    test.tsc = rdtsc();

    int ret = PMPI_Test(request, flag, status);

    test.duration = rdtsc() - test.tsc;

    test.req = PMPI_Request_c2f(*request);
    test.finished = flag != 0 ? true : false;

    test.comm = -1;
    test.nb_bytes = 0;
    test.time = 0.0;
    test.current_rank = current_rank;
    test.partner_rank = NULL;
    test.tag = -1;
    test.required_thread_lvl = 0;
    test.provided_thread_lvl = 0;

    register_mpi_call(test);
    return ret;
}

int MPI_Wait(MPI_Request* request, MPI_Status* status)
{
    MpiCall wait;
    
    wait.call = Wait;
    wait.tsc = rdtsc();

    int ret = PMPI_Wait(request, status);

    wait.duration = rdtsc() - wait.tsc;

    wait.req = PMPI_Request_c2f(*request);

    wait.comm = -1;
    wait.nb_bytes = 0;
    wait.time = 0.0;
    wait.current_rank = current_rank;
    wait.partner_rank = NULL;
    wait.tag = -1;
    wait.required_thread_lvl = 0;
    wait.provided_thread_lvl = 0;
    wait.finished = false;

    register_mpi_call(wait);
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
