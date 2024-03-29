# Interpol

**Interpol** is a thread-safe interposition library designed to trace and profile the runtime behavior of non-blocking MPI routines. It intercepts calls to MPI with a C/Fortran interpostion library (these two being the de-facto languages in which MPI applications are written in), which are forwarded to a Rust backend that does all of the profiling and traces generation work.
This tools aims at evaluating the efficiency of MPI applications that use the non-blocking calls defined in the standard, specifically measuring the communication-computation overlap and pin pointing the critical code sections.

The library generates JSON traces of the intercepted MPI events, which can thereafter be exploited using the project's provided GUI interface, [Interpol Trace Analyzer](https://github.com/async-mpi-benchmarks/Interface), developed alongside the **Interpol** library.

This project was done as part of the **Parallel Programming Project** for the [M1 High Performance Computing and Simulation at University of Paris-Saclay](http://www.chps.uvsq.fr/), under the supervision of Mr [Jean-Baptiste Besnard](https://github.com/besnardjb). Although the project has already been turned in, we will keep maintening this repository and add new functionnalities to it in the coming months. Feel free to file issues or open PRs in case of bugs or to suggest additions!


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

This tool also supports tracing of Fortran applications, just make sure to preload the `libinterpol-f.so` shared library.

At the moment, **Interpol Trace Analyzer** only support the `MPI_COMM_WORLD` communicator (support for user-defined communicators is planned in the future).

Currently, the library has been tested with the following MPI implementations:
- [OpenMPI](https://www.open-mpi.org/);
- [MPICH](https://www.mpich.org/);
- [MPC](https://mpc.hpcframework.com/frontpage/).


## Dependencies
To build the library and for it to work properly, please make sure that the following dependencies are installed on your system:
- Rust 2021 edition (v.1.56.0 or above, nightly channel *required*);
- an MPI implementation and its provided patched compiler;
- GNU Make.


## Building
To build both the C and Fortran libraries, it is recommended to use the provided Makefile:
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
And for the Fortran version of Interpol:
```sh
LD_PRELOAD=libinterpol-f.so <MPICMD> -n <NB_PROC> <BINARY>
```

Otherwise, you need to provide the absolute path to the `libinterpol.so` or `libinterpol-f.so` file.

You can also check the documentation for the Rust back-end with the `make doc` command and run the unit tests with `make test`.

Link to the PMPI wrapper generator: [LLNL/wrap](https://github.com/LLNL/wrap)
