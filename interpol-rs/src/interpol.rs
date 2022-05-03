use lazy_static::lazy_static;
use std::fs::File;
use std::io::Write;
use std::{collections::TryReserveError, sync::Mutex};

use crate::mpi_events::{
    management::{
        mpi_finalize::MpiFinalizeBuilder, mpi_init::MpiInitBuilder,
        mpi_init_thread::MpiInitThreadBuilder,
    },
    point_to_point::{
        mpi_irecv::MpiIrecvBuilder, mpi_isend::MpiIsendBuilder, mpi_recv::MpiRecvBuilder,
        mpi_send::MpiSendBuilder,
    },
    synchronization::{
        mpi_barrier::MpiBarrierBuilder, mpi_test::MpiTestBuilder, mpi_wait::MpiWaitBuilder,
    },
};
use crate::types::{MpiComm, MpiRank, MpiReq, MpiTag, Tsc, Usecs};

#[repr(transparent)]
pub struct Trace(Mutex<Vec<Box<dyn Register>>>);

#[typetag::serde(tag = "type")]
pub trait Register: Send + Sync {
    fn register(self, events: &mut Vec<Box<dyn Register>>) -> Result<(), TryReserveError>;
}

lazy_static! {
    /// A vector that keeps track of interposed MPI functions called by a process.
    ///
    /// The `lazy_static` macro creates static objects that are only initialized when needed at
    /// runtime. In this case, this implementation is similar to a singleton. It removes the need
    /// to pass a constant pointer on the `Vec` back to the C part of the interposition library,
    /// therefore avoiding the use of `unsafe` code sections.
    ///
    /// As the MPI standard allows for processes to run code in parallel (e.g. through libraries
    /// like OpenMP or pthread), the `Vec` *must* be wrapped in a `Mutex` to prevent concurrent
    /// attempts at pushing onto the traces vector. Each time an event is registered, the caller
    /// must first take the lock on the `Mutex` before pushing an `Event`. This ensures that
    /// `interpol-rs` is thread-safe even in `MPI_THREAD_MULTIPLE` context.
    ///
    /// We have chosen to implement mutual exclusion in the Rust part of the interposition library
    /// to reduce the critical section of code to the minimum, i.e. when a MPI call has been
    /// registered and *needs* to be saved. This choice theoretically allows for the best
    /// safety/performance ratio.
    ///
    /// It should be noted that in a MPI context, it is "rare" that the same process manages a
    /// large number of threads. Therefore, the contention on the `Mutex` should not impact the
    /// performance of the application and the blocking of threads will be kept to a minimum.
    static ref EVENTS: Trace = Trace(Mutex::new(Vec::new()));
}

#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
#[allow(dead_code)]
enum MpiCallType {
    Init,
    Initthread,
    Finalize,
    Send,
    Isend,
    Recv,
    Irecv,
    Wait,
    Test,
    Barrier,
    Ibarrier,
    Ibcast, 
    Ireduce, 
    Iscatter,
    Igather
}

#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct MpiCall {
    call: MpiCallType,
    tsc: Tsc,
    duration: Tsc,
    time: Usecs,
    nb_bytes: u32,
    comm: MpiComm,
    req: MpiReq,
    current_rank: MpiRank,
    partner_rank: *mut MpiRank,
    tag: MpiTag,
    required_thread_lvl: i32,
    provided_thread_lvl: i32,
    finished: bool,
}

