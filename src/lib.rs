pub mod event;
pub mod trace;

type MpiComm = i64;
type MpiReq = i64;

pub const WORLD: MpiComm = 0;
