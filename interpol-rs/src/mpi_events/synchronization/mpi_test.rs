use crate::types::{MpiRank, MpiReq, Tsc};
use crate::{impl_builder_error, impl_register};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A structure that stores information about `MPI_Test` calls.
///
/// The following data is gathered when the MPI function is called:
/// - the rank of the process;
/// - the request that was tested;
/// - wheter the requested communication had finished;
/// - the current value of the Time Stamp counter before the call to `MPI_Test`;
/// - the duration of the call.
/// The TSC is measured using the `rdtscp` and `lfence` instructions (see Intel documentation for
/// further information).
#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MpiTest {
    current_rank: MpiRank,
    req: MpiReq,
    finished: bool,
    tsc: Tsc,
    duration: Tsc,
}

impl MpiTest {
    /// Creates a new `MpiTest` structure from the specified parameters.
    pub fn new(
        current_rank: MpiRank,
        req: MpiReq,
        finished: bool,
        tsc: Tsc,
        duration: Tsc,
    ) -> Self {
        Self {
            current_rank,
            req,
            finished,
            tsc,
            duration,
        }
    }
}

impl_builder_error!(MpiTestBuilderError);
impl_register!(MpiTest);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds() {
        let test_new = MpiTest::new(0, 0, false, 1024, 2048);
        let test_builder = MpiTestBuilder::default()
            .current_rank(0)
            .req(0)
            .finished(false)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiTest`");

        assert_eq!(test_new, test_builder);
    }

    #[test]
    fn serializes() {
        let test = MpiTest::new(0, 0, true, 1024, 2048);
        let json = String::from(
            "{\"current_rank\":0,\"req\":0,\"finished\":true,\"tsc\":1024,\"duration\":2048}",
        );
        let serialized = serde_json::to_string(&test).expect("failed to serialize `MpiTest`");

        assert_eq!(json, serialized);
    }

    #[test]
    fn deserializes() {
        let test = MpiTestBuilder::default()
            .current_rank(0)
            .req(0)
            .finished(true)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiTest`");
        let serialized =
            serde_json::to_string_pretty(&test).expect("failed to serialize `MpiTest`");
        let deserialized: MpiTest =
            serde_json::from_str(&serialized).expect("failed to deserialize `MpiTest`");

        assert_eq!(test, deserialized);
    }
}
