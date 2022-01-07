# Interpol

An interposition library for tracing MPI calls. This tools aims at helping developers
when writing MPI applications that take advantage of asynchronous calls such as
`MPI_Isend` or `MPI_Irecv`.

The library will generate a trace of all MPI events which can thereafter be exploited
using the GUI interface, **MPI Trace Analyzer**, we developed alongside the **Interpol** library.

## Dependencies
This library mainly depends on Rust (2018 edition) and a MPI toolkit.
You will need to have Rust's build tool `cargo` installed as well as a MPI compiler such as `mpicc`.

*TODO: exhaustive list may be necessary*

## Building
To build the library, it is recommended to use the provided Makefile:
```sh
make
sudo make setup # optional
```
This will first call `cargo` to build the Rust back-end and then compile the interposition library
into a single `.so` file.
Running the optional `sudo make setup` will set the `LD_LIBRARY_PATH` environment variable and
install the library on your system.

If you wish to do this manually, you will need to execute the following command:
```sh
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/absolute-path-to/interpol/interpol-rs/target/release/
```
To then preload the library at runtime, the command should be:
```sh
LD_PRELOAD=/absolute-path-to/interpol/libinterpol.so mpirun -np NB_PROC <mpi_binary>
```

You can also check the documentation for the Rust back-end with the `make doc` command and run
the tests with `make test`.
