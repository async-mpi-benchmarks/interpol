use crate::types::{MpiComm, MpiRank, MpiReq, MpiTag, Tsc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A structure that stores information about `MPI_Irecv` calls.
///
/// The following data is gathered when the MPI function is called:
/// - the rank of the processes involved in the communication;
/// - the number of bytes exchanged;
/// - the identifier of the MPI communicator;
/// - the identifier of the MPI request;
/// - the tag of the communication;
/// - the number of CPU cycles before and after the call.
/// The TSC is measured using the `rdtscp` and `lfence` instructions (see Intel documentation for
/// further information).
#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MpiIrecv {
    current_rank: MpiRank,
    partner_rank: MpiRank,
    nb_bytes: u32,
    comm: MpiComm,
    req: MpiReq,
    tag: MpiTag,
    tsc_before: Tsc,
    tsc_after: Tsc,
}

impl MpiIrecv {
    /// Creates a new `MpiIrecv` structure based off of the specified parameters.
    pub fn new(
        current_rank: MpiRank,
        partner_rank: MpiRank,
        nb_bytes: u32,
        comm: MpiComm,
        req: MpiReq,
        tag: MpiTag,
        tsc_before: Tsc,
        tsc_after: Tsc,
    ) -> Self {
        Self {
            current_rank,
            partner_rank,
            nb_bytes,
            comm,
            req,
            tag,
            tsc_before,
            tsc_after,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpi_consts::MPI_COMM_WORLD;

    #[test]
    fn builds() {
        let irecv_new = MpiIrecv::new(0, 1, 8, MPI_COMM_WORLD, 7, 42, 1024, 2048);
        let irecv_builder = MpiIrecvBuilder::default()
            .current_rank(0)
            .partner_rank(1)
            .nb_bytes(8)
            .comm(MPI_COMM_WORLD)
            .req(7)
            .tag(42)
            .tsc_before(1024)
            .tsc_after(2048)
            .build()
            .expect("failed to build `MpiIrecv`");

        assert_eq!(irecv_new, irecv_builder);
    }

    #[test]
    fn serializes() {
        let irecv = MpiIrecv::new(0, 1, 8, MPI_COMM_WORLD, 7, 42, 1024, 2048);
        let json = String::from("{\"current_rank\":0,\"partner_rank\":1,\"nb_bytes\":8,\"comm\":0,\"req\":7,\"tag\":42,\"tsc_before\":1024,\"tsc_after\":2048}");
        let serialized = serde_json::to_string(&irecv).expect("failed to serialize `MpiIrecv`");

        assert_eq!(json, serialized);
    }

    #[test]
    fn deserializes() {
        let irecv = MpiIrecvBuilder::default()
            .current_rank(0)
            .partner_rank(1)
            .nb_bytes(8)
            .comm(MPI_COMM_WORLD)
            .req(7)
            .tag(42)
            .tsc_before(1024)
            .tsc_after(2048)
            .build()
            .expect("failed to build `MpiIrecv`");
        let serialized =
            serde_json::to_string_pretty(&irecv).expect("failed to serialize `MpiIrecv`");
        let deserialized: MpiIrecv =
            serde_json::from_str(&serialized).expect("failed to deserialize `MpiIrecv`");

        assert_eq!(irecv, deserialized);
    }
}
