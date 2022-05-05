use crate::{
    impl_builder_error, impl_register,
    types::{MpiComm, MpiRank, MpiTag, Tsc},
};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A structure that stores information about `MPI_Recv` calls.
///
/// The following data is gathered when the MPI function is called:
/// - the rank of the processes involved in the communication;
/// - the number of bytes exchanged;
/// - the identifier of the MPI communicator;
/// - the tag of the communication;
/// - the current value of the Time Stamp counter before the call to `MPI_Recv`.
/// - the duration of the call.
/// The TSC is measured using the `rdtscp` and `lfence` instructions (see Intel documentation for
/// further information).
#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MpiRecv {
    current_rank: MpiRank,
    partner_rank: MpiRank,
    nb_bytes: u32,
    comm: MpiComm,
    tag: MpiTag,
    tsc: Tsc,
    duration: Tsc,
}

impl MpiRecv {
    /// Creates a new `MpiRecv` structure based off of the specified parameters.
    pub fn new(
        current_rank: MpiRank,
        partner_rank: MpiRank,
        nb_bytes: u32,
        comm: MpiComm,
        tag: MpiTag,
        tsc: Tsc,
        duration: Tsc,
    ) -> Self {
        Self {
            current_rank,
            partner_rank,
            nb_bytes,
            comm,
            tag,
            tsc,
            duration,
        }
    }
}

impl_builder_error!(MpiRecvBuilderError);
impl_register!(MpiRecv);

#[cfg(test)]
mod tests {
    use super::*;
    const MPI_COMM_WORLD: i32 = 0;

    #[test]
    fn builds() {
        let recv_new = MpiRecv::new(0, 1, 8, MPI_COMM_WORLD, 42, 1024, 2048);
        let recv_builder = MpiRecvBuilder::default()
            .current_rank(0)
            .partner_rank(1)
            .nb_bytes(8)
            .comm(MPI_COMM_WORLD)
            .tag(42)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiRecv`");

        assert_eq!(recv_new, recv_builder);
    }

    #[test]
    fn serializes() {
        let recv = MpiRecv::new(0, 1, 8, MPI_COMM_WORLD, 42, 1024, 2048);
        let json = String::from("{\"current_rank\":0,\"partner_rank\":1,\"nb_bytes\":8,\"comm\":0,\"tag\":42,\"tsc\":1024,\"duration\":2048}");
        let serialized = serde_json::to_string(&recv).expect("failed to serialize `MpiRecv`");

        assert_eq!(json, serialized);
    }

    #[test]
    fn deserializes() {
        let recv = MpiRecvBuilder::default()
            .current_rank(0)
            .partner_rank(1)
            .nb_bytes(8)
            .comm(MPI_COMM_WORLD)
            .tag(42)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiRecv`");
        let serialized =
            serde_json::to_string_pretty(&recv).expect("failed to serialize `MpiRecv`");
        let deserialized: MpiRecv =
            serde_json::from_str(&serialized).expect("failed to deserialize `MpiRecv`");

        assert_eq!(recv, deserialized);
    }
}
