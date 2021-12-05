use crate::{MpiComm, MpiReq};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Init {
    cycles: u64,
    time: f64,
}

impl Init {
    pub fn new(cycles: u64, time: f64) -> Self {
        Init { cycles, time }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Finalize {
    current_rank: i32,
    cycles: u64,
    time: f64,
}

impl Finalize {
    pub fn new(current_rank: i32, cycles: u64, time: f64) -> Self {
        Finalize { current_rank, cycles, time }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Blocking {
    current_rank: i32,
    partner_rank: i32,
    comm: MpiComm,
    tag: i32,
    cycles_lo: u64,
    cycles_hi: u64,
}

impl Blocking {
    pub fn new(
        current_rank: i32,
        partner_rank: i32,
        comm: MpiComm,
        tag: i32,
        cycles_lo: u64,
        cycles_hi: u64
    ) -> Self {
        Blocking {
            current_rank,
            partner_rank,
            comm,
            tag,
            cycles_lo,
            cycles_hi,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct NonBlocking {
    current_rank: i32,
    partner_rank: i32,
    comm: MpiComm,
    tag: i32,
    req: MpiReq,
    cycles_lo: u64,
    cycles_hi: u64,
}

impl NonBlocking {
    pub fn new(
        current_rank: i32,
        partner_rank: i32,
        comm: MpiComm,
        tag: i32,
        req: MpiReq,
        cycles_lo: u64,
        cycles_hi: u64
    ) -> Self {
        NonBlocking {
            current_rank,
            partner_rank,
            comm,
            tag,
            req,
            cycles_lo,
            cycles_hi,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Wait {
    current_rank: i32,
    comm: MpiComm,
    req: MpiReq,
    cycles_lo: u64,
    cycles_hi: u64,
}

impl Wait {
    pub fn new(
        current_rank: i32,
        comm: MpiComm,
        req: MpiReq,
        cycles_lo: u64,
        cycles_hi: u64
    ) -> Self {
        Self {
            current_rank,
            comm,
            req,
            cycles_lo,
            cycles_hi,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Event {
    Init(Init),
    Finalize(Finalize),
    Send(Blocking),
    Recv(Blocking),
    Isend(NonBlocking),
    Irecv(NonBlocking),
    Wait(Wait),
}

#[cfg(test)]
mod tests {
    use crate::WORLD;
    use super::*;

    #[test]
    fn builds() {
        let e = Event::Send(Blocking::new(1, 2, WORLD, 0, 132, 243));

        if let Event::Send(Blocking { current_rank, partner_rank, comm, tag, cycles_lo, cycles_hi }) = e {
            assert_eq!(current_rank, 1);
            assert_eq!(partner_rank, 2);
            assert_eq!(comm, WORLD);
            assert_eq!(tag, 0);
            assert_eq!(cycles_lo, 132);
            assert_eq!(cycles_hi, 243);
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
        let e = Event::Isend(NonBlocking::new(0, 1, WORLD, 0, 0, 132, 243));
        let serialized = serde_json::to_string_pretty(&e).expect("Failed to serialize");
        let deserialized: Event = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(e, deserialized);
    }
}
