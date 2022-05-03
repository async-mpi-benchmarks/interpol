use crate::interpol::Register;
use crate::types::{MpiComm, MpiRank, MpiReq, Tsc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::TryReserveError;

/// A structure that stores information about `MPI_Ireduce` calls.
///
/// The information stored are:
/// - the rank of the process making the call to `MPI_Ireduce`;
/// - the rank of the root process making the broadcast;
/// - the number of bytes exchanged;
/// - the type of MPI reduction operation;
/// - the identifier of the MPI communicator;
/// - the identifier of the MPI request;
/// - the tag of the communication;
/// - the current value of the Time Stamp counter before the call to `MPI_Ireduce`.
/// - the duration of the call.
#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MpiIreduce {
    current_rank: MpiRank,
    root_rank: MpiRank,
    nb_bytes: u32,
    op_type: u8,
    comm: MpiComm,
    req: MpiReq,
    tsc: Tsc,
    duration: Tsc,
}

impl MpiIreduce {
    /// Creates a new `MpiIreduce` structure from the specified parameters.
    pub fn new(
        current_rank: MpiRank,
        root_rank: MpiRank,
        nb_bytes: u32,
        op_type: u8,
        comm: MpiComm,
        req: MpiReq,
        tsc: Tsc,
        duration: Tsc,
    ) -> Self {
        MpiIreduce {
            current_rank,
            root_rank,
            nb_bytes,
            op_type,
            comm,
            req,
            tsc,
            duration,
        }
    }
}

#[typetag::serde]
impl Register for MpiIreduce {
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

    enum MpiOpType {
        Max,
        Sum,
        Prod,
    }

    #[test]
    fn builds() {
        let ireduce_new =
            MpiIreduce::new(0, 1, 8, MpiOpType::Sum as u8, MPI_COMM_WORLD, 7, 1024, 2048);
        let ireduce_builder = MpiIreduceBuilder::default()
            .current_rank(0)
            .root_rank(1)
            .nb_bytes(8)
            .op_type(MpiOpType::Sum as u8)
            .comm(MPI_COMM_WORLD)
            .req(7)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiIreduce`");

        assert_eq!(ireduce_new, ireduce_builder);
    }

    #[test]
    fn serializes() {
        let ireduce = MpiIreduce::new(
            0,
            0,
            8,
            MpiOpType::Prod as u8,
            MPI_COMM_WORLD,
            7,
            1024,
            2048,
        );
        let json = String::from("{\"current_rank\":0,\"root_rank\":0,\"nb_bytes\":8,\"op_type\":2,\"comm\":0,\"req\":7,\"tsc\":1024,\"duration\":2048}");
        let serialized = serde_json::to_string(&ireduce).expect("failed to serialize `MpiIreduce`");

        assert_eq!(json, serialized);
    }

    #[test]
    fn deserializes() {
        let ireduce = MpiIreduceBuilder::default()
            .current_rank(1)
            .root_rank(0)
            .nb_bytes(8)
            .op_type(MpiOpType::Max as u8)
            .comm(MPI_COMM_WORLD)
            .req(7)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiIreduce`");
        let serialized =
            serde_json::to_string_pretty(&ireduce).expect("failed to serialize `MpiIreduce`");
        let deserialized: MpiIreduce =
            serde_json::from_str(&serialized).expect("failed to deserialize `MpiIreduce`");

        assert_eq!(ireduce, deserialized);
    }
}