#[no_mangle]
pub extern "C" fn register_mpi_call(call: MpiCall) {
    unsafe
    {
        match call.call {
            MpiCallType::Init=>register_init(call.current_rank, call.tsc, call.time),

            MpiCallType::Initthread=>register_init_thread(call.current_rank, call.tsc, call.time, call.required_thread_lvl, call.provided_thread_lvl),

            MpiCallType::Finalize=>register_finalize(call.current_rank, call.tsc, call.time),

            MpiCallType::Send=>register_send(call.current_rank, *call.partner_rank ,call.nb_bytes , call.comm , call.tag , call.tsc , call.duration),

            MpiCallType::Isend=>register_isend(call.current_rank, *call.partner_rank, call.nb_bytes, call.comm, call.req, call.tag, call.tsc, call.duration),

            MpiCallType::Recv=>register_recv(call.current_rank, *call.partner_rank, call.nb_bytes, call.comm, call.tag, call.tsc, call.duration),

            MpiCallType::Irecv=>register_irecv(call.current_rank, *call.partner_rank, call.nb_bytes, call.comm, call.req, call.tag, call.tsc, call.duration),

            MpiCallType::Wait=>register_wait(call.current_rank, call.req, call.tsc, call.duration),

            MpiCallType::Test=>register_test(call.current_rank, call.req, call.finished, call.tsc, call.duration),

            MpiCallType::Barrier=>register_barrier(call.current_rank, call.comm, call.tsc, call.duration),

            //MpiCallType::Ibarrier=>register_ibarrier(),

            //MpiCallType::Ibcast=>register_ibcast(),

            //MpiCallType::Ireduce=>register_ireduce(),

            //MpiCallType::Iscatter=>register_iscatter(),

            //MpiCallType::Igather=>register_igather(),

            _ => ()
        }
    }
}

/// Registers an `MPI_Init` call into a static vector.
#[no_mangle]
pub fn register_init(current_rank: MpiRank, tsc: Tsc, time: Usecs) {
    let init_event = match MpiInitBuilder::default()
        .current_rank(current_rank)
        .tsc(tsc)
        .time(time)
        .build()
    {
        Ok(event) => event,
        Err(err) => {
            print_err(
                current_rank,
                "failed to build `MpiInit` event",
                format!("{err:#?}").as_str(),
            );
            return;
        }
    };

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiInit` event");
    match init_event.register(&mut guard) {
        Ok(_) => (),
        Err(err) => {
            print_err(
                current_rank,
                "failed to register `MpiInit` event",
                format!("{err}").as_str(),
            );
        }
    }
}

/// Registers an `MPI_Init_thread` call into a static vector.
#[no_mangle]
pub fn register_init_thread(
    current_rank: MpiRank,
    tsc: Tsc,
    time: Usecs,
    required_thread_lvl: i32,
    provided_thread_lvl: i32,
) {
    let init_thread_event = match MpiInitThreadBuilder::default()
        .current_rank(current_rank)
        .required_thread_lvl(required_thread_lvl)
        .provided_thread_lvl(provided_thread_lvl)
        .tsc(tsc)
        .time(time)
        .build()
    {
        Ok(event) => event,
        Err(err) => {
            print_err(
                current_rank,
                "failed to build `MpiInitThread` event",
                format!("{err:#?}").as_str(),
            );
            return;
        }
    };

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiInitThread` event");
    match init_thread_event.register(&mut guard) {
        Ok(_) => (),
        Err(err) => {
            print_err(
                current_rank,
                "failed to register `MpiInitThread` event",
                format!("{err}").as_str(),
            );
        }
    }
}

