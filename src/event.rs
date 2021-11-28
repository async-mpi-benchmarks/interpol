use serde::{Deserialize, Serialize};

type Time = u64;
type MpiComm = i64;
type MpiReq = i64;

pub const WORLD: MpiComm = 0;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MpiOp {
    Init,
    Finalize {
        current_rank: i32,
    },
    Isend {
        current_rank: i32,
        partner_rank: i32,
        tag: i32,
        req: MpiReq,
    },
    Irecv {
        current_rank: i32,
        partner_rank: i32,
        tag: i32,
        req: MpiReq,
    },
    Send {
        current_rank: i32,
        partner_rank: i32,
        tag: i32,
    },
    Recv {
        current_rank: i32,
        partner_rank: i32,
        tag: i32,
    },
    Wait {
        current_rank: i32,
        req: MpiReq,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Event {
    op: MpiOp,
    comm: Option<MpiComm>,
    before: Time,
    after: Time,
}

impl Event {
    pub fn new(op: MpiOp, comm: Option<MpiComm>, before: Time, after: Time) -> Self {
        Event {
            op,
            comm,
            before,
            after,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds() {
        let e = Event::new(
            MpiOp::Send {
                current_rank: 1,
                partner_rank: 2,
                tag: 0,
            },
            Some(WORLD),
            132,
            243,
        );

        assert_eq!(
            e.op,
            MpiOp::Send {
                current_rank: 1,
                partner_rank: 2,
                tag: 0
            }
        );
        assert_eq!(e.comm, Some(WORLD));
        assert_eq!(e.before, 132);
        assert_eq!(e.after, 243);
    }

    #[test]
    fn serialize() {
        let e = Event::new(MpiOp::Init, None, 132, 243);
        let json = String::from("{\"op\":\"Init\",\"comm\":null,\"before\":132,\"after\":243}");

        let serialized = serde_json::to_string(&e).expect("Failed to serialize");
        assert_eq!(json, serialized);
    }

    #[test]
    fn deserialize() {
        let e = Event::new(
            MpiOp::Isend {
                current_rank: 0,
                partner_rank: 1,
                req: 0,
                tag: 0,
            },
            Some(WORLD),
            132,
            243,
        );

        let serialized = serde_json::to_string_pretty(&e).expect("Failed to serialize");
        println!("{}", serialized);

        let deserialized: Event = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(e, deserialized);
    }
}
