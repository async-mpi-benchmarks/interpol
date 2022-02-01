extern crate lazy_static;
extern crate serde;
extern crate serde_json;

use crate::event::*;
use crate::{MpiComm, MpiReq};
use lazy_static::lazy_static;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

lazy_static! {
    /// A vector that keeps track of interposed MPI functions called by a process.
    ///
    /// The `lazy_static` macro creates static objects that are only initialized when
    /// needed at runtime. In our case, this implementation is similar to a singleton.
    /// It removes the need to pass a constant pointer on the `Vec` of traces back to
    /// the C part of the interposition library, therefore avoiding the use of `unsafe`
    /// code sections.
    ///
    /// As the MPI standard allows for processes to run code in parallel (e.g. through
    /// libraries like OpenMP or pthread), the `Vec` *must* be wrapped in a `Mutex` to
    /// prevent concurrent attempts at pushing onto the traces vector. Each time an event
    /// is registered, the caller must first take the lock on the `Mutex` before pushing
    /// an `Event`.
    ///
    /// We have chosen to implement mutual exclusion in the Rust part of the interposition
    /// library to reduce the critical section of code to the minimum, i.e. when a MPI call
    /// has been registered and *needs* to be saved. This choice theoretically allows for
    /// the best safety/performance ratio.
    ///
    /// It should be noted that in a MPI context, it is "rare" that the same process manages
    /// a large number of threads. Therefore, the contention on the `Mutex` should not
    /// impact the performance of the application and the blocking of threads will be kept
    /// to a minimum.
    static ref TRACES: Mutex<Vec<Event>> = Mutex::new(Vec::new());
}

/// Registers an `MPI_Init` call into the static `TRACES` vector.
#[no_mangle]
pub extern "C" fn register_init(cycles: u64, time: f64) {
    let init_event = Event::Init(Init::new(cycles, time));
    TRACES.lock().unwrap().push(init_event);
}

/// Registers an `MPI_Send` call into the static `TRACES` vector.
#[no_mangle]
pub extern "C" fn register_send(
    cycles_lo: u64,
    cycles_hi: u64,
    bytes: usize,
    comm: MpiComm,
    current_rank: i32,
    partner_rank: i32,
    tag: i32,
) {
    let send_event = Event::Send(Blocking::new(
        cycles_lo,
        cycles_hi,
        bytes,
        comm,
        current_rank,
        partner_rank,
        tag,
    ));
    TRACES.lock().unwrap().push(send_event);
}

/// Registers an `MPI_Recv` call into the static `TRACES` vector.
#[no_mangle]
pub extern "C" fn register_recv(
    cycles_lo: u64,
    cycles_hi: u64,
    bytes: usize,
    comm: MpiComm,
    current_rank: i32,
    partner_rank: i32,
    tag: i32,
) {
    let recv_event = Event::Recv(Blocking::new(
        cycles_lo,
        cycles_hi,
        bytes,
        comm,
        current_rank,
        partner_rank,
        tag,
    ));
    TRACES.lock().unwrap().push(recv_event);
}

/// Registers an `MPI_Isend` call into the static `TRACES` vector.
#[no_mangle]
pub extern "C" fn register_isend(
    cycles_lo: u64,
    cycles_hi: u64,
    bytes: usize,
    comm: MpiComm,
    req: MpiReq,
    current_rank: i32,
    partner_rank: i32,
    tag: i32,
) {
    let isend_event = Event::Isend(NonBlocking::new(
        cycles_lo,
        cycles_hi,
        bytes,
        comm,
        req,
        current_rank,
        partner_rank,
        tag,
    ));
    TRACES.lock().unwrap().push(isend_event);
}

/// Registers an `MPI_Irecv` call into the static `TRACES` vector.
#[no_mangle]
pub extern "C" fn register_irecv(
    cycles_lo: u64,
    cycles_hi: u64,
    bytes: usize,
    comm: MpiComm,
    req: MpiReq,
    current_rank: i32,
    partner_rank: i32,
    tag: i32,
) {
    let irecv_event = Event::Irecv(NonBlocking::new(
        cycles_lo,
        cycles_hi,
        bytes,
        comm,
        req,
        current_rank,
        partner_rank,
        tag,
    ));
    TRACES.lock().unwrap().push(irecv_event);
}

