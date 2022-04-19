use crate::interpol::Register;
use crate::types::{MpiRank, Tsc, Usecs};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::TryReserveError;

/// A structure that stores information about `MPI_Init_thread` calls.
///
/// The following data is gathered when the MPI function is called:
/// - the rank of the process;
/// - the number of CPU cycles;
/// - the time in microseconds;
/// - the provided thread level desired by the caller.
/// The TSC is measured using the `rdtscp` and `lfence` instructions (see Intel documentation for
/// further information).
#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MpiInitThread {
    rank: MpiRank,
    tsc: Tsc,
    time: Usecs,
    provided_thread_lvl: i32,
}

impl MpiInitThread {
    /// Creates a new `MpiInitThread` structure based off of a `MpiRank`, a number of CPU cycles, a
    /// time in microseconds and the provided thread level support.
    pub fn new(rank: MpiRank, tsc: Tsc, time: Usecs, provided_thread_lvl: i32) -> Self {
        Self {
            rank,
            tsc,
            time,
            provided_thread_lvl,
        }
    }
}

#[typetag::serde]
impl Register for MpiInitThread {
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

    const MPI_THREAD_SINGLE: i32 = 0;
    // const MPI_THREAD_FUNNELED: i32 = 1;
    const MPI_THREAD_SERIALIZED: i32 = 2;
    const MPI_THREAD_MULTIPLE: i32 = 3;

    #[test]
    fn builds() {
        let init_thread_new = MpiInitThread::new(0, 1024, 0.1, MPI_THREAD_SINGLE);
        let init_thread_builder = MpiInitThreadBuilder::default()
            .rank(0)
            .tsc(1024)
            .time(0.1)
            .provided_thread_lvl(MPI_THREAD_SINGLE)
            .build()
            .expect("failed to build `MpiInitThread`");

        assert_eq!(init_thread_new, init_thread_builder);
    }

    #[test]
    fn serializes() {
        let init_thread = MpiInitThread::new(0, 1024, 0.1, MPI_THREAD_SERIALIZED);
        let json =
            String::from("{\"rank\":0,\"tsc\":1024,\"time\":0.1,\"provided_thread_lvl\":2}");
        let serialized =
            serde_json::to_string(&init_thread).expect("failed to serialize `MpiInitThread`");

        assert_eq!(json, serialized);
    }

    #[test]
    fn deserializes() {
        let init_thread = MpiInitThreadBuilder::default()
            .rank(0)
            .tsc(1024)
            .time(0.1)
            .provided_thread_lvl(MPI_THREAD_MULTIPLE)
            .build()
            .expect("failed to build `MpiInitThread`");

        let serialized = serde_json::to_string_pretty(&init_thread)
            .expect("failed to serialize `MpiInitThread`");
        let deserialized: MpiInitThread =
            serde_json::from_str(&serialized).expect("failed to deserialize `MpiInitThread`");

        assert_eq!(init_thread, deserialized);
    }
}
