#include "../include/interpol.h"
#include "../include/tsc.h"

#include <mpi.h>
#include <stdint.h>
#include <stdio.h>
#include <sys/time.h>

#ifndef _EXTERN_C_
#ifdef __cplusplus
#define _EXTERN_C_ extern "C"
#else /* __cplusplus */
#define _EXTERN_C_
#endif /* __cplusplus */
#endif /* _EXTERN_C_ */

#ifdef MPICH_HAS_C2F
_EXTERN_C_ void *MPIR_ToPointer(int);
#endif // MPICH_HAS_C2F

#ifdef PIC
/* For shared libraries, declare these weak and figure out which one was linked
   based on which init wrapper was called.  See mpi_init wrappers.  */
#pragma weak pmpi_init
#pragma weak PMPI_INIT
#pragma weak pmpi_init_
#pragma weak pmpi_init__
#endif /* PIC */

_EXTERN_C_ void pmpi_init(MPI_Fint *ierr);
_EXTERN_C_ void PMPI_INIT(MPI_Fint *ierr);
_EXTERN_C_ void pmpi_init_(MPI_Fint *ierr);
_EXTERN_C_ void pmpi_init__(MPI_Fint *ierr);

static int fortran_init = 0;
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


static void MPI_Init_fortran_wrapper(MPI_Fint *ierr) { 
    int argc = 0;
    char ** argv = NULL;
    int _wrap_py_return_val = 0;

    _wrap_py_return_val = PMPI_Init(&argc, &argv);
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

    *ierr = _wrap_py_return_val;
}

_EXTERN_C_ void MPI_INIT(MPI_Fint *ierr) { 
    fortran_init = 1;
    MPI_Init_fortran_wrapper(ierr);
}

_EXTERN_C_ void mpi_init(MPI_Fint *ierr) { 
    fortran_init = 2;
    MPI_Init_fortran_wrapper(ierr);
}

_EXTERN_C_ void mpi_init_(MPI_Fint *ierr) { 
    fortran_init = 3;
    MPI_Init_fortran_wrapper(ierr);
}

_EXTERN_C_ void mpi_init__(MPI_Fint *ierr) { 
    fortran_init = 4;
    MPI_Init_fortran_wrapper(ierr);
}


static void MPI_Init_thread_fortran_wrapper(MPI_Fint *argc, MPI_Fint ***argv, MPI_Fint *required, MPI_Fint *provided, MPI_Fint *ierr) { 
    int _wrap_py_return_val = 0;

    _wrap_py_return_val = PMPI_Init_thread((int*)argc, (char***)argv, *required, (int*)provided);
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
        .required_thread_lvl = *required,
        .provided_thread_lvl = *provided,
        .op_type = -1,
        .finished = false,
    };

    register_mpi_call(initthread);
    *ierr = _wrap_py_return_val;
}

_EXTERN_C_ void MPI_INIT_THREAD(MPI_Fint *argc, MPI_Fint ***argv, MPI_Fint *required, MPI_Fint *provided, MPI_Fint *ierr) { 
    MPI_Init_thread_fortran_wrapper(argc, argv, required, provided, ierr);
}

_EXTERN_C_ void mpi_init_thread(MPI_Fint *argc, MPI_Fint ***argv, MPI_Fint *required, MPI_Fint *provided, MPI_Fint *ierr) { 
    MPI_Init_thread_fortran_wrapper(argc, argv, required, provided, ierr);
}

_EXTERN_C_ void mpi_init_thread_(MPI_Fint *argc, MPI_Fint ***argv, MPI_Fint *required, MPI_Fint *provided, MPI_Fint *ierr) { 
    MPI_Init_thread_fortran_wrapper(argc, argv, required, provided, ierr);
}

_EXTERN_C_ void mpi_init_thread__(MPI_Fint *argc, MPI_Fint ***argv, MPI_Fint *required, MPI_Fint *provided, MPI_Fint *ierr) { 
    MPI_Init_thread_fortran_wrapper(argc, argv, required, provided, ierr);
}


static void MPI_Finalize_fortran_wrapper(MPI_Fint *ierr) { 
    int _wrap_py_return_val = 0;

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

    _wrap_py_return_val = PMPI_Finalize();

    *ierr = _wrap_py_return_val;
}

_EXTERN_C_ void MPI_FINALIZE(MPI_Fint *ierr) { 
    MPI_Finalize_fortran_wrapper(ierr);
}

_EXTERN_C_ void mpi_finalize(MPI_Fint *ierr) { 
    MPI_Finalize_fortran_wrapper(ierr);
}

_EXTERN_C_ void mpi_finalize_(MPI_Fint *ierr) { 
    MPI_Finalize_fortran_wrapper(ierr);
}

_EXTERN_C_ void mpi_finalize__(MPI_Fint *ierr) { 
    MPI_Finalize_fortran_wrapper(ierr);
}