/// Registers an `MPI_Finalize` call into a static vector.
///
/// As this *should* be the final registered event, the contents of the vector will be sorted with
/// every other MPI processes vectors' and then serialized.
#[no_mangle]
pub fn register_finalize(current_rank: MpiRank, tsc: Tsc, time: Usecs) {
    let finalize_event = match MpiFinalizeBuilder::default()
        .current_rank(current_rank)
        .tsc(tsc)
        .time(time)
        .build()
    {
        Ok(event) => event,
        Err(err) => {
            print_err(
                current_rank,
                "failed to build `MpiFinalize` event",
                format!("{err}").as_str(),
            );
            return;
        }
    };

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiFinalize` event");
    match finalize_event.register(&mut guard) {
        Ok(_) => (),
        Err(err) => {
            print_err(
                current_rank,
                "failed to register `MpiFinalize` event",
                format!("{err}").as_str(),
            );
        }
    }

    // Serialize the contents of the `Vec` and write them to an output file
    let ser_traces = serde_json::to_string_pretty(&*guard)
        .expect("failed to serialize vector contents to string");
    let filename = format!("/tmp/rank{}_traces.json", current_rank.to_string());
    let mut file = match File::create(filename.clone()) {
        Ok(file) => file,
        Err(err) => {
            print_err(
                current_rank,
                format!("failed to create file `{}`", filename).as_str(),
                format!("{err}").as_str(),
            );
            return;
        }
    };

    match write!(file, "{}", ser_traces) {
        Ok(_) => (),
        Err(err) => {
            print_err(
                current_rank,
                format!("failed to write to file `{}`", filename).as_str(),
                format!("{err}").as_str(),
            );
        }
    };

    if current_rank != 0 {
        return;
    }
    // TODO: Deserialize every trace files, sort and serialize everything in order.
}

/// Registers an `MPI_Send` call into a static vector.
#[no_mangle]
pub fn register_send(
    current_rank: MpiRank,
    partner_rank: MpiRank,
    nb_bytes: u32,
    comm: MpiComm,
    tag: MpiTag,
    tsc: Tsc,
    duration: Tsc,
) {
    let send_event = match MpiSendBuilder::default()
        .current_rank(current_rank)
        .partner_rank(partner_rank)
        .nb_bytes(nb_bytes)
        .comm(comm)
        .tag(tag)
        .tsc(tsc)
        .duration(duration)
        .build()
    {
        Ok(event) => event,
        Err(err) => {
            print_err(
                current_rank,
                "failed to build `MpiSend` event",
                format!("{err}").as_str(),
            );
            return;
        }
    };

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiSend` event");
    match send_event.register(&mut guard) {
        Ok(_) => (),
        Err(err) => {
            print_err(
                current_rank,
                "failed to register `MpiSend` event",
                format!("{err}").as_str(),
            );
            return;
        }
    }
}

/// Registers an `MPI_Recv` call into a static vector.
#[no_mangle]
pub fn register_recv(
    current_rank: MpiRank,
    partner_rank: MpiRank,
    nb_bytes: u32,
    comm: MpiComm,
    tag: MpiTag,
    tsc: Tsc,
    duration: Tsc,
) {
    let recv_event = match MpiRecvBuilder::default()
        .current_rank(current_rank)
        .partner_rank(partner_rank)
        .nb_bytes(nb_bytes)
        .comm(comm)
        .tag(tag)
        .tsc(tsc)
        .duration(duration)
        .build()
    {
        Ok(event) => event,
        Err(err) => {
            print_err(
                current_rank,
                "failed to build `MpiRecv` event",
                format!("{err}").as_str(),
            );
            return;
        }
    };

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiRecv` event");
    match recv_event.register(&mut guard) {
        Ok(_) => (),
        Err(err) => {
            print_err(
                current_rank,
                "failed to register `MpiRecv` event",
                format!("{err}").as_str(),
            );
            return;
        }
    }
}

/// Registers an `MPI_Isend` call into a static vector.
#[no_mangle]
pub fn register_isend(
    current_rank: MpiRank,
    partner_rank: MpiRank,
    nb_bytes: u32,
    comm: MpiComm,
    req: MpiReq,
    tag: MpiTag,
    tsc: Tsc,
    duration: Tsc,
) {
    let isend_event = match MpiIsendBuilder::default()
        .current_rank(current_rank)
        .partner_rank(partner_rank)
        .nb_bytes(nb_bytes)
        .comm(comm)
        .req(req)
        .tag(tag)
        .tsc(tsc)
        .duration(duration)
        .build()
    {
        Ok(event) => event,
        Err(err) => {
            print_err(
                current_rank,
                "failed to build `MpiIsend` event",
                format!("{err}").as_str(),
            );
            return;
        }
    };

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiIsend` event");
    match isend_event.register(&mut guard) {
        Ok(_) => (),
        Err(err) => {
            print_err(
                current_rank,
                "failed to register `MpiIsend` event",
                format!("{err}").as_str(),
            );
            return;
        }
    }
}

