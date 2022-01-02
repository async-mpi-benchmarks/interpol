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

#[no_mangle]
extern "C" fn register_init(cycles: u64, time: f64) {
    TRACES
        .lock()
        .expect("Failed to take the lock")
        .push(Event::Init(Init::new(cycles, time)));
}

#[no_mangle]
extern "C" fn register_send(
    cycles_lo: u64,
    cycles_hi: u64,
    bytes: usize,
    comm: MpiComm,
    current_rank: i32,
    partner_rank: i32,
    tag: i32,
) {
    TRACES
        .lock()
        .expect("Failed to take the lock")
        .push(Event::Send(Blocking::new(
            cycles_lo,
            cycles_hi,
            bytes,
            comm,
            current_rank,
            partner_rank,
            tag,
        )));
}

#[no_mangle]
extern "C" fn register_recv(
    cycles_lo: u64,
    cycles_hi: u64,
    bytes: usize,
    comm: MpiComm,
    current_rank: i32,
    partner_rank: i32,
    tag: i32,
) {
    TRACES
        .lock()
        .expect("Failed to take the lock")
        .push(Event::Recv(Blocking::new(
            cycles_lo,
            cycles_hi,
            bytes,
            comm,
            current_rank,
            partner_rank,
            tag,
        )));
}

#[no_mangle]
extern "C" fn register_isend(
    cycles_lo: u64,
    cycles_hi: u64,
    bytes: usize,
    comm: MpiComm,
    req: MpiReq,
    current_rank: i32,
    partner_rank: i32,
    tag: i32,
) {
    TRACES
        .lock()
        .expect("Failed to take the lock")
        .push(Event::Isend(NonBlocking::new(
            cycles_lo,
            cycles_hi,
            bytes,
            comm,
            req,
            current_rank,
            partner_rank,
            tag,
        )));
}

#[no_mangle]
extern "C" fn register_irecv(
    cycles_lo: u64,
    cycles_hi: u64,
    bytes: usize,
    comm: MpiComm,
    req: MpiReq,
    current_rank: i32,
    partner_rank: i32,
    tag: i32,
) {
    TRACES
        .lock()
        .expect("Failed to take the lock")
        .push(Event::Irecv(NonBlocking::new(
            cycles_lo,
            cycles_hi,
            bytes,
            comm,
            req,
            current_rank,
            partner_rank,
            tag,
        )));
}

#[no_mangle]
extern "C" fn register_wait(
    cycles_lo: u64,
    cycles_hi: u64,
    req: MpiReq,
    current_rank: i32,
) {
    TRACES
        .lock()
        .expect("Failed to take the lock")
        .push(Event::Wait(Wait::new(
            cycles_lo,
            cycles_hi,
            req,
            current_rank,
        )));
}

#[no_mangle]
extern "C" fn register_finalize(cycles: u64, time: f64, rank: i32) {
    TRACES
        .lock()
        .expect("Failed to take the lock")
        .push(Event::Finalize(Finalize::new(cycles, time, rank)));

    let guard = TRACES.lock().unwrap();

    let serialized = serde_json::to_string_pretty(&*guard).expect("Failed to serialize");
    let filename = format!("rank{}_traces.json", rank.to_string());
    let mut file = File::create(filename).expect("Failed to create serialization file");
    write!(file, "{}", serialized).expect("Failed to write to serialization file");
}
