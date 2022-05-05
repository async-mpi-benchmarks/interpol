use crate::{impl_builder_error, impl_register};
use crate::types::{MpiComm, MpiRank, MpiReq, Tsc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A structure that stores information about `MPI_Igather` calls.
///
/// The information stored are:
/// - the rank of the process making the call to `MPI_Igather`;
/// - the rank of the root process making the broadcast;
/// - the number of bytes sent;
/// - the number of bytes received;
/// - the identifier of the MPI communicator;
/// - the identifier of the MPI request;
/// - the tag of the communication;
/// - the current value of the Time Stamp counter before the call to `MPI_Igather`.
/// - the duration of the call.
#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MpiIgather {
    current_rank: MpiRank,
    partner_rank: MpiRank,
    nb_bytes_send: u32,
    nb_bytes_recv: u32,
    comm: MpiComm,
    req: MpiReq,
    tsc: Tsc,
    duration: Tsc,
}

impl MpiIgather {
    /// Creates a new `MpiIgather` structure from the specified parameters.
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
        MpiIgather {
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

impl_builder_error!(MpiIgatherBuilderError);
impl_register!(MpiIgather);

#[cfg(test)]
mod tests {
    use super::*;
    const MPI_COMM_WORLD: i32 = 0;

    #[test]
    fn builds() {
        let igather_new = MpiIgather::new(0, 1, 8, 0, MPI_COMM_WORLD, 7, 1024, 2048);
        let igather_builder = MpiIgatherBuilder::default()
            .current_rank(0)
            .partner_rank(1)
            .nb_bytes_send(8)
            .nb_bytes_recv(0)
            .comm(MPI_COMM_WORLD)
            .req(7)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiIgather`");

        assert_eq!(igather_new, igather_builder);
    }

    #[test]
    fn serializes() {
        let igather = MpiIgather::new(0, 0, 8, 64, MPI_COMM_WORLD, 7, 1024, 2048);
        let json = String::from("{\"current_rank\":0,\"partner_rank\":0,\"nb_bytes_send\":8,\"nb_bytes_recv\":64,\"comm\":0,\"req\":7,\"tsc\":1024,\"duration\":2048}");
        let serialized = serde_json::to_string(&igather).expect("failed to serialize `MpiIgather`");

        assert_eq!(json, serialized);
    }

    #[test]
    fn deserializes() {
        let igather = MpiIgatherBuilder::default()
            .current_rank(1)
            .partner_rank(0)
            .nb_bytes_send(64)
            .nb_bytes_recv(0)
            .comm(MPI_COMM_WORLD)
            .req(7)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiIgather`");
        let serialized =
            serde_json::to_string_pretty(&igather).expect("failed to serialize `MpiIgather`");
        let deserialized: MpiIgather =
            serde_json::from_str(&serialized).expect("failed to deserialize `MpiIgather`");

        assert_eq!(igather, deserialized);
    }
}
