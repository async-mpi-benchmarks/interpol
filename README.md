# Interpol

Interpol is an interposition library designed to trace and profile
non-blocking MPI calls.
This tools aims at evaluating the efficiency of non-blocking MPI applications,
specifically measuring the communication-computation overlap and pin pointing
the critical code sections.

The library generates JSON traces of the intercepted MPI events which can 
thereafter be exploited using the project's provided GUI interface, [MPI Trace
Analyzer](https://github.com/async-mpi-benchmarks/Interface), developed
alongside the Interpol library.

## Features
Currently, the library redefines the following *OpenMPI* functions:
- `MPI_Init`/`MPI_Finalize`
- `MPI_Send`/`MPI_Recv`
- `MPI_Isend`/`MPI_Irecv`
- `MPI_Wait`

At the moment, Interpol only support the `MPI_COMM_WORLD` communicator (support
for user-defined communicators is planned in the future).
Although only `MPI_Init` has been redefined, the library is thread-safe.

## Dependencies
For the library to work, please make sure that the following dependencies are
installed on your system:
- cargo
- gcc + mpicc
- OpenMPI
- GNU Make

## Building
To build the library, it is recommended to use the provided Makefile:
```sh
make
sudo make setup # optional
```
This will first call `cargo` to build the Rust back-end and then compile the
interposition library into a single `.so` file.
Running the optional `sudo make setup` will set the `LD_LIBRARY_PATH`
environment variable and install the library on your system.

If you wish to do this manually, you will need to execute the following command:
```sh
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:absolute_path/interpol/interpol-rs/target/release/
```
To then preload the library at runtime, the command should be:
```sh
LD_PRELOAD=absolute_path/interpol/libinterpol.so mpirun -np NB_PROC <mpi_binary>
```

You can also check the documentation for the Rust back-end with the `make doc`
command and run the unit tests with `make test`.
