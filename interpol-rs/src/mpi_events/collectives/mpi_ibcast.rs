use crate::types::{MpiComm, MpiRank, MpiReq, Tsc};
use crate::{impl_builder_error, impl_register};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A structure that stores information about `MPI_Ibcast` calls.
///
/// The information stored are:
/// - the rank of the process making the call to `MPI_Ibcast`;
/// - the rank of the root process making the broadcast;
/// - the number of bytes exchanged;
/// - the identifier of the MPI communicator;
/// - the identifier of the MPI request;
/// - the tag of the communication;
/// - the current value of the Time Stamp counter before the call to `MPI_Ibcast`.
/// - the duration of the call.
#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MpiIbcast {
    current_rank: MpiRank,
    partner_rank: MpiRank,
    nb_bytes: u32,
    comm: MpiComm,
    req: MpiReq,
    tsc: Tsc,
    duration: Tsc,
}

impl MpiIbcast {
    /// Creates a new `MpiIbcast` structure from the specified parameters.
    pub fn new(
        current_rank: MpiRank,
        partner_rank: MpiRank,
        nb_bytes: u32,
        comm: MpiComm,
        req: MpiReq,
        tsc: Tsc,
        duration: Tsc,
    ) -> Self {
        MpiIbcast {
            current_rank,
            partner_rank,
            nb_bytes,
            comm,
            req,
            tsc,
            duration,
        }
    }
}

impl_builder_error!(MpiIbcastBuilderError);
impl_register!(MpiIbcast);

#[cfg(test)]
mod tests {
    use super::*;
    const MPI_COMM_WORLD: i32 = 0;

    #[test]
    fn builds() {
        let ibcast_new = MpiIbcast::new(0, 1, 8, MPI_COMM_WORLD, 7, 1024, 2048);
        let ibcast_builder = MpiIbcastBuilder::default()
            .current_rank(0)
            .partner_rank(1)
            .nb_bytes(8)
            .comm(MPI_COMM_WORLD)
            .req(7)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiIbcast`");

        assert_eq!(ibcast_new, ibcast_builder);
    }

    #[test]
    fn serializes() {
        let ibcast = MpiIbcast::new(0, 0, 8, MPI_COMM_WORLD, 7, 1024, 2048);
        let json = String::from("{\"current_rank\":0,\"partner_rank\":0,\"nb_bytes\":8,\"comm\":0,\"req\":7,\"tsc\":1024,\"duration\":2048}");
        let serialized = serde_json::to_string(&ibcast).expect("failed to serialize `MpiIbcast`");

        assert_eq!(json, serialized);
    }

    #[test]
    fn deserializes() {
        let ibcast = MpiIbcastBuilder::default()
            .current_rank(1)
            .partner_rank(0)
            .nb_bytes(8)
            .comm(MPI_COMM_WORLD)
            .req(7)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiIbcast`");
        let serialized =
            serde_json::to_string_pretty(&ibcast).expect("failed to serialize `MpiIbcast`");
        let deserialized: MpiIbcast =
            serde_json::from_str(&serialized).expect("failed to deserialize `MpiIbcast`");

        assert_eq!(ibcast, deserialized);
    }
}
