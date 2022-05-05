use crate::{impl_builder_error, impl_register};
use crate::types::{MpiRank, MpiReq, Tsc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A structure that stores information about `MPI_Wait` calls.
///
/// The following data is gathered when the MPI function is called:
/// - the rank of the process;
/// - the request that was waited on;
/// - the current value of the Time Stamp counter before the call to `MPI_Wait`;
/// - the duration of the call.
/// The TSC is measured using the `rdtscp` and `lfence` instructions (see Intel documentation for
/// further information).
#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MpiWait {
    current_rank: MpiRank,
    req: MpiReq,
    tsc: Tsc,
    duration: Tsc,
}

impl MpiWait {
    /// Creates a new `MpiWait` structure from the specified parameters.
    pub fn new(current_rank: MpiRank, req: MpiReq, tsc: Tsc, duration: Tsc) -> Self {
        Self {
            current_rank,
            req,
            tsc,
            duration,
        }
    }
}

impl_builder_error!(MpiWaitBuilderError);
impl_register!(MpiWait);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds() {
        let wait_new = MpiWait::new(0, 0, 1024, 2048);
        let wait_builder = MpiWaitBuilder::default()
            .current_rank(0)
            .req(0)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiWait`");

        assert_eq!(wait_new, wait_builder);
    }

    #[test]
    fn serializes() {
        let wait = MpiWait::new(0, 0, 1024, 2048);
        let json = String::from("{\"current_rank\":0,\"req\":0,\"tsc\":1024,\"duration\":2048}");
        let serialized = serde_json::to_string(&wait).expect("failed to serialize `MpiWait`");

        assert_eq!(json, serialized);
    }

    #[test]
    fn deserializes() {
        let wait = MpiWaitBuilder::default()
            .current_rank(0)
            .req(0)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiWait`");
        let serialized =
            serde_json::to_string_pretty(&wait).expect("failed to serialize `MpiWait`");
        let deserialized: MpiWait =
            serde_json::from_str(&serialized).expect("failed to deserialize `MpiWait`");

        assert_eq!(wait, deserialized);
    }
}
