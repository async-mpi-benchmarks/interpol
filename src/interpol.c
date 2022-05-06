#include "../include/interpol.h"
#include "../include/tsc.h"

#include <mpi.h>
#include <stdint.h>
#include <stdio.h>
#include <sys/time.h>

/// Global variable that stores the rank of the current process.
static MpiRank current_rank = -1;

uint32_t jenkins_one_at_a_time_hash(char *key, size_t len) {
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

/** ------------------------------------------------------------------------ **
 * Management functions.                                                      *
 ** ------------------------------------------------------------------------ **/

int MPI_Init(int *argc, char ***argv) {
  // Measure the current time and TSC.
  struct timeval timeofday;
  gettimeofday(&timeofday, NULL);

  int ret = PMPI_Init(argc, argv);

  // Set the rank of the current MPI process/thread
  PMPI_Comm_rank(MPI_COMM_WORLD, &current_rank);

  MpiCall const init = {
      .kind = Init,
      .time = timeofday.tv_sec + timeofday.tv_usec / 1e6,
      .tsc = fenced_rdtscp(),
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

int MPI_Init_thread(int *argc, char ***argv, int required, int *provided) {
  // Measure the current time and TSC.
  struct timeval timeofday;
  gettimeofday(&timeofday, NULL);

  int ret = PMPI_Init_thread(argc, argv, required, provided);

  // Set the rank of the current MPI process/thread
  PMPI_Comm_rank(MPI_COMM_WORLD, &current_rank);

  MpiCall const initthread = {
      .kind = Initthread,
      .time = timeofday.tv_sec + timeofday.tv_usec / 1e6,
      .tsc = fenced_rdtscp(),
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

int MPI_Finalize() {
  // Measure the current time and TSC.
  struct timeval timeofday;
  gettimeofday(&timeofday, NULL);

  MpiCall const finalize = {
      .kind = Finalize,
      .time = timeofday.tv_sec + timeofday.tv_usec / 1e6,
      .tsc = fenced_rdtscp(),
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

  if (current_rank == 0)
    sort_all_traces();

  int ret = PMPI_Finalize();
  return ret;
}

/** ------------------------------------------------------------------------ **
 * Point-to-point functions.                                                  *
 ** ------------------------------------------------------------------------ **/

int MPI_Send(const void *buf, int count, MPI_Datatype datatype, int dest,
             int tag, MPI_Comm comm) {
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

int MPI_Recv(void *buf, int count, MPI_Datatype datatype, int source, int tag,
             MPI_Comm comm, MPI_Status *status) {
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

int MPI_Isend(const void *buf, int count, MPI_Datatype datatype, int dest,
              int tag, MPI_Comm comm, MPI_Request *request) {
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
      .req = jenkins_one_at_a_time_hash((char *)request, sizeof(MPI_Request)),
      .tag = tag,
      .required_thread_lvl = -1,
      .provided_thread_lvl = -1,
      .op_type = -1,
      .finished = false,
  };

  register_mpi_call(isend);
  return ret;
}

int MPI_Irecv(void *buf, int count, MPI_Datatype datatype, int source, int tag,
              MPI_Comm comm, MPI_Request *request) {
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
      .req = jenkins_one_at_a_time_hash((char *)request, sizeof(MPI_Request)),
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

int MPI_Barrier(MPI_Comm comm) {
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

int MPI_Test(MPI_Request *request, int *flag, MPI_Status *status) {
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
      .req = jenkins_one_at_a_time_hash((char *)request, sizeof(MPI_Request)),
      .tag = -1,
      .required_thread_lvl = -1,
      .provided_thread_lvl = -1,
      .op_type = -1,
      .finished = *flag != 0 ? true : false,
  };

  register_mpi_call(test);
  return ret;
}

int MPI_Wait(MPI_Request *request, MPI_Status *status) {
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
      .req = jenkins_one_at_a_time_hash((char *)request, sizeof(MPI_Request)),
      .tag = -1,
      .required_thread_lvl = -1,
      .provided_thread_lvl = -1,
      .op_type = -1,
      .finished = false,
  };

  register_mpi_call(wait);
  return ret;
}

int MPI_Ibarrier(MPI_Comm comm, MPI_Request *request) {
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
      .req = jenkins_one_at_a_time_hash((char *)request, sizeof(MPI_Request)),
      .tag = -1,
      .required_thread_lvl = -1,
      .provided_thread_lvl = -1,
      .op_type = -1,
      .finished = false,
  };

  register_mpi_call(ibarrier);
  return ret;
}

/** ------------------------------------------------------------------------ **
 * Collective functions.                                                      *
 ** ------------------------------------------------------------------------ **/

int MPI_Ibcast(void *buf, int count, MPI_Datatype datatype, int root,
               MPI_Comm comm, MPI_Request *request) {
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
      .req = jenkins_one_at_a_time_hash((char *)request, sizeof(MPI_Request)),
      .tag = -1,
      .required_thread_lvl = -1,
      .provided_thread_lvl = -1,
      .op_type = -1,
      .finished = false,
  };

  register_mpi_call(ibcast);
  return ret;
}

int MPI_Igather(void const *sendbuf, int sendcount, MPI_Datatype sendtype,
                void *recvbuf, int recvcount, MPI_Datatype recvtype, int root,
                MPI_Comm comm, MPI_Request *request) {
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
      .req = jenkins_one_at_a_time_hash((char *)request, sizeof(MPI_Request)),
      .tag = -1,
      .required_thread_lvl = -1,
      .provided_thread_lvl = -1,
      .op_type = -1,
      .finished = false,
  };

  register_mpi_call(igather);
  return ret;
}

int MPI_Iscatter(void const *sendbuf, int sendcount, MPI_Datatype sendtype,
                 void *recvbuf, int recvcount, MPI_Datatype recvtype, int root,
                 MPI_Comm comm, MPI_Request *request) {
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
      .req = jenkins_one_at_a_time_hash((char *)request, sizeof(MPI_Request)),
      .tag = -1,
      .required_thread_lvl = -1,
      .provided_thread_lvl = -1,
      .op_type = -1,
      .finished = false,
  };

  register_mpi_call(iscatter);
  return ret;
}

int MPI_Ireduce(const void *sendbuf, void *recvbuf, int count,
                MPI_Datatype datatype, MPI_Op op, int root, MPI_Comm comm,
                MPI_Request *request) {
  Tsc const tsc = rdtsc();

  int ret =
      PMPI_Ireduce(sendbuf, recvbuf, count, datatype, op, root, comm, request);

  Tsc const duration = rdtsc() - tsc;

  int nb_bytes;
  PMPI_Type_size(datatype, &nb_bytes);

  // int verif = PMPI_Op_c2f(op);
  MPIOp op_type;
  // initialisé à null si il n'y a pas de correspondance
  op_type = MPIOPNULL;

  if (op == MPI_MAX) {
    op_type = MPIMAX;
  } else if (op == MPI_MIN) {
    op_type = MPIMIN;
  } else if (op == MPI_SUM) {
    op_type = MPISUM;
  } else if (op == MPI_PROD) {
    op_type = MPIPROD;
  } else if (op == MPI_LAND) {
    op_type = MPILAND;
  } else if (op == MPI_BAND) {
    op_type = MPIBAND;
  } else if (op == MPI_LOR) {
    op_type = MPILOR;
  } else if (op == MPI_BOR) {
    op_type = MPIBOR;
  } else if (op == MPI_LXOR) {
    op_type = MPILXOR;
  } else if (op == MPI_BXOR) {
    op_type = MPIBXOR;
  } else if (op == MPI_MINLOC) {
    op_type = MPIMINLOC;
  } else if (op == MPI_MAXLOC) {
    op_type = MPIMAXLOC;
  } else if (op == MPI_REPLACE) {
    op_type = MPIREPLACE;
  } else if (op == MPI_OP_NULL) {
    op_type = MPIOPNULL;
  } else if (op == MPI_NO_OP) {
    op_type = MPIOPNULL;
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
      .req = jenkins_one_at_a_time_hash((char *)request, sizeof(MPI_Request)),
      .tag = -1,
      .required_thread_lvl = -1,
      .provided_thread_lvl = -1,
      .op_type = op_type,
      .finished = false,
  };

  register_mpi_call(ireduce);
  return ret;
}

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

/*
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
*/