static void MPI_Send_fortran_wrapper(MPI_Fint *buf, MPI_Fint *count, MPI_Fint *datatype, MPI_Fint *dest, MPI_Fint *tag, MPI_Fint *comm, MPI_Fint *ierr) { 
    int _wrap_py_return_val = 0;

    Tsc const tsc = rdtsc();

    #if (!defined(MPICH_HAS_C2F) && defined(MPICH_NAME) && (MPICH_NAME == 1)) /* MPICH test */
        _wrap_py_return_val = PMPI_Send((const void*)buf, *count, (MPI_Datatype)(*datatype), *dest, *tag, (MPI_Comm)(*comm));
    #else /* MPI-2 safe call */
        _wrap_py_return_val = PMPI_Send((const void*)buf, *count, MPI_Type_f2c(*datatype), *dest, *tag, MPI_Comm_f2c(*comm));
    #endif /* MPICH test */

    Tsc const duration = rdtsc() - tsc;

    int nb_bytes;
    PMPI_Type_size(MPI_Type_f2c(*datatype), &nb_bytes);

    MpiCall const send = {
        .kind = Send,
        .time = -1.0,
        .tsc = tsc,
        .duration = duration,
        .current_rank = current_rank,
        .partner_rank = *dest,
        .nb_bytes_s = nb_bytes * (*count),
        .nb_bytes_r = 0,
        .comm = PMPI_Comm_c2f((MPI_Comm)comm),
        .req = -1,
        .tag = *tag,
        .required_thread_lvl = -1,
        .provided_thread_lvl = -1,
        .op_type = -1,
        .finished = false,
    };

    register_mpi_call(send);

    *ierr = _wrap_py_return_val;
}

_EXTERN_C_ void MPI_SEND(MPI_Fint *buf, MPI_Fint *count, MPI_Fint *datatype, MPI_Fint *dest, MPI_Fint *tag, MPI_Fint *comm, MPI_Fint *ierr) { 
    MPI_Send_fortran_wrapper(buf, count, datatype, dest, tag, comm, ierr);
}

_EXTERN_C_ void mpi_send(MPI_Fint *buf, MPI_Fint *count, MPI_Fint *datatype, MPI_Fint *dest, MPI_Fint *tag, MPI_Fint *comm, MPI_Fint *ierr) { 
    MPI_Send_fortran_wrapper(buf, count, datatype, dest, tag, comm, ierr);
}

_EXTERN_C_ void mpi_send_(MPI_Fint *buf, MPI_Fint *count, MPI_Fint *datatype, MPI_Fint *dest, MPI_Fint *tag, MPI_Fint *comm, MPI_Fint *ierr) { 
    MPI_Send_fortran_wrapper(buf, count, datatype, dest, tag, comm, ierr);
}

_EXTERN_C_ void mpi_send__(MPI_Fint *buf, MPI_Fint *count, MPI_Fint *datatype, MPI_Fint *dest, MPI_Fint *tag, MPI_Fint *comm, MPI_Fint *ierr) { 
    MPI_Send_fortran_wrapper(buf, count, datatype, dest, tag, comm, ierr);
}


static void MPI_Recv_fortran_wrapper(MPI_Fint *buf, MPI_Fint *count, MPI_Fint *datatype, MPI_Fint *source, MPI_Fint *tag, MPI_Fint *comm, MPI_Fint *status, MPI_Fint *ierr) { 
    int _wrap_py_return_val = 0;

    Tsc const tsc = rdtsc();

    #if (!defined(MPICH_HAS_C2F) && defined(MPICH_NAME) && (MPICH_NAME == 1)) /* MPICH test */
        _wrap_py_return_val = PMPI_Recv((void*)buf, *count, (MPI_Datatype)(*datatype), *source, *tag, (MPI_Comm)(*comm), (MPI_Status*)status);
    #else /* MPI-2 safe call */
        //MPI_Status temp_status;
        //MPI_Status_f2c(status, &temp_status);
        _wrap_py_return_val = PMPI_Recv((void*)buf, *count, MPI_Type_f2c(*datatype), *source, *tag, MPI_Comm_f2c(*comm), (MPI_Status*)status);
        //MPI_Status_c2f(&temp_status, status);
    #endif /* MPICH test */

    Tsc const duration = rdtsc() - tsc;

    int nb_bytes;
    PMPI_Type_size(MPI_Type_f2c(*datatype), &nb_bytes);

    MpiCall const recv = {
        .kind = Recv,
        .time = -1.0,
        .tsc = tsc,
        .duration = duration,
        .current_rank = current_rank,
        .partner_rank = *source,
        .nb_bytes_s = 0,
        .nb_bytes_r = nb_bytes * (*count),
        .comm = PMPI_Comm_c2f((MPI_Comm)comm),
        .req = -1,
        .tag = *tag,
        .required_thread_lvl = -1,
        .provided_thread_lvl = -1,
        .op_type = -1,
        .finished = false,
    };

    register_mpi_call(recv);

    *ierr = _wrap_py_return_val;
}

