use crate::types::{MpiRank, Tsc, Usecs};
use crate::{impl_builder_error, impl_register};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A structure that stores information about `MPI_Init_thread` calls.
///
/// The following data is gathered when the MPI function is called:
/// - the rank of the process;
/// - the current value of the Time Stamp counter;
/// - the time in microseconds;
/// - the provided thread level desired by the caller.
/// The TSC is measured using the `rdtscp` and `lfence` instructions (see Intel documentation for
/// further information).
#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MpiInitThread {
    current_rank: MpiRank,
    required_thread_lvl: i32,
    provided_thread_lvl: i32,
    tsc: Tsc,
    time: Usecs,
}

impl MpiInitThread {
    /// Creates a new `MpiInitThread` structure based off of a `MpiRank`, a number of CPU cycles, a
    /// time in microseconds and the provided thread level support.
    pub fn new(
        current_rank: MpiRank,
        required_thread_lvl: i32,
        provided_thread_lvl: i32,
        tsc: Tsc,
        time: Usecs,
    ) -> Self {
        Self {
            current_rank,
            required_thread_lvl,
            provided_thread_lvl,
            tsc,
            time,
        }
    }
}

impl_builder_error!(MpiInitThreadBuilderError);
impl_register!(MpiInitThread);

#[cfg(test)]
mod tests {
    use super::*;

    const MPI_THREAD_SINGLE: i32 = 0;
    const MPI_THREAD_FUNNELED: i32 = 1;
    const MPI_THREAD_SERIALIZED: i32 = 2;
    const MPI_THREAD_MULTIPLE: i32 = 3;

    #[test]
    fn builds() {
        let init_thread_new =
            MpiInitThread::new(0, MPI_THREAD_FUNNELED, MPI_THREAD_SINGLE, 1024, 0.1);
        let init_thread_builder = MpiInitThreadBuilder::default()
            .current_rank(0)
            .required_thread_lvl(MPI_THREAD_FUNNELED)
            .provided_thread_lvl(MPI_THREAD_SINGLE)
            .tsc(1024)
            .time(0.1)
            .build()
            .expect("failed to build `MpiInitThread`");

        assert_eq!(init_thread_new, init_thread_builder);
    }

    #[test]
    fn serializes() {
        let init_thread =
            MpiInitThread::new(0, MPI_THREAD_SERIALIZED, MPI_THREAD_SERIALIZED, 1024, 0.1);
        let json = String::from("{\"current_rank\":0,\"required_thread_lvl\":2,\"provided_thread_lvl\":2,\"tsc\":1024,\"time\":0.1}");
        let serialized =
            serde_json::to_string(&init_thread).expect("failed to serialize `MpiInitThread`");

        assert_eq!(json, serialized);
    }

    #[test]
    fn deserializes() {
        let init_thread = MpiInitThreadBuilder::default()
            .current_rank(0)
            .required_thread_lvl(MPI_THREAD_MULTIPLE)
            .provided_thread_lvl(MPI_THREAD_MULTIPLE)
            .tsc(1024)
            .time(0.1)
            .build()
            .expect("failed to build `MpiInitThread`");

        let serialized = serde_json::to_string_pretty(&init_thread)
            .expect("failed to serialize `MpiInitThread`");
        let deserialized: MpiInitThread =
            serde_json::from_str(&serialized).expect("failed to deserialize `MpiInitThread`");

        assert_eq!(init_thread, deserialized);
    }
}
