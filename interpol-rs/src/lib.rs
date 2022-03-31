pub mod interpol;
pub mod mpi_events;
pub mod types;

pub(crate) mod mpi_consts {
    use crate::types;

    pub const MPI_COMM_WORLD: types::MpiComm = 0;
}
