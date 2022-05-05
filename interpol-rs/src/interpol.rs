use crate::mpi_events::{
    collectives::{
        mpi_ibcast::MpiIbcastBuilder, mpi_igather::MpiIgatherBuilder,
        mpi_ireduce::MpiIreduceBuilder, mpi_iscatter::MpiIscatterBuilder,
    },
    management::{
        mpi_finalize::MpiFinalizeBuilder, mpi_init::MpiInitBuilder,
        mpi_init_thread::MpiInitThreadBuilder,
    },
    point_to_point::{
        mpi_irecv::MpiIrecvBuilder, mpi_isend::MpiIsendBuilder, mpi_recv::MpiRecvBuilder,
        mpi_send::MpiSendBuilder,
    },
    synchronization::{
        mpi_barrier::MpiBarrierBuilder, mpi_ibarrier::MpiIbarrierBuilder, mpi_test::MpiTestBuilder,
        mpi_wait::MpiWaitBuilder,
    },
};
use crate::types::{MpiComm, MpiRank, MpiReq, MpiTag, Tsc, Usecs};
use crate::InterpolError;
use lazy_static::lazy_static;
use rayon::prelude::*;
use std::fs::{self, File};
use std::io::Write;
use std::sync::Mutex;

static INTERPOL_DIR: &str = "interpol-tmp";

#[repr(transparent)]
pub struct Trace(Mutex<Vec<Box<dyn Register>>>);

#[typetag::serde(tag = "type")]
pub trait Register: Send + Sync {
    fn register(
        self,
        events: &mut Vec<Box<dyn Register>>,
    ) -> Result<(), std::collections::TryReserveError>;

    fn tsc(&self) -> Tsc;
}

