#pragma once

// Warning, this file is autogenerated by cbindgen. Don't modify this manually.

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


enum MpiCallType
{
    Init,
    Initthread,
    Finalize,
    Send,
    Recv,
    Isend,
    Irecv,
    Test,
    Wait,
    Barrier,
    Ibarrier,
    Ibcast,
    Igather,
    Ireduce,
    Iscatter,
};
typedef int8_t MpiCallType;

enum MpiOp
{
    Opnull,
    Max,
    Min,
    Sum,
    Prod,
    Land,
    Band,
    Lor,
    Bor,
    Lxor,
    Bxor,
    Minloc,
    Maxloc,
    Replace,
};
typedef int8_t MpiOp;

typedef double Usecs;

typedef uint64_t Tsc;

typedef int32_t MpiRank;

typedef int32_t MpiComm;

typedef int32_t MpiReq;

typedef int32_t MpiTag;

typedef struct MpiCall
{
    Usecs time;
    Tsc tsc;
    Tsc duration;
    MpiRank partner_rank;
    MpiRank current_rank;
    uint32_t nb_bytes_s;
    uint32_t nb_bytes_r;
    MpiComm comm;
    MpiReq req;
    MpiTag tag;
    int32_t required_thread_lvl;
    int32_t provided_thread_lvl;
    bool finished;
    MpiOp op_type;
    MpiCallType kind;
} MpiCall;

void register_mpi_call(struct MpiCall mpi_call);

void sort_all_traces(void);
