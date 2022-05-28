use serde::{Deserialize, Serialize};

// MPI related types
pub type MpiComm = i32;
pub type MpiRank = i32;
pub type MpiReq = i32;
pub type MpiTag = i32;

// Others
pub type Tsc = u64;
pub type Usecs = f64;

#[derive(Debug, PartialEq, Eq)]
#[repr(i8)]
pub enum MpiCallType {
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i8)]
pub enum MpiOp {
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
}