#[macro_export]
macro_rules! impl_register {
    ($t:ty) => {
        use crate::interpol::Register;
        use std::collections::TryReserveError;

        #[typetag::serde]
        impl Register for $t {
            fn register(self, events: &mut Vec<Box<dyn Register>>) -> Result<(), TryReserveError> {
                // Ensure that the program does not panic if allocation fails
                events.try_reserve_exact(2 * events.len())?;
                events.push(Box::new(self));
                Ok(())
            }

            fn tsc(&self) -> crate::types::Tsc {
                self.tsc
            }
        }
    };
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

#[derive(Debug, PartialEq)]
#[repr(i8)]
pub enum MpiCallType {
    Init,
    Initthread,
    Finalize,
    Send,
    Recv,
    Isend,
    Irecv,
    Test,
    Wait,
    Barrier,
    Ibarrier,
    Ibcast,
    Igather,
    Ireduce,
    Iscatter,
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct MpiCall {
    time: Usecs,
    tsc: Tsc,
    duration: Tsc,
    partner_rank: MpiRank,
    current_rank: MpiRank,
    nb_bytes_s: u32,
    nb_bytes_r: u32,
    comm: MpiComm,
    req: MpiReq,
    tag: MpiTag,
    required_thread_lvl: i32,
    provided_thread_lvl: i32,
    finished: bool,
    op_type: i8,
    kind: MpiCallType,
}

/// Serialize the contents of the `Vec` and write them to an output file
fn serialize(
    events: &mut Vec<Box<dyn Register>>,
    current_rank: MpiRank,
) -> Result<(), InterpolError> {
    let ser_traces = serde_json::to_string_pretty(events)
        .expect("failed to serialize vector contents to string");
    let filename = format!(
        "{}/rank{}_traces.json",
        INTERPOL_DIR,
        current_rank.to_string()
    );

    fs::create_dir_all(INTERPOL_DIR)?;
    let mut file = File::create(filename.clone())?;
    write!(file, "{}", ser_traces)?;
    Ok(())
}

#[no_mangle]
pub extern "C" fn register_mpi_call(mpi_call: MpiCall) {
    let rank = mpi_call.current_rank;
    match dispatch(mpi_call) {
        Ok(_) => (),
        Err(e) => eprintln!("Rank {}: {e}", rank),
    }
}

fn dispatch(call: MpiCall) -> Result<(), InterpolError> {
    match call.kind {
        MpiCallType::Init => register_init(call.current_rank, call.tsc, call.time),
        MpiCallType::Initthread => register_init_thread(
            call.current_rank,
            call.required_thread_lvl,
            call.provided_thread_lvl,
            call.tsc,
            call.time,
        ),
        MpiCallType::Finalize => register_finalize(call.current_rank, call.tsc, call.time),
        MpiCallType::Send => register_send(
            call.current_rank,
            call.partner_rank,
            call.nb_bytes_s,
            call.comm,
            call.tag,
            call.tsc,
            call.duration,
        ),
        MpiCallType::Isend => register_isend(
            call.current_rank,
            call.partner_rank,
            call.nb_bytes_s,
            call.comm,
            call.req,
            call.tag,
            call.tsc,
            call.duration,
        ),
        MpiCallType::Recv => register_recv(
            call.current_rank,
            call.partner_rank,
            call.nb_bytes_r,
            call.comm,
            call.tag,
            call.tsc,
            call.duration,
        ),
        MpiCallType::Irecv => register_irecv(
            call.current_rank,
            call.partner_rank,
            call.nb_bytes_r,
            call.comm,
            call.req,
            call.tag,
            call.tsc,
            call.duration,
        ),
        MpiCallType::Barrier => {
            register_barrier(call.current_rank, call.comm, call.tsc, call.duration)
        }
        MpiCallType::Ibarrier => register_ibarrier(
            call.current_rank,
            call.comm,
            call.req,
            call.tsc,
            call.duration,
        ),
        MpiCallType::Test => register_test(
            call.current_rank,
            call.req,
            call.finished,
            call.tsc,
            call.duration,
        ),
        MpiCallType::Wait => register_wait(call.current_rank, call.req, call.tsc, call.duration),
        MpiCallType::Ibcast => register_ibcast(
            call.current_rank,
            call.partner_rank,
            call.nb_bytes_s,
            call.comm,
            call.req,
            call.tsc,
            call.duration,
        ),
        MpiCallType::Igather => register_igather(
            call.current_rank,
            call.partner_rank,
            call.nb_bytes_s,
            call.nb_bytes_r,
            call.comm,
            call.req,
            call.tsc,
            call.duration,
        ),
        MpiCallType::Ireduce => register_ireduce(
            call.current_rank,
            call.partner_rank,
            call.nb_bytes_s,
            call.op_type,
            call.comm,
            call.req,
            call.tsc,
            call.duration,
        ),
        MpiCallType::Iscatter => register_iscatter(
            call.current_rank,
            call.partner_rank,
            call.nb_bytes_s,
            call.nb_bytes_r,
            call.comm,
            call.req,
            call.tsc,
            call.duration,
        ),
    }
}

/// Registers an `MPI_Init` call into a static vector.
fn register_init(current_rank: MpiRank, tsc: Tsc, time: Usecs) -> Result<(), InterpolError> {
    let init_event = MpiInitBuilder::default()
        .current_rank(current_rank)
        .tsc(tsc)
        .time(time)
        .build()?;

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiInit` event");
    init_event.register(&mut guard)?;

    Ok(())
}

/// Registers an `MPI_Init_thread` call into a static vector.
fn register_init_thread(
    current_rank: MpiRank,
    required_thread_lvl: i32,
    provided_thread_lvl: i32,
    tsc: Tsc,
    time: Usecs,
) -> Result<(), InterpolError> {
    let init_thread_event = MpiInitThreadBuilder::default()
        .current_rank(current_rank)
        .required_thread_lvl(required_thread_lvl)
        .provided_thread_lvl(provided_thread_lvl)
        .tsc(tsc)
        .time(time)
        .build()?;

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiInitThread` event");
    init_thread_event.register(&mut guard)?;

    Ok(())
}

/// Registers an `MPI_Finalize` call into a static vector.
///
/// As this *should* be the final registered event, the contents of the vector will be sorted with
/// every other MPI processes vectors' and then serialized.
fn register_finalize(current_rank: MpiRank, tsc: Tsc, time: Usecs) -> Result<(), InterpolError> {
    let finalize_event = MpiFinalizeBuilder::default()
        .current_rank(current_rank)
        .tsc(tsc)
        .time(time)
        .build()?;

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiFinalize` event");
    finalize_event.register(&mut guard)?;

    // Serialize all events of the current rank
    serialize(&mut *guard, current_rank)?;
    Ok(())
}

/// Registers an `MPI_Send` call into a static vector.
fn register_send(
    current_rank: MpiRank,
    partner_rank: MpiRank,
    nb_bytes: u32,
    comm: MpiComm,
    tag: MpiTag,
    tsc: Tsc,
    duration: Tsc,
) -> Result<(), InterpolError> {
    let send_event = MpiSendBuilder::default()
        .current_rank(current_rank)
        .partner_rank(partner_rank)
        .nb_bytes(nb_bytes)
        .comm(comm)
        .tag(tag)
        .tsc(tsc)
        .duration(duration)
        .build()?;

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiSend` event");
    send_event.register(&mut guard)?;

    Ok(())
}

/// Registers an `MPI_Recv` call into a static vector.
fn register_recv(
    current_rank: MpiRank,
    partner_rank: MpiRank,
    nb_bytes: u32,
    comm: MpiComm,
    tag: MpiTag,
    tsc: Tsc,
    duration: Tsc,
) -> Result<(), InterpolError> {
    let recv_event = MpiRecvBuilder::default()
        .current_rank(current_rank)
        .partner_rank(partner_rank)
        .nb_bytes(nb_bytes)
        .comm(comm)
        .tag(tag)
        .tsc(tsc)
        .duration(duration)
        .build()?;

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiRecv` event");
    recv_event.register(&mut guard)?;

    Ok(())
}

/// Registers an `MPI_Isend` call into a static vector.
fn register_isend(
    current_rank: MpiRank,
    partner_rank: MpiRank,
    nb_bytes: u32,
    comm: MpiComm,
    req: MpiReq,
    tag: MpiTag,
    tsc: Tsc,
    duration: Tsc,
) -> Result<(), InterpolError> {
    let isend_event = MpiIsendBuilder::default()
        .current_rank(current_rank)
        .partner_rank(partner_rank)
        .nb_bytes(nb_bytes)
        .comm(comm)
        .req(req)
        .tag(tag)
        .tsc(tsc)
        .duration(duration)
        .build()?;

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiIsend` event");
    isend_event.register(&mut guard)?;

    Ok(())
}

/// Registers an `MPI_Irecv` call into a static vector.
fn register_irecv(
    current_rank: MpiRank,
    partner_rank: MpiRank,
    nb_bytes: u32,
    comm: MpiComm,
    req: MpiReq,
    tag: MpiTag,
    tsc: Tsc,
    duration: Tsc,
) -> Result<(), InterpolError> {
    let irecv_event = MpiIrecvBuilder::default()
        .current_rank(current_rank)
        .partner_rank(partner_rank)
        .nb_bytes(nb_bytes)
        .comm(comm)
        .req(req)
        .tag(tag)
        .tsc(tsc)
        .duration(duration)
        .build()?;

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiIrecv` event");
    irecv_event.register(&mut guard)?;

    Ok(())
}

/// Registers an `MPI_Barrier` call into a static vector.
fn register_barrier(
    current_rank: MpiRank,
    comm: MpiComm,
    tsc: Tsc,
    duration: Tsc,
) -> Result<(), InterpolError> {
    let barrier_event = MpiBarrierBuilder::default()
        .current_rank(current_rank)
        .comm(comm)
        .tsc(tsc)
        .duration(duration)
        .build()?;

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiBarrier` event");
    barrier_event.register(&mut guard)?;

    Ok(())
}

/// Registers an `MPI_Ibarrier` call into a static vector.
fn register_ibarrier(
    current_rank: MpiRank,
    comm: MpiComm,
    req: MpiReq,
    tsc: Tsc,
    duration: Tsc,
) -> Result<(), InterpolError> {
    let ibarrier_event = MpiIbarrierBuilder::default()
        .current_rank(current_rank)
        .comm(comm)
        .req(req)
        .tsc(tsc)
        .duration(duration)
        .build()?;

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiIbarrier` event");
    ibarrier_event.register(&mut guard)?;

    Ok(())
}

/// Registers an `MPI_Test` call into a static vector.
fn register_test(
    current_rank: MpiRank,
    req: MpiReq,
    finished: bool,
    tsc: Tsc,
    duration: Tsc,
) -> Result<(), InterpolError> {
    let test_event = MpiTestBuilder::default()
        .current_rank(current_rank)
        .req(req)
        .finished(finished)
        .tsc(tsc)
        .duration(duration)
        .build()?;

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiTest` event");
    test_event.register(&mut guard)?;

    Ok(())
}

/// Registers an `MPI_Wait` call into a static vector.
fn register_wait(
    current_rank: MpiRank,
    req: MpiReq,
    tsc: Tsc,
    duration: Tsc,
) -> Result<(), InterpolError> {
    let wait_event = MpiWaitBuilder::default()
        .current_rank(current_rank)
        .req(req)
        .tsc(tsc)
        .duration(duration)
        .build()?;

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiWait` event");
    wait_event.register(&mut guard)?;

    Ok(())
}

fn register_ibcast(
    current_rank: MpiRank,
    partner_rank: MpiRank,
    nb_bytes: u32,
    comm: MpiComm,
    req: MpiReq,
    tsc: Tsc,
    duration: Tsc,
) -> Result<(), InterpolError> {
    let ibcast_event = MpiIbcastBuilder::default()
        .current_rank(current_rank)
        .partner_rank(partner_rank)
        .nb_bytes(nb_bytes)
        .comm(comm)
        .req(req)
        .tsc(tsc)
        .duration(duration)
        .build()?;

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiIbcast` event");
    ibcast_event.register(&mut guard)?;
    Ok(())
}

fn register_igather(
    current_rank: MpiRank,
    partner_rank: MpiRank,
    nb_bytes_send: u32,
    nb_bytes_recv: u32,
    comm: MpiComm,
    req: MpiReq,
    tsc: Tsc,
    duration: Tsc,
) -> Result<(), InterpolError> {
    let igather_event = MpiIgatherBuilder::default()
        .current_rank(current_rank)
        .partner_rank(partner_rank)
        .nb_bytes_send(nb_bytes_send)
        .nb_bytes_recv(nb_bytes_recv)
        .comm(comm)
        .req(req)
        .tsc(tsc)
        .duration(duration)
        .build()?;

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiIbcast` event");
    igather_event.register(&mut guard)?;
    Ok(())
}

fn register_ireduce(
    current_rank: MpiRank,
    partner_rank: MpiRank,
    nb_bytes: u32,
    op_type: i8,
    comm: MpiComm,
    req: MpiReq,
    tsc: Tsc,
    duration: Tsc,
) -> Result<(), InterpolError> {
    let ireduce_event = MpiIreduceBuilder::default()
        .current_rank(current_rank)
        .partner_rank(partner_rank)
        .nb_bytes(nb_bytes)
        .op_type(op_type)
        .comm(comm)
        .req(req)
        .tsc(tsc)
        .duration(duration)
        .build()?;

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiIbcast` event");
    ireduce_event.register(&mut guard)?;
    Ok(())
}

fn register_iscatter(
    current_rank: MpiRank,
    partner_rank: MpiRank,
    nb_bytes_send: u32,
    nb_bytes_recv: u32,
    comm: MpiComm,
    req: MpiReq,
    tsc: Tsc,
    duration: Tsc,
) -> Result<(), InterpolError> {
    let iscatter_event = MpiIscatterBuilder::default()
        .current_rank(current_rank)
        .partner_rank(partner_rank)
        .nb_bytes_send(nb_bytes_send)
        .nb_bytes_recv(nb_bytes_recv)
        .comm(comm)
        .req(req)
        .tsc(tsc)
        .duration(duration)
        .build()?;

    let mut guard = EVENTS
        .0
        .lock()
        .expect("failed to take the lock on vector for `MpiIbcast` event");
    iscatter_event.register(&mut guard)?;
    Ok(())
}

#[no_mangle]
pub extern "C" fn sort_all_traces() {
    let mut all_traces = match deserialize_all_traces() {
        Ok(t) => t,
        Err(e) => panic!("{e}"),
    };

    let start = std::time::Instant::now();
    all_traces.par_sort_unstable_by_key(|event| event.tsc());
    let end = start.elapsed();
    eprintln!("Sort took {end:?}");

    let serialized_traces =
        serde_json::to_string_pretty(&all_traces).expect("failed to serialize all traces");
    match write_all_traces(serialized_traces) {
        Ok(_) => (),
        Err(e) => eprintln!("{e}"),
    }
}

fn deserialize_all_traces() -> Result<Vec<Box<dyn Register>>, InterpolError> {
    let mut all_traces = Vec::new();

    for entry in fs::read_dir(INTERPOL_DIR)? {
        let dir = entry?;
        let contents = fs::read_to_string(dir.path())?;
        let mut deserialized: Vec<Box<dyn Register>> =
            serde_json::from_str(&contents).expect("failed to deserialize trace file contents");
        all_traces.append(&mut deserialized);
    }

    Ok(all_traces)
}

fn write_all_traces(serialized_traces: String) -> Result<(), InterpolError> {
    let mut file = File::create(format!("{}/{}", INTERPOL_DIR, "interpol_traces.json"))?;
    write!(file, "{}", serialized_traces)?;
    Ok(())
}