_EXTERN_C_ void MPI_RECV(MPI_Fint *buf, MPI_Fint *count, MPI_Fint *datatype, MPI_Fint *source, MPI_Fint *tag, MPI_Fint *comm, MPI_Fint *status, MPI_Fint *ierr) { 
    MPI_Recv_fortran_wrapper(buf, count, datatype, source, tag, comm, status, ierr);
}

_EXTERN_C_ void mpi_recv(MPI_Fint *buf, MPI_Fint *count, MPI_Fint *datatype, MPI_Fint *source, MPI_Fint *tag, MPI_Fint *comm, MPI_Fint *status, MPI_Fint *ierr) { 
    MPI_Recv_fortran_wrapper(buf, count, datatype, source, tag, comm, status, ierr);
}

_EXTERN_C_ void mpi_recv_(MPI_Fint *buf, MPI_Fint *count, MPI_Fint *datatype, MPI_Fint *source, MPI_Fint *tag, MPI_Fint *comm, MPI_Fint *status, MPI_Fint *ierr) { 
    MPI_Recv_fortran_wrapper(buf, count, datatype, source, tag, comm, status, ierr);
}

_EXTERN_C_ void mpi_recv__(MPI_Fint *buf, MPI_Fint *count, MPI_Fint *datatype, MPI_Fint *source, MPI_Fint *tag, MPI_Fint *comm, MPI_Fint *status, MPI_Fint *ierr) { 
    MPI_Recv_fortran_wrapper(buf, count, datatype, source, tag, comm, status, ierr);
}


static void MPI_Isend_fortran_wrapper(MPI_Fint *buf, MPI_Fint *count, MPI_Fint *datatype, MPI_Fint *dest, MPI_Fint *tag, MPI_Fint *comm, MPI_Fint *request, MPI_Fint *ierr) { 
    int _wrap_py_return_val = 0;

    Tsc const tsc = rdtsc();

    #if (!defined(MPICH_HAS_C2F) && defined(MPICH_NAME) && (MPICH_NAME == 1)) /* MPICH test */
        _wrap_py_return_val = PMPI_Isend((const void*)buf, *count, (MPI_Datatype)(*datatype), *dest, *tag, (MPI_Comm)(*comm), (MPI_Request*)request);
    #else /* MPI-2 safe call */
        MPI_Request temp_request;
        temp_request = MPI_Request_f2c(*request);
        _wrap_py_return_val = PMPI_Isend((const void*)buf, *count, MPI_Type_f2c(*datatype), *dest, *tag, MPI_Comm_f2c(*comm), &temp_request);
        *request = MPI_Request_c2f(temp_request);
    #endif /* MPICH test */

    Tsc const duration = rdtsc() - tsc;

    int nb_bytes;
    PMPI_Type_size(MPI_Type_f2c(*datatype), &nb_bytes);

    MpiCall const isend = {
        .kind = Isend,
        .time = -1.0,
        .tsc = tsc,
        .duration = duration,
        .current_rank = current_rank,
        .partner_rank = *dest,
        .nb_bytes_s = nb_bytes * (*count),
        .nb_bytes_r = 0,
        .comm = PMPI_Comm_c2f((MPI_Comm)comm),
        .req = *request,
        .tag = *tag,
        .required_thread_lvl = -1,
        .provided_thread_lvl = -1,
        .op_type = -1,
        .finished = false,
    };

    register_mpi_call(isend);

    *ierr = _wrap_py_return_val;
}

_EXTERN_C_ void MPI_ISEND(MPI_Fint *buf, MPI_Fint *count, MPI_Fint *datatype, MPI_Fint *dest, MPI_Fint *tag, MPI_Fint *comm, MPI_Fint *request, MPI_Fint *ierr) { 
    MPI_Isend_fortran_wrapper(buf, count, datatype, dest, tag, comm, request, ierr);
}

_EXTERN_C_ void mpi_isend(MPI_Fint *buf, MPI_Fint *count, MPI_Fint *datatype, MPI_Fint *dest, MPI_Fint *tag, MPI_Fint *comm, MPI_Fint *request, MPI_Fint *ierr) { 
    MPI_Isend_fortran_wrapper(buf, count, datatype, dest, tag, comm, request, ierr);
}

_EXTERN_C_ void mpi_isend_(MPI_Fint *buf, MPI_Fint *count, MPI_Fint *datatype, MPI_Fint *dest, MPI_Fint *tag, MPI_Fint *comm, MPI_Fint *request, MPI_Fint *ierr) { 
    MPI_Isend_fortran_wrapper(buf, count, datatype, dest, tag, comm, request, ierr);
}

_EXTERN_C_ void mpi_isend__(MPI_Fint *buf, MPI_Fint *count, MPI_Fint *datatype, MPI_Fint *dest, MPI_Fint *tag, MPI_Fint *comm, MPI_Fint *request, MPI_Fint *ierr) { 
    MPI_Isend_fortran_wrapper(buf, count, datatype, dest, tag, comm, request, ierr);
}