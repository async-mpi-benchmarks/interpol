use crate::types::{MpiComm, MpiRank, MpiReq, Tsc};
use crate::{impl_builder_error, impl_register};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A structure that stores information about `MPI_Barrier` calls.
///
/// The following data is gathered when the MPI function is called:
/// - the rank of the process;
/// - the communicator on which the barrier was called;
/// - the request associated with the barrier;
/// - the current value of the Time Stamp counter before the call to `MPI_Barrier`;
/// - the duration of the call.
/// The TSC is measured using the `rdtscp` and `lfence` instructions (see Intel documentation for
/// further information).
#[derive(Builder, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MpiIbarrier {
    current_rank: MpiRank,
    comm: MpiComm,
    req: MpiReq,
    tsc: Tsc,
    duration: Tsc,
}

impl MpiIbarrier {
    /// Creates a new `MpiBarrier` structure from the specified parameters.
    pub fn new(current_rank: MpiRank, comm: MpiComm, req: MpiReq, tsc: Tsc, duration: Tsc) -> Self {
        Self {
            current_rank,
            comm,
            req,
            tsc,
            duration,
        }
    }
}

impl_builder_error!(MpiIbarrierBuilderError);
impl_register!(MpiIbarrier);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds() {
        let ibarrier_new = MpiIbarrier::new(0, 0, 0, 1024, 2048);
        let ibarrier_builder = MpiIbarrierBuilder::default()
            .current_rank(0)
            .comm(0)
            .req(0)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiIbarrier`");

        assert_eq!(ibarrier_new, ibarrier_builder);
    }

    #[test]
    fn serializes() {
        let ibarrier = MpiIbarrier::new(0, 0, 0, 1024, 2048);
        let json = String::from(
            "{\"current_rank\":0,\"comm\":0,\"req\":0,\"tsc\":1024,\"duration\":2048}",
        );
        let serialized =
            serde_json::to_string(&ibarrier).expect("failed to serialize `MpiIbarrier`");

        assert_eq!(json, serialized);
    }

    #[test]
    fn deserializes() {
        let ibarrier = MpiIbarrierBuilder::default()
            .current_rank(0)
            .comm(0)
            .req(0)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiIbarrier`");
        let serialized =
            serde_json::to_string_pretty(&ibarrier).expect("failed to serialize `MpiIbarrier`");
        let deserialized: MpiIbarrier =
            serde_json::from_str(&serialized).expect("failed to deserialize `MpiIbarrier`");

        assert_eq!(ibarrier, deserialized);
    }
}
