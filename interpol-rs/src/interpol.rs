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
    static ref TRACES: Mutex<Vec<Event>> = Mutex::new(vec![]);
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

/// Registers an `MPI_Wait` call into the static `TRACES` vector.
#[no_mangle]
pub extern "C" fn register_wait(cycles_lo: u64, cycles_hi: u64, req: MpiReq, current_rank: i32) {
    let wait_event = Event::Wait(Wait::new(cycles_lo, cycles_hi, req, current_rank));
    TRACES.lock().unwrap().push(wait_event);
}

/// Registers an `MPI_Finalize` call into the static `TRACES` vector.
///
/// As this should be the final registered event, serialize the contents of the `TRACES` vector.
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
