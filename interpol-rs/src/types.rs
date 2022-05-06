use serde::{Deserialize, Serialize};

// MPI related types
pub type MpiComm = i32;
pub type MpiRank = i32;
pub type MpiReq = i32;
pub type MpiTag = i32;

// Others
pub type Tsc = u64;
pub type Usecs = f64;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(i8)]
pub enum MPIOp {
	MPIOPNULL,
	MPIMAX,
  	MPIMIN,
  	MPISUM,
  	MPIPROD,
  	MPILAND,
  	MPIBAND,
  	MPILOR,
  	MPIBOR,
  	MPILXOR,
  	MPIBXOR,
  	MPIMINLOC,
  	MPIMAXLOC,
  	MPIREPLACE,
}