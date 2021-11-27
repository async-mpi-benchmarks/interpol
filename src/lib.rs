type MPIComm = i64; 
type MPIReq = i64;

#[derive(Debug)]
enum MPIColl {
    Isend(i32),
    Irecv(i32),
    Wait,
    Send(i32),
    Recv(i32),
}

#[derive(Debug)]
#[repr(C)]
struct Event {
    time_b: i32,
    time_e: i32,
    rank: i32,
    dest: i32,
    comm: MPIComm,
    req: MPIReq,
    coll: MPIColl,
}

struct list
{
    v: Vec<Event>;
}

fn init() -> Vec<Event>
{
    let v = Vec::new();
}

impl Event
{

    fn new_send(time_b: i32, time_e: i32, rank: i32, dest:i32, comm: MPIComm, req: MPIReq, coll: MPIColl, tag: i32)
    {
        let e = Event{time_b, time_e, rank, dest, comm, req, coll: MPIColl::Send(i32::from(tag))};
        v.push(e);
    }
}

