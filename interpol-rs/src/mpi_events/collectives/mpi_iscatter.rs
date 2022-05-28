use crate::types::{MpiComm, MpiRank, MpiReq, Tsc};
use crate::{impl_builder_error, impl_register};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A structure that stores information about `MPI_Iscatter` calls.
///
/// The information stored are:
/// - the rank of the process making the call to `MPI_Iscatter`;
/// - the rank of the root process making the broadcast;
/// - the number of bytes sent;
/// - the number of bytes received;
/// - the identifier of the MPI communicator;
/// - the identifier of the MPI request;
/// - the tag of the communication;
/// - the current value of the Time Stamp counter before the call to `MPI_Iscatter`.
/// - the duration of the call.
#[derive(Builder, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MpiIscatter {
    current_rank: MpiRank,
    partner_rank: MpiRank,
    nb_bytes_send: u32,
    nb_bytes_recv: u32,
    comm: MpiComm,
    req: MpiReq,
    tsc: Tsc,
    duration: Tsc,
}

impl MpiIscatter {
    /// Creates a new `MpiIscatter` structure from the specified parameters.
    pub fn new(
        current_rank: MpiRank,
        partner_rank: MpiRank,
        nb_bytes_send: u32,
        nb_bytes_recv: u32,
        comm: MpiComm,
        req: MpiReq,
        tsc: Tsc,
        duration: Tsc,
    ) -> Self {
        MpiIscatter {
            current_rank,
            partner_rank,
            nb_bytes_send,
            nb_bytes_recv,
            comm,
            req,
            tsc,
            duration,
        }
    }
}

impl_builder_error!(MpiIscatterBuilderError);
impl_register!(MpiIscatter);

#[cfg(test)]
mod tests {
    use super::*;
    const MPI_COMM_WORLD: i32 = 0;

    #[test]
    fn builds() {
        let iscatter_new = MpiIscatter::new(0, 1, 8, 0, MPI_COMM_WORLD, 7, 1024, 2048);
        let iscatter_builder = MpiIscatterBuilder::default()
            .current_rank(0)
            .partner_rank(1)
            .nb_bytes_send(8)
            .nb_bytes_recv(0)
            .comm(MPI_COMM_WORLD)
            .req(7)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiIscatter`");

        assert_eq!(iscatter_new, iscatter_builder);
    }

    #[test]
    fn serializes() {
        let iscatter = MpiIscatter::new(0, 0, 8, 64, MPI_COMM_WORLD, 7, 1024, 2048);
        let json = String::from("{\"current_rank\":0,\"partner_rank\":0,\"nb_bytes_send\":8,\"nb_bytes_recv\":64,\"comm\":0,\"req\":7,\"tsc\":1024,\"duration\":2048}");
        let serialized =
            serde_json::to_string(&iscatter).expect("failed to serialize `MpiIscatter`");

        assert_eq!(json, serialized);
    }

    #[test]
    fn deserializes() {
        let iscatter = MpiIscatterBuilder::default()
            .current_rank(1)
            .partner_rank(0)
            .nb_bytes_send(64)
            .nb_bytes_recv(0)
            .comm(MPI_COMM_WORLD)
            .req(7)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiIscatter`");
        let serialized =
            serde_json::to_string_pretty(&iscatter).expect("failed to serialize `MpiIscatter`");
        let deserialized: MpiIscatter =
            serde_json::from_str(&serialized).expect("failed to deserialize `MpiIscatter`");

        assert_eq!(iscatter, deserialized);
    }
}
