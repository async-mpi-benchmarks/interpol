extern crate serde;
extern crate serde_json;

use crate::{MpiComm, MpiReq};
use serde::{Deserialize, Serialize};

/// A structure that stores information about `MPI_Init` calls.
///
/// Stores the current time in milliseconds and the current number of CPU cycles.
/// The latter is get from the C `sync_rdtscp` function which uses the `rdtscp` and
/// `cpuid` instructions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Init {
    cycles: u64,
    time: f64,
}

impl Init {
    /// Creates a new `Init` structure from a number of cycles and a time in milliseconds.
    pub fn new(cycles: u64, time: f64) -> Self {
        Init { cycles, time }
    }
}

/// A structure that stores information about `MPI_Finalize` calls.
///
/// Stores the rank of the process calling `MPI_Finalize` as well as the current time
/// in milliseconds and the current number of CPU cycles.
/// The latter is get from the C `sync_rdtscp` function which uses the `rdtscp` and
/// `cpuid` instructions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Finalize {
    cycles: u64,
    time: f64,
    current_rank: i32,
}

impl Finalize {
    /// Creates a new `Finalize` structure from a rank, a number of cycles and a time in
    /// milliseconds.
    pub fn new(cycles: u64, time: f64, current_rank: i32) -> Self {
        Finalize {
            cycles,
            time,
            current_rank,
        }
    }
}

/// A structure that stores information about synchronous MPI calls.
///
/// Currently only allows to store data from `MPI_Send` and `MPI_Recv` calls.
/// 
/// Stores the number of cycles before and after calling the MPI function, the number of bytes
/// exchanged, the MPI communicator, the rank of the process making the call, the rank of the
/// partner process and the tag for the exchange.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Blocking {
    cycles_lo: u64,
    cycles_hi: u64,
    bytes: usize,
    comm: MpiComm,
    current_rank: i32,
    partner_rank: i32,
    tag: i32,
}

impl Blocking {
    /// Creates a new `Blocking` structure from the specified parameters.
    pub fn new(
        cycles_lo: u64,
        cycles_hi: u64,
        bytes: usize,
        comm: MpiComm,
        current_rank: i32,
        partner_rank: i32,
        tag: i32,
    ) -> Self {
        Blocking {
            cycles_lo,
            cycles_hi,
            bytes,
            comm,
            current_rank,
            partner_rank,
            tag,
        }
    }
}

/// A structure that stores information about asynchronous MPI calls.
///
/// Currently only allows to store data from `MPI_Isend` and `MPI_Irecv` calls.
/// 
/// Stores the number of cycles before and after calling the MPI function, the number of bytes
/// exchanged, the MPI communicator and MPI request qualifying the call, the rank of the
/// process making the call, the rank of the partner process and the tag for the exchange.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct NonBlocking {
    cycles_lo: u64,
    cycles_hi: u64,
    bytes: usize,
    comm: MpiComm,
    req: MpiReq,
    current_rank: i32,
    partner_rank: i32,
    tag: i32,
}

impl NonBlocking {
    /// Creates a new `NonBlocking` structure from the specified parameters.
    pub fn new(
        cycles_lo: u64,
        cycles_hi: u64,
        bytes: usize,
        comm: MpiComm,
        req: MpiReq,
        current_rank: i32,
        partner_rank: i32,
        tag: i32,
    ) -> Self {
        NonBlocking {
            cycles_lo,
            cycles_hi,
            bytes,
            comm,
            req,
            current_rank,
            partner_rank,
            tag,
        }
    }
}

/// A structure that stores information about `MPI_Wait` calls.
///
/// Stores the number of cycles before and after calling the MPI function, the MPI request
/// qualifying the call and the rank of the process making the call.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Wait {
    cycles_lo: u64,
    cycles_hi: u64,
    req: MpiReq,
    current_rank: i32,
}

impl Wait {
    /// Creates a new `Wait` structure from the specified parameters.
    pub fn new(
        cycles_lo: u64,
        cycles_hi: u64,
        req: MpiReq,
        current_rank: i32,
    ) -> Self {
        Wait {
            cycles_lo,
            cycles_hi,
            req,
            current_rank,
        }
    }
}

/// Information on a MPI event.
///
/// This enum is used to store data on an MPI call, primarily in a `Vec`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Event {
    Init(Init),
    Send(Blocking),
    Recv(Blocking),
    Isend(NonBlocking),
    Irecv(NonBlocking),
    Wait(Wait),
    Finalize(Finalize),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WORLD;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn builds() {
        let e = Event::Send(Blocking::new(132, 243, 216, WORLD, 1, 2, 0));

        if let Event::Send(Blocking {
            cycles_lo,
            cycles_hi,
            bytes,
            comm,
            tag,
            current_rank,
            partner_rank,
        }) = e
        {
            assert_eq!(cycles_lo, 132);
            assert_eq!(cycles_hi, 243);
            assert_eq!(bytes, 216);
            assert_eq!(comm, WORLD);
            assert_eq!(tag, 0);
            assert_eq!(current_rank, 1);
            assert_eq!(partner_rank, 2);
        } else {
            unreachable!();
        }
    }

    #[test]
    fn serialize() {
        let e = Event::Init(Init::new(123, 0.44));
        let json = String::from("{\"Init\":{\"cycles\":123,\"time\":0.44}}");
        let serialized = serde_json::to_string(&e).expect("Failed to serialize");
        assert_eq!(json, serialized);
    }

    #[test]
    fn deserialize() {
        let e = Event::Isend(NonBlocking::new(132, 243, 477, WORLD, 0, 0, 1, 0));
        let serialized = serde_json::to_string_pretty(&e).expect("Failed to serialize");
        let deserialized: Event = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(e, deserialized);
    }

    #[test]
    fn multiple_events() {
        let mut t = Vec::new();
        let e = Event::Irecv(NonBlocking::new(1, 0, 8, WORLD, 0, 0, 69, 420));

        t.push(e.clone());
        assert_eq!(t[0], e);
    }

    #[test]
    fn serialize_async() {
        let mut t = Vec::new();
        t.push(Event::Init(Init::new(2, 0.78)));
        t.push(Event::Isend(NonBlocking::new(9, 19, 64, WORLD, 0, 0, 1, 0)));
        t.push(Event::Wait(Wait::new(20, 27, 0, 0)));
        t.push(Event::Irecv(NonBlocking::new(
            69, 420, 64, WORLD, 1, 0, 1, 1,
        )));
        t.push(Event::Wait(Wait::new(555, 567, 1, 0)));
        t.push(Event::Finalize(Finalize::new(978, 1024f64, 0)));

        let serialized = serde_json::to_string_pretty(&t).expect("Failed to serialize");
        let mut file = File::create("./target/test_async.json").unwrap();
        write!(file, "{}", serialized).unwrap();

        assert_eq!(
            std::path::Path::new("./target/test_async.json").exists(),
            true
        );
    }

    #[test]
    fn serialize_sync() {
        let mut t = Vec::new();
        t.push(Event::Init(Init::new(2, 0.78)));
        t.push(Event::Send(Blocking::new(9, 19, 246, WORLD, 0, 1, 0)));
        t.push(Event::Recv(Blocking::new(69, 420, 246, WORLD, 1, 0, 1)));
        t.push(Event::Finalize(Finalize::new(978, 1024f64, 0)));

        let serialized = serde_json::to_string_pretty(&t).expect("Failed to serialize");
        let mut file = File::create("./target/test_sync.json").unwrap();
        write!(file, "{}", serialized).unwrap();

        assert_eq!(
            std::path::Path::new("./target/test_sync.json").exists(),
            true
        );
    }
}
