use crate::interpol::Register;
use crate::types::{MpiRank, Tsc, Usecs};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::TryReserveError;

/// A structure that stores information about `MPI_Finalize` calls.
///
/// The following data is gathered when the MPI function is called:
/// - the rank of the process;
/// - the number of CPU cycles;
/// - the time in microseconds;
/// The TSC is measured using the `rdtscp` and `lfence` instructions (see Intel documentation for
/// further information).
#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MpiFinalize {
    rank: MpiRank,
    tsc: Tsc,
    time: Usecs,
}

impl MpiFinalize {
    /// Creates a new `MpiFinalize` structure based off of a `MpiRank`, a number of CPU cycles and
    /// a time in microseconds.
    pub fn new(rank: MpiRank, tsc: Tsc, time: Usecs) -> Self {
        Self { rank, tsc, time }
    }
}

#[typetag::serde]
impl Register for MpiFinalize {
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
        let finalize_new = MpiFinalize::new(0, 1024, 0.1);
        let finalize_builder = MpiFinalizeBuilder::default()
            .rank(0)
            .tsc(1024)
            .time(0.1)
            .build()
            .expect("failed to build `MpiFinalize`");

        assert_eq!(finalize_new, finalize_builder);
    }

    #[test]
    fn serializes() {
        let finalize = MpiFinalize::new(0, 1024, 0.1);
        let json = String::from("{\"rank\":0,\"tsc\":1024,\"time\":0.1}");
        let serialized =
            serde_json::to_string(&finalize).expect("failed to serialize `MpiFinalize`");
        assert_eq!(json, serialized);
    }

    #[test]
    fn deserializes() {
        let finalize = MpiFinalizeBuilder::default()
            .rank(0)
            .tsc(1024)
            .time(0.1)
            .build()
            .expect("failed to build `MpiFinalize`");

        let serialized =
            serde_json::to_string_pretty(&finalize).expect("failed to serialize `MpiFinalize`");
        let deserialized: MpiFinalize =
            serde_json::from_str(&serialized).expect("failed to deserialize `MpiFinalize`");

        assert_eq!(finalize, deserialized);
    }
}
