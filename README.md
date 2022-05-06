# Interpol

**Interpol** is a thread-safe interposition library designed to trace and profile the runtime behavior of non-blocking MPI routines.
This tools aims at evaluating the efficiency of MPI applications that use the non-blocking calls defined in the standard, specifically measuring the communication-computation overlap and pin pointing the critical code sections.

The library generates JSON traces of the intercepted MPI events, which can thereafter be exploited using the project's provided GUI interface, [Interpol Trace Analyzer](https://github.com/async-mpi-benchmarks/Interface), developed alongside the **Interpol** library.


## Features
Currently, the library redefines the following **MPI** functions:
- `MPI_Init`/`MPI_Init_thread`;
- `MPI_Finalize`;
- `MPI_Send`/`MPI_Recv`;
- `MPI_Isend`/`MPI_Irecv`;
- `MPI_Barrier`/`MPI_Ibarrier`;
- `MPI_Test`;
- `MPI_Wait`;
- `MPI_Ibcast`;
- `MPI_Igather`;
- `MPI_Ireduce`;
- `MPI_Iscatter`.

At the moment, **Interpol Trace Analyzer** only support the `MPI_COMM_WORLD` communicator (support for user-defined communicators is planned in the future).

Currently, the library has been tested with the following MPI implementations:
- [OpenMPI](https://www.open-mpi.org/);
- [MPICH](https://www.mpich.org/);
- [MPC](https://mpc.hpcframework.com/frontpage/).


## Dependencies
To build the library and for it to work properly, please make sure that the following dependencies are installed on your system:
- Rust 2021 edition (v.1.56.0 or above, *nightly* channel required);
- an MPI implementation and its provided patched compiler;
- GNU Make.


## Building
To build the library, it is recommended to use the provided Makefile:
```sh
make
```
Optionnaly, you can install/uninstall it from your computer (in `/usr/lib/` by default):
```sh
sudo make install
sudo make uninstall
```
This will first call `cargo` to build the Rust back-end in release mode (automatically exported to the `LD_LIBRARY_PATH` environment variable).
Then, it will compile the interposition library into a single `.so` file.


## Usage
**IMPORTANT NOTE:** It is mandatory that you compile both the Interpol library and the MPI application that you want to trace using the _same_ `mpicc` compiler. This is because the MPI standard does not enforce any particular ABI, therefore, if the library and your program are not compiled with the same MPI implementation, conflicts may cause the traced program or the library to crash or generate incorrect traces.

If you've installed the library, the command to preload it when running your MPI application should be:
```sh
LD_PRELOAD=libinterpol.so <MPICMD> -n <NB_PROC> <BINARY>
```
Otherwise, you need to provide the absolute path to the `libinterpol.so` file.

You can also check the documentation for the Rust back-end with the `make doc` command and run the unit tests with `make test`.
