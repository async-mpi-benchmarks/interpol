use crate::interpol::Register;
use crate::types::{MpiRank, MpiReq, Tsc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::TryReserveError;

/// A structure that stores information about `MPI_Wait` calls.
///
/// The following data is gathered when the MPI function is called:
/// - the rank of the process;
/// - the request that was waited on;
/// - the number of CPU cycles.
/// The TSC is measured using the `rdtscp` and `lfence` instructions (see Intel documentation for
/// further information).
#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MpiWait {
    rank: MpiRank,
    req: MpiReq,
    tsc_before: Tsc,
    tsc_after: Tsc,
}

impl MpiWait {
    /// Creates a new `MpiWait` structure from the specified parameters.
    pub fn new(rank: MpiRank, req: MpiReq, tsc_before: Tsc, tsc_after: Tsc) -> Self {
        Self {
            rank,
            req,
            tsc_before,
            tsc_after,
        }
    }
}

#[typetag::serde]
impl Register for MpiWait {
    fn register(self, events: &mut Vec<Box<dyn Register>>) -> Result<(), TryReserveError> {
        // Ensure that the program does not panic if allocation fails
        events.try_reserve_exact(2 * events.len())?;
        events.push(Box::new(self));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds() {
        let wait_new = MpiWait::new(0, 0, 1024, 2048);
        let wait_builder = MpiWaitBuilder::default()
            .rank(0)
            .req(0)
            .tsc_before(1024)
            .tsc_after(2048)
            .build()
            .expect("failed to build `MpiWait`");

        assert_eq!(wait_new, wait_builder);
    }

    #[test]
    fn serializes() {
        let wait = MpiWait::new(0, 0, 1024, 2048);
        let json = String::from("{\"rank\":0,\"req\":0,\"tsc_before\":1024,\"tsc_after\":2048}");
        let serialized = serde_json::to_string(&wait).expect("failed to serialize `MpiWait`");

        assert_eq!(json, serialized);
    }

    #[test]
    fn deserializes() {
        let wait = MpiWaitBuilder::default()
            .rank(0)
            .req(0)
            .tsc_before(1024)
            .tsc_after(2048)
            .build()
            .expect("failed to build `MpiWait`");
        let serialized =
            serde_json::to_string_pretty(&wait).expect("failed to serialize `MpiWait`");
        let deserialized: MpiWait =
            serde_json::from_str(&serialized).expect("failed to deserialize `MpiWait`");

        assert_eq!(wait, deserialized);
    }
}