/// Registers an `MPI_Bcast` call into the static `TRACES` vector.
#[no_mangle]
pub extern "C" fn register_bcast(
    cycles_lo: u64,
    cycles_hi: u64,
    bytes: usize,
    comm: MpiComm,
    current_rank: i32,
    partner_rank: i32,
) {
    let bcast_event = Event::Bcast(Bcast::new(
        cycles_lo,
        cycles_hi,
        bytes,
        comm,
        current_rank,
        partner_rank,
    ));
    TRACES.lock().unwrap().push(bcast_event);
}

/// Registers an `MPI_Ibcast` call into the static `TRACES` vector.
#[no_mangle]
pub extern "C" fn register_ibcast(
    cycles_lo: u64,
    cycles_hi: u64,
    bytes: usize,
    comm: MpiComm,
    req: MpiReq,
    current_rank: i32,
    partner_rank: i32,
) {
    let ibcast_event = Event::Ibcast(Ibcast::new(
        cycles_lo,
        cycles_hi,
        bytes,
        comm,
        req,
        current_rank,
        partner_rank,
    ));
    TRACES.lock().unwrap().push(ibcast_event);
}

/// Registers an `MPI_Gather` call into the static `TRACES` vector.
#[no_mangle]
pub extern "C" fn register_gather(
    cycles_lo: u64,
    cycles_hi: u64,
    bytes_s: usize,
    bytes_r: usize,
    comm: MpiComm,
    current_rank: i32,
    partner_rank: i32,
) {
    let gather_event = Event::Gather(Gather::new(
        cycles_lo,
        cycles_hi,
        bytes_s,
        bytes_r,
        comm,
        current_rank,
        partner_rank,
    ));
    TRACES.lock().unwrap().push(gather_event);
}

/// Registers an `MPI_Igather` call into the static `TRACES` vector.
#[no_mangle]
pub extern "C" fn register_igather(
    cycles_lo: u64,
    cycles_hi: u64,
    bytes_s: usize,
    bytes_r: usize,
    comm: MpiComm,
    req: MpiReq,
    current_rank: i32,
    partner_rank: i32,
) {
    let igather_event = Event::Igather(Igather::new(
        cycles_lo,
        cycles_hi,
        bytes_s,
        bytes_r,
        comm,
        req,
        current_rank,
        partner_rank,
    ));
    TRACES.lock().unwrap().push(igather_event);
}

/// Registers an `MPI_Wait` call into the static `TRACES` vector.
#[no_mangle]
pub extern "C" fn register_wait(cycles_lo: u64, cycles_hi: u64, req: MpiReq, current_rank: i32) {
    let wait_event = Event::Wait(Wait::new(cycles_lo, cycles_hi, req, current_rank));
    TRACES.lock().unwrap().push(wait_event);
}

/// Registers an `MPI_Barrier` call into the static `TRACES` vector.
#[no_mangle]
pub extern "C" fn register_barrier(cycles_lo: u64, cycles_hi: u64, comm: MpiComm, current_rank: i32) {
    let barrier_event = Event::Barrier(Barrier::new(cycles_lo, cycles_hi, comm, current_rank));
    TRACES.lock().unwrap().push(barrier_event);
}

/// Registers an `MPI_Finalize` call into the static `TRACES` vector.
///
/// As this *should* be the final registered event, serializes the contents of the `TRACES` vector.
#[no_mangle]
pub extern "C" fn register_finalize(cycles: u64, time: f64, rank: i32) {
    let finalize_event = Event::Finalize(Finalize::new(cycles, time, rank));
    TRACES.lock().unwrap().push(finalize_event);

    // Take the lock on the `Mutex<Vec<T>>` before serializing
    let guard = TRACES.lock().unwrap();

    // Serialize the contents of the `Vec` and write them to an output file
    let ser_traces = serde_json::to_string_pretty(&*guard).unwrap();
    let mut file = File::create(format!("rank{}_traces.json", rank.to_string())).unwrap();
    write!(file, "{}", ser_traces).unwrap();
}
