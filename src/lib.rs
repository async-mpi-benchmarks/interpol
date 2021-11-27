use serde::{Deserialize, Serialize};

type Time = u64;
type MpiComm = i64;
type MpiReq = i64;

pub const WORLD: MpiComm = 0;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MpiOp {
    Init,
    Finalize,
    Isend(MpiReq, i32),
    Irecv(MpiReq, i32),
    Send(i32),
    Recv(i32),
    Wait(MpiReq),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Event {
    op: MpiOp,
    comm: Option<MpiComm>,
    before: Time,
    after: Time,
    rank: Option<i32>,
    dest: Option<i32>,
}

impl Event {
    pub fn new(op: MpiOp, comm: Option<MpiComm>, before: Time, after: Time, rank: Option<i32>, dest: Option<i32>) -> Self {
        Event {
            op,
            comm,
            before,
            after,
            rank,
            dest,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds() {
        let e = Event::new(MpiOp::Send(1), Some(WORLD), 132, 243, Some(0), Some(1));

        assert_eq!(e.op, MpiOp::Send(1));
        assert_eq!(e.comm, Some(WORLD));
        assert_eq!(e.before, 132);
        assert_eq!(e.after, 243);
        assert_eq!(e.rank, Some(0));
        assert_eq!(e.dest, Some(1));
    }

    #[test]
    fn serialize() {
        let e = Event::new(MpiOp::Init, None, 132, 243, None, None);
        let json = String::from("{\"op\":\"Init\",\"comm\":null,\"before\":132,\"after\":243,\"rank\":null,\"dest\":null}");
        let serialized = serde_json::to_string(&e).expect("Failed to serialize");
        assert_eq!(json, serialized);
    }

    #[test]
    fn deserialize() {
        let e = Event::new(MpiOp::Isend(1, 3), Some(WORLD), 132, 243, Some(0), Some(1));
        let serialized = serde_json::to_string_pretty(&e).expect("Failed to serialize");
        let deserialized: Event = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(e, deserialized);
    }
}
