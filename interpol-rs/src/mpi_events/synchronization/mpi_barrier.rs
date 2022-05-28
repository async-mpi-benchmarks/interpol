use crate::types::{MpiComm, MpiRank, Tsc};
use crate::{impl_builder_error, impl_register};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A structure that stores information about `MPI_Barrier` calls.
///
/// The following data is gathered when the MPI function is called:
/// - the rank of the process;
/// - the communicator on which the barrier was called;
/// - the current value of the Time Stamp counter before the call to `MPI_Barrier`;
/// - the duration of the call.
/// The TSC is measured using the `rdtscp` and `lfence` instructions (see Intel documentation for
/// further information).
#[derive(Builder, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MpiBarrier {
    current_rank: MpiRank,
    comm: MpiComm,
    tsc: Tsc,
    duration: Tsc,
}

impl MpiBarrier {
    /// Creates a new `MpiBarrier` structure from the specified parameters.
    pub fn new(current_rank: MpiRank, comm: MpiComm, tsc: Tsc, duration: Tsc) -> Self {
        Self {
            current_rank,
            comm,
            tsc,
            duration,
        }
    }
}

impl_builder_error!(MpiBarrierBuilderError);
impl_register!(MpiBarrier);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds() {
        let barrier_new = MpiBarrier::new(0, 0, 1024, 2048);
        let barrier_builder = MpiBarrierBuilder::default()
            .current_rank(0)
            .comm(0)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiBarrier`");

        assert_eq!(barrier_new, barrier_builder);
    }

    #[test]
    fn serializes() {
        let barrier = MpiBarrier::new(0, 0, 1024, 2048);
        let json = String::from("{\"current_rank\":0,\"comm\":0,\"tsc\":1024,\"duration\":2048}");
        let serialized = serde_json::to_string(&barrier).expect("failed to serialize `MpiBarrier`");

        assert_eq!(json, serialized);
    }

    #[test]
    fn deserializes() {
        let barrier = MpiBarrierBuilder::default()
            .current_rank(0)
            .comm(0)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiBarrier`");
        let serialized =
            serde_json::to_string_pretty(&barrier).expect("failed to serialize `MpiBarrier`");
        let deserialized: MpiBarrier =
            serde_json::from_str(&serialized).expect("failed to deserialize `MpiBarrier`");

        assert_eq!(barrier, deserialized);
    }
}
