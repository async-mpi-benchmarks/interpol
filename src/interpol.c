#include "interpol.h"
#include "tsc.h"

#include <mpi.h>
#include <stdint.h>
#include <stdio.h>
#include <sys/time.h>

/// Global variable that stores the rank of the current process.
static MpiRank current_rank = -1;

int32_t jenkins_one_at_a_time_hash(char const* key, size_t len)
{
    int32_t hash = 0;

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

/** ------------------------------------------------------------------------ **
 * Management functions.                                                      *
 ** ------------------------------------------------------------------------ **/

int MPI_Init(int* argc, char*** argv)
{
    int ret = PMPI_Init(argc, argv);
    PMPI_Barrier(MPI_COMM_WORLD);

    // Measure the current time and TSC.
    Tsc const tsc = fenced_rdtscp();
    struct timeval timeofday;
    gettimeofday(&timeofday, NULL);

    // Set the rank of the current MPI process/thread
    PMPI_Comm_rank(MPI_COMM_WORLD, &current_rank);

    MpiCall const init = {
        .kind = Init,
        .time = timeofday.tv_sec + timeofday.tv_usec / 1e6,
        .tsc = tsc,
        .duration = 0,
        .current_rank = current_rank,
        .partner_rank = -1,
        .nb_bytes_s = 0,
        .nb_bytes_r = 0,
        .comm = -1,
        .req = -1,
        .tag = -1,
        .required_thread_lvl = -1,
        .provided_thread_lvl = -1,
        .op_type = -1,
        .finished = false,
    };

    register_mpi_call(init);
    return ret;
}

int MPI_Init_thread(int* argc, char*** argv, int required, int* provided)
{
    int ret = PMPI_Init_thread(argc, argv, required, provided);
    PMPI_Barrier(MPI_COMM_WORLD);

    // Measure the current time and TSC.
    Tsc const tsc = fenced_rdtscp();
    struct timeval timeofday;
    gettimeofday(&timeofday, NULL);

    // Set the rank of the current MPI process/thread
    PMPI_Comm_rank(MPI_COMM_WORLD, &current_rank);

    MpiCall const initthread = {
        .kind = Initthread,
        .time = timeofday.tv_sec + timeofday.tv_usec / 1e6,
        .tsc = tsc,
        .duration = 0,
        .current_rank = current_rank,
        .partner_rank = -1,
        .nb_bytes_s = 0,
        .nb_bytes_r = 0,
        .comm = -1,
        .req = -1,
        .tag = -1,
        .required_thread_lvl = required,
        .provided_thread_lvl = *provided,
        .op_type = -1,
        .finished = false,
    };

    register_mpi_call(initthread);
    return ret;
}

int MPI_Finalize()
{
    PMPI_Barrier(MPI_COMM_WORLD);
    // Measure the current time and TSC.
    Tsc const tsc = fenced_rdtscp();
    struct timeval timeofday;
    gettimeofday(&timeofday, NULL);

    MpiCall const finalize = {
        .kind = Finalize,
        .time = timeofday.tv_sec + timeofday.tv_usec / 1e6,
        .tsc = tsc,
        .duration = 0,
        .current_rank = current_rank,
        .partner_rank = -1,
        .nb_bytes_s = 0,
        .nb_bytes_r = 0,
        .comm = -1,
        .req = -1,
        .tag = -1,
        .required_thread_lvl = -1,
        .provided_thread_lvl = -1,
        .op_type = -1,
        .finished = false,
    };

    register_mpi_call(finalize);

    PMPI_Barrier(MPI_COMM_WORLD);
    if (current_rank == 0) {
        sort_all_traces();
    }

    int ret = PMPI_Finalize();
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

    MpiCall const send = {
        .kind = Send,
        .time = -1.0,
        .tsc = tsc,
        .duration = duration,
        .current_rank = current_rank,
        .partner_rank = dest,
        .nb_bytes_s = nb_bytes * count,
        .nb_bytes_r = 0,
        .comm = PMPI_Comm_c2f(comm),
        .req = -1,
        .tag = tag,
        .required_thread_lvl = -1,
        .provided_thread_lvl = -1,
        .op_type = -1,
        .finished = false,
    };

    register_mpi_call(send);
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

    MpiCall const recv = {
        .kind = Recv,
        .time = -1.0,
        .tsc = tsc,
        .duration = duration,
        .current_rank = current_rank,
        .partner_rank = source,
        .nb_bytes_s = 0,
        .nb_bytes_r = nb_bytes * count,
        .comm = PMPI_Comm_c2f(comm),
        .req = -1,
        .tag = tag,
        .required_thread_lvl = -1,
        .provided_thread_lvl = -1,
        .op_type = -1,
        .finished = false,
    };

    register_mpi_call(recv);
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

    MpiCall const isend = {
        .kind = Isend,
        .time = -1.0,
        .tsc = tsc,
        .duration = duration,
        .current_rank = current_rank,
        .partner_rank = dest,
        .nb_bytes_s = nb_bytes * count,
        .nb_bytes_r = 0,
        .comm = PMPI_Comm_c2f(comm),
        .req = jenkins_one_at_a_time_hash((char*)request, sizeof(MPI_Request)),
        .tag = tag,
        .required_thread_lvl = -1,
        .provided_thread_lvl = -1,
        .op_type = -1,
        .finished = false,
    };

    register_mpi_call(isend);
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

    MpiCall const irecv = {
        .kind = Irecv,
        .time = -1.0,
        .tsc = tsc,
        .duration = duration,
        .current_rank = current_rank,
        .partner_rank = source,
        .nb_bytes_s = 0,
        .nb_bytes_r = nb_bytes * count,
        .comm = PMPI_Comm_c2f(comm),
        .req = jenkins_one_at_a_time_hash((char*)request, sizeof(MPI_Request)),
        .tag = tag,
        .required_thread_lvl = -1,
        .provided_thread_lvl = -1,
        .op_type = -1,
        .finished = false,
    };

    register_mpi_call(irecv);
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

    MpiCall const barrier = {
        .kind = Barrier,
        .time = -1.0,
        .tsc = tsc,
        .duration = duration,
        .current_rank = current_rank,
        .partner_rank = -1,
        .nb_bytes_s = 0,
        .nb_bytes_r = 0,
        .comm = PMPI_Comm_c2f(comm),
        .req = -1,
        .tag = -1,
        .required_thread_lvl = -1,
        .provided_thread_lvl = -1,
        .op_type = -1,
        .finished = false,
    };

    register_mpi_call(barrier);
    return ret;
}

int MPI_Ibarrier(MPI_Comm comm, MPI_Request* request)
{
    Tsc const tsc = rdtsc();
    int ret = PMPI_Ibarrier(comm, request);
    Tsc const duration = rdtsc() - tsc;

    MpiCall const ibarrier = {
        .kind = Ibarrier,
        .time = -1.0,
        .tsc = tsc,
        .duration = duration,
        .current_rank = current_rank,
        .partner_rank = -1,
        .nb_bytes_s = 0,
        .nb_bytes_r = 0,
        .comm = PMPI_Comm_c2f(comm),
        .req = jenkins_one_at_a_time_hash((char*)request, sizeof(MPI_Request)),
        .tag = -1,
        .required_thread_lvl = -1,
        .provided_thread_lvl = -1,
        .op_type = -1,
        .finished = false,
    };

    register_mpi_call(ibarrier);
    return ret;
}

int MPI_Test(MPI_Request* request, int* flag, MPI_Status* status)
{
    Tsc const tsc = rdtsc();
    int ret = PMPI_Test(request, flag, status);
    Tsc const duration = rdtsc() - tsc;

    MpiCall const test = {
        .kind = Test,
        .time = -1.0,
        .tsc = tsc,
        .duration = duration,
        .current_rank = current_rank,
        .partner_rank = -1,
        .nb_bytes_s = 0,
        .nb_bytes_r = 0,
        .comm = -1,
        .req = jenkins_one_at_a_time_hash((char*)request, sizeof(MPI_Request)),
        .tag = -1,
        .required_thread_lvl = -1,
        .provided_thread_lvl = -1,
        .op_type = -1,
        .finished = *flag != 0 ? true : false,
    };

    register_mpi_call(test);
    return ret;
}

int MPI_Wait(MPI_Request* request, MPI_Status* status)
{
    Tsc const tsc = rdtsc();
    int ret = PMPI_Wait(request, status);
    Tsc const duration = rdtsc() - tsc;

    MpiCall const wait = {
        .kind = Wait,
        .time = -1.0,
        .tsc = tsc,
        .duration = duration,
        .current_rank = current_rank,
        .partner_rank = -1,
        .nb_bytes_s = 0,
        .nb_bytes_r = 0,
        .comm = -1,
        .req = jenkins_one_at_a_time_hash((char*)request, sizeof(MPI_Request)),
        .tag = -1,
        .required_thread_lvl = -1,
        .provided_thread_lvl = -1,
        .op_type = -1,
        .finished = false,
    };

    register_mpi_call(wait);
    return ret;
}

/** ------------------------------------------------------------------------ **
 * Collective functions.                                                      *
 ** ------------------------------------------------------------------------ **/

int MPI_Ibcast(void* buf, int count, MPI_Datatype datatype, int root,
               MPI_Comm comm, MPI_Request* request)
{
    Tsc const tsc = rdtsc();
    int ret = PMPI_Ibcast(buf, count, datatype, root, comm, request);
    Tsc const duration = rdtsc() - tsc;

    int nb_bytes;
    PMPI_Type_size(datatype, &nb_bytes);

    MpiCall const ibcast = {
        .kind = Ibcast,
        .time = -1.0,
        .tsc = tsc,
        .duration = duration,
        .current_rank = current_rank,
        .partner_rank = root,
        .nb_bytes_s = nb_bytes * count,
        .nb_bytes_r = 0,
        .comm = PMPI_Comm_c2f(comm),
        .req = jenkins_one_at_a_time_hash((char*)request, sizeof(MPI_Request)),
        .tag = -1,
        .required_thread_lvl = -1,
        .provided_thread_lvl = -1,
        .op_type = -1,
        .finished = false,
    };

    register_mpi_call(ibcast);
    return ret;
}

int MPI_Igather(void const* sendbuf, int sendcount, MPI_Datatype sendtype,
                void* recvbuf, int recvcount, MPI_Datatype recvtype, int root,
                MPI_Comm comm, MPI_Request* request)
{
    Tsc const tsc = rdtsc();
    int ret = PMPI_Igather(sendbuf, sendcount, sendtype, recvbuf, recvcount,
                           recvtype, root, comm, request);
    Tsc const duration = rdtsc() - tsc;

    int nb_bytes_send, nb_bytes_recv;
    PMPI_Type_size(sendtype, &nb_bytes_send);
    PMPI_Type_size(recvtype, &nb_bytes_recv);

    MpiCall const igather = {
        .kind = Igather,
        .time = -1.0,
        .tsc = tsc,
        .duration = duration,
        .current_rank = current_rank,
        .partner_rank = root,
        .nb_bytes_s = nb_bytes_send * sendcount,
        .nb_bytes_r = nb_bytes_recv * recvcount,
        .comm = PMPI_Comm_c2f(comm),
        .req = jenkins_one_at_a_time_hash((char*)request, sizeof(MPI_Request)),
        .tag = -1,
        .required_thread_lvl = -1,
        .provided_thread_lvl = -1,
        .op_type = -1,
        .finished = false,
    };

    register_mpi_call(igather);
    return ret;
}

int MPI_Iscatter(void const* sendbuf, int sendcount, MPI_Datatype sendtype,
                 void* recvbuf, int recvcount, MPI_Datatype recvtype, int root,
                 MPI_Comm comm, MPI_Request* request)
{
    Tsc const tsc = rdtsc();
    int ret = PMPI_Iscatter(sendbuf, sendcount, sendtype, recvbuf, recvcount,
                            recvtype, root, comm, request);
    Tsc const duration = rdtsc() - tsc;

    int nb_bytes_send, nb_bytes_recv;
    PMPI_Type_size(sendtype, &nb_bytes_send);
    PMPI_Type_size(recvtype, &nb_bytes_recv);

    MpiCall const iscatter = {
        .kind = Iscatter,
        .time = -1.0,
        .tsc = tsc,
        .duration = duration,
        .current_rank = current_rank,
        .partner_rank = root,
        .nb_bytes_s = nb_bytes_send * sendcount,
        .nb_bytes_r = nb_bytes_recv * recvcount,
        .comm = PMPI_Comm_c2f(comm),
        .req = jenkins_one_at_a_time_hash((char*)request, sizeof(MPI_Request)),
        .tag = -1,
        .required_thread_lvl = -1,
        .provided_thread_lvl = -1,
        .op_type = -1,
        .finished = false,
    };

    register_mpi_call(iscatter);
    return ret;
}

int MPI_Ireduce(const void* sendbuf, void* recvbuf, int count,
                MPI_Datatype datatype, MPI_Op op, int root, MPI_Comm comm,
                MPI_Request* request)
{
    Tsc const tsc = rdtsc();
    int ret = PMPI_Ireduce(sendbuf, recvbuf, count, datatype, op, root, comm,
                           request);
    Tsc const duration = rdtsc() - tsc;

    int nb_bytes;
    PMPI_Type_size(datatype, &nb_bytes);

    MpiOp op_type;
    if (op == MPI_MAX) {
        op_type = Max;
    } else if (op == MPI_MIN) {
        op_type = Min;
    } else if (op == MPI_SUM) {
        op_type = Sum;
    } else if (op == MPI_PROD) {
        op_type = Prod;
    } else if (op == MPI_LAND) {
        op_type = Land;
    } else if (op == MPI_BAND) {
        op_type = Band;
    } else if (op == MPI_LOR) {
        op_type = Lor;
    } else if (op == MPI_BOR) {
        op_type = Bor;
    } else if (op == MPI_LXOR) {
        op_type = Lxor;
    } else if (op == MPI_BXOR) {
        op_type = Bxor;
    } else if (op == MPI_MAXLOC) {
        op_type = Maxloc;
    } else if (op == MPI_MINLOC) {
        op_type = Minloc;
    } else if (op == MPI_REPLACE) {
        op_type = Replace;
    } else if (op == MPI_OP_NULL) {
        op_type = Opnull;
    } else if (op == MPI_NO_OP) {
        op_type = Opnull;
    } else {
        op_type = -1;
    }

    MpiCall const ireduce = {
        .kind = Ireduce,
        .time = -1.0,
        .tsc = tsc,
        .duration = duration,
        .current_rank = current_rank,
        .partner_rank = root,
        .nb_bytes_s = nb_bytes,
        .nb_bytes_r = nb_bytes * count,
        .comm = PMPI_Comm_c2f(comm),
        .req = jenkins_one_at_a_time_hash((char*)request, sizeof(MPI_Request)),
        .tag = -1,
        .required_thread_lvl = -1,
        .provided_thread_lvl = -1,
        .op_type = op_type,
        .finished = false,
    };

    register_mpi_call(ireduce);
    return ret;
}
