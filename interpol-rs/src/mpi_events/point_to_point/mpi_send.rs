use crate::{
    interpol::Register,
    types::{MpiComm, MpiRank, MpiTag, Tsc},
};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::TryReserveError;

/// A structure that stores information about `MPI_Send` calls.
///
/// The following data is gathered when the MPI function is called:
/// - the rank of the processes involved in the communication;
/// - the number of bytes exchanged;
/// - the identifier of the MPI communicator;
/// - the tag of the communication;
/// - the current value of the Time Stamp counter before the call to `MPI_Send`.
/// - the duration of the call.
/// The TSC is measured using the `rdtscp` and `lfence` instructions (see Intel documentation for
/// further information).
#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MpiSend {
    current_rank: MpiRank,
    partner_rank: MpiRank,
    nb_bytes: u32,
    comm: MpiComm,
    tag: MpiTag,
    tsc: Tsc,
    duration: Tsc,
}

impl MpiSend {
    /// Creates a new `MpiSend` structure based off of the specified parameters.
    pub fn new(
        current_rank: MpiRank,
        partner_rank: MpiRank,
        nb_bytes: u32,
        comm: MpiComm,
        tag: MpiTag,
        tsc: Tsc,
        duration: Tsc,
    ) -> Self {
        Self {
            current_rank,
            partner_rank,
            nb_bytes,
            comm,
            tag,
            tsc,
            duration,
        }
    }
}

#[typetag::serde]
impl Register for MpiSend {
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
    const MPI_COMM_WORLD: i32 = 0;

    #[test]
    fn builds() {
        let send_new = MpiSend::new(0, 1, 8, MPI_COMM_WORLD, 42, 1024, 2048);
        let send_builder = MpiSendBuilder::default()
            .current_rank(0)
            .partner_rank(1)
            .nb_bytes(8)
            .comm(MPI_COMM_WORLD)
            .tag(42)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiSend`");

        assert_eq!(send_new, send_builder);
    }

    #[test]
    fn serializes() {
        let send = MpiSend::new(0, 1, 8, MPI_COMM_WORLD, 42, 1024, 2048);
        let json = String::from("{\"current_rank\":0,\"partner_rank\":1,\"nb_bytes\":8,\"comm\":0,\"tag\":42,\"tsc\":1024,\"duration\":2048}");
        let serialized = serde_json::to_string(&send).expect("failed to serialize `MpiSend`");

        assert_eq!(json, serialized);
    }

    #[test]
    fn deserializes() {
        let send = MpiSendBuilder::default()
            .current_rank(0)
            .partner_rank(1)
            .nb_bytes(8)
            .comm(MPI_COMM_WORLD)
            .tag(42)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiSend`");
        let serialized =
            serde_json::to_string_pretty(&send).expect("failed to serialize `MpiSend`");
        let deserialized: MpiSend =
            serde_json::from_str(&serialized).expect("failed to deserialize `MpiSend`");

        assert_eq!(send, deserialized);
    }
}
