use crate::interpol::Register;
use crate::types::{MpiRank, Tsc, Usecs};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::TryReserveError;

/// A structure that stores information about `MPI_Init` calls.
///
/// The following data is gathered when the MPI function is called:
/// - the rank of the process;
/// - the current value of the Time Stamp counter;
/// - the time in microseconds.
/// The TSC is measured using the `rdtscp` and `lfence` instructions (see Intel documentation for
/// further information).
#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MpiInit {
    current_rank: MpiRank,
    tsc: Tsc,
    time: Usecs,
}

impl MpiInit {
    /// Creates a new `MpiInit` structure based off of a `MpiRank`, a number of CPU cycles and a
    /// time in microseconds.
    pub fn new(current_rank: MpiRank, tsc: Tsc, time: Usecs) -> Self {
        Self {
            current_rank,
            tsc,
            time,
        }
    }
}

#[typetag::serde]
impl Register for MpiInit {
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
        let init_new = MpiInit::new(0, 1024, 0.1);
        let init_builder = MpiInitBuilder::default()
            .current_rank(0)
            .tsc(1024)
            .time(0.1)
            .build()
            .expect("failed to build `MpiInit`");

        assert_eq!(init_new, init_builder);
    }

    #[test]
    fn serializes() {
        let init = MpiInit::new(0, 1024, 0.1);
        let json = String::from("{\"current_rank\":0,\"tsc\":1024,\"time\":0.1}");
        let serialized = serde_json::to_string(&init).expect("failed to serialize `MpiInit`");

        assert_eq!(json, serialized);
    }

    #[test]
    fn deserializes() {
        let init = MpiInitBuilder::default()
            .current_rank(0)
            .tsc(1024)
            .time(0.1)
            .build()
            .expect("failed to build `MpiInit`");
        let serialized =
            serde_json::to_string_pretty(&init).expect("failed to serialize `MpiInit`");
        let deserialized: MpiInit =
            serde_json::from_str(&serialized).expect("failed to deserialize `MpiInit`");

        assert_eq!(init, deserialized);
    }
}
