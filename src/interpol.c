#include <stdio.h>
#include <time.h>

#include "mpi.h"
#include "sync.h"

// TODO: Change calls to `clock_gettime` into `rdtsc` and `sync_rdtscp`
// in `MPI_Init` and `MPI_Finalize`.

int MPI_Init(int *argc, char ***argv)
{
	struct timespec before, after;

	//appel barrier

  	clock_gettime(CLOCK_REALTIME, &before);
    int ret = PMPI_Init(argc, argv);
    clock_gettime(CLOCK_REALTIME, &after);

    //appel fonction en rust

    return ret;
}

int MPI_Send(const void *buf, int count, MPI_Datatype datatype, int dest,
             int tag, MPI_Comm comm)
{
	struct timespec before, after;

  	clock_gettime(CLOCK_REALTIME, &before);
    int ret = PMPI_Send(buf, count, datatype, dest, tag, comm);
    clock_gettime(CLOCK_REALTIME, &after);

    //appel fonction en rust

    return ret;
}

int MPI_Recv(void *buf, int count, MPI_Datatype datatype, int source, int tag,
             MPI_Comm comm, MPI_Status *status)
{
	struct timespec before, after;

  	clock_gettime(CLOCK_REALTIME, &before);
    int ret = PMPI_Recv(buf, count, datatype, source, tag, comm, status);
    clock_gettime(CLOCK_REALTIME, &after);

    //appel fonction en rust

    return ret;
}

int MPI_Wait(MPI_Request *request, MPI_Status *status)
{
	struct timespec before, after;

  	clock_gettime(CLOCK_REALTIME, &before);
    int ret = PMPI_Wait(request, status);
    clock_gettime(CLOCK_REALTIME, &after);

    //appel fonction en rust

    return ret;
}

int MPI_Isend(const void *buf, int count, MPI_Datatype datatype, int dest,
              int tag, MPI_Comm comm, MPI_Request *request)
{
	struct timespec before, after;

  	clock_gettime(CLOCK_REALTIME, &before);
    int ret = PMPI_Isend(buf, count, datatype, dest, tag, comm, request);
    clock_gettime(CLOCK_REALTIME, &after);

    //appel fonction en rust

    return ret;
}

int MPI_Irecv(void *buf, int count, MPI_Datatype datatype, int source,
              int tag, MPI_Comm comm, MPI_Request * request)
{
	struct timespec before, after;

  	clock_gettime(CLOCK_REALTIME, &before);
    int ret = PMPI_Irecv(buf, count, datatype, source, tag, comm, request);
    clock_gettime(CLOCK_REALTIME, &after);

    //appel fonction en rust

    return ret;
}

int MPI_Finalize()
{
	struct timespec before, after;

	//appel barrier

  	clock_gettime(CLOCK_REALTIME, &before);
    int ret = PMPI_Finalize();
    clock_gettime(CLOCK_REALTIME, &after);

    //appel fonction en rust

    return ret;
}
