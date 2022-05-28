use crate::types::{MpiComm, MpiRank, MpiReq, MpiTag, Tsc};
use crate::{impl_builder_error, impl_register};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A structure that stores information about `MPI_Isend` calls.
///
/// The following data is gathered when the MPI function is called:
/// - the rank of the processes involved in the communication;
/// - the number of bytes exchanged;
/// - the identifier of the MPI communicator;
/// - the identifier of the MPI request;
/// - the tag of the communication;
/// - the current value of the Time Stamp counter before the call to `MPI_Isend`.
/// - the duration of the call.
/// The TSC is measured using the `rdtscp` and `lfence` instructions (see Intel documentation for
/// further information).
#[derive(Builder, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MpiIsend {
    current_rank: MpiRank,
    partner_rank: MpiRank,
    nb_bytes: u32,
    comm: MpiComm,
    req: MpiReq,
    tag: MpiTag,
    tsc: Tsc,
    duration: Tsc,
}

impl MpiIsend {
    /// Creates a new `MpiIsend` structure based off of the specified parameters.
    pub fn new(
        current_rank: MpiRank,
        partner_rank: MpiRank,
        nb_bytes: u32,
        comm: MpiComm,
        req: MpiReq,
        tag: MpiTag,
        tsc: Tsc,
        duration: Tsc,
    ) -> Self {
        Self {
            current_rank,
            partner_rank,
            nb_bytes,
            comm,
            req,
            tag,
            tsc,
            duration,
        }
    }
}

impl_builder_error!(MpiIsendBuilderError);
impl_register!(MpiIsend);

#[cfg(test)]
mod tests {
    use super::*;
    const MPI_COMM_WORLD: i32 = 0;

    #[test]
    fn builds() {
        let isend_new = MpiIsend::new(0, 1, 8, MPI_COMM_WORLD, 7, 42, 1024, 2048);
        let isend_builder = MpiIsendBuilder::default()
            .current_rank(0)
            .partner_rank(1)
            .nb_bytes(8)
            .comm(MPI_COMM_WORLD)
            .req(7)
            .tag(42)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiIsend`");

        assert_eq!(isend_new, isend_builder);
    }

    #[test]
    fn serializes() {
        let isend = MpiIsend::new(0, 1, 8, MPI_COMM_WORLD, 7, 42, 1024, 2048);
        let json = String::from("{\"current_rank\":0,\"partner_rank\":1,\"nb_bytes\":8,\"comm\":0,\"req\":7,\"tag\":42,\"tsc\":1024,\"duration\":2048}");
        let serialized = serde_json::to_string(&isend).expect("failed to serialize `MpiIsend`");

        assert_eq!(json, serialized);
    }

    #[test]
    fn deserializes() {
        let isend = MpiIsendBuilder::default()
            .current_rank(0)
            .partner_rank(1)
            .nb_bytes(8)
            .comm(MPI_COMM_WORLD)
            .req(7)
            .tag(42)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiIsend`");
        let serialized =
            serde_json::to_string_pretty(&isend).expect("failed to serialize `MpiIsend`");
        let deserialized: MpiIsend =
            serde_json::from_str(&serialized).expect("failed to deserialize `MpiIsend`");

        assert_eq!(isend, deserialized);
    }
}