/// Registers an `MPI_Irecv` call into a static vector.
#[no_mangle]
pub fn register_irecv(
    current_rank: MpiRank,
    partner_rank: MpiRank,
    nb_bytes: u32,
    comm: MpiComm,
    req: MpiReq,
    tag: MpiTag,
    tsc: Tsc,
    duration: Tsc,
) {
    let irecv_event = match MpiIrecvBuilder::default()
        .current_rank(current_rank)
        .partner_rank(partner_rank)
        .nb_bytes(nb_bytes)
        .comm(comm)
        .req(req)
        .tag(tag)
        .tsc(tsc)
        .duration(duration)
        .build()
    {
        Ok(event) => event,
        Err(err) => {
            print_err(
                current_rank,
                "failed to build `MpiIrecv` event",
                format!("{err}").as_str(),
            );
            return;
        }
    };

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiIrecv` event");
    match irecv_event.register(&mut guard) {
        Ok(_) => (),
        Err(err) => {
            print_err(
                current_rank,
                "failed to register `MpiIrecv` event",
                format!("{err}").as_str(),
            );
            return;
        }
    }
}

/// Registers an `MPI_Barrier` call into a static vector.
#[no_mangle]
pub fn register_barrier(current_rank: MpiRank, comm: MpiComm, tsc: Tsc, duration: Tsc) {
    let barrier_event = match MpiBarrierBuilder::default()
        .current_rank(current_rank)
        .comm(comm)
        .tsc(tsc)
        .duration(duration)
        .build()
    {
        Ok(event) => event,
        Err(err) => {
            print_err(
                current_rank,
                "failed to build `MpiBarrier` event",
                format!("{err}").as_str(),
            );
            return;
        }
    };

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiBarrier` event");
    match barrier_event.register(&mut guard) {
        Ok(_) => (),
        Err(err) => {
            print_err(
                current_rank,
                "failed to register `MpiBarrier` event",
                format!("{err}").as_str(),
            );
            return;
        }
    }
}

/// Registers an `MPI_Test` call into a static vector.
#[no_mangle]
pub fn register_test(
    current_rank: MpiRank,
    req: MpiReq,
    finished: bool,
    tsc: Tsc,
    duration: Tsc,
) {
    let test_event = match MpiTestBuilder::default()
        .current_rank(current_rank)
        .req(req)
        .finished(finished)
        .tsc(tsc)
        .duration(duration)
        .build()
    {
        Ok(event) => event,
        Err(err) => {
            print_err(
                current_rank,
                "failed to build `MpiTest` event",
                format!("{err}").as_str(),
            );
            return;
        }
    };

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiTest` event");
    match test_event.register(&mut guard) {
        Ok(_) => (),
        Err(err) => {
            print_err(
                current_rank,
                "failed to register `MpiTest` event",
                format!("{err}").as_str(),
            );
            return;
        }
    }
}

/// Registers an `MPI_Wait` call into a static vector.
#[no_mangle]
pub fn register_wait(current_rank: MpiRank, req: MpiReq, tsc: Tsc, duration: Tsc) {
    let wait_event = match MpiWaitBuilder::default()
        .current_rank(current_rank)
        .req(req)
        .tsc(tsc)
        .duration(duration)
        .build()
    {
        Ok(event) => event,
        Err(err) => {
            print_err(
                current_rank,
                "failed to build `MpiWait` event",
                format!("{err}").as_str(),
            );
            return;
        }
    };

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiWait` event");
    match wait_event.register(&mut guard) {
        Ok(_) => (),
        Err(err) => {
            print_err(
                current_rank,
                "failed to register `MpiWait` event",
                format!("{err}").as_str(),
            );
            return;
        }
    }
}

fn print_err(rank: MpiRank, err: &str, reason: &str) {
    eprintln!(
        "{} {} \n  {} {}",
        format!("\x1b[1;31merror[rank {}]:\x1b[0m", rank.to_string()).as_str(),
        format!("\x1b[1m{}\x1b[0m", err),
        "\x1b[34m-->\x1b[0m",
        reason
    );
}
