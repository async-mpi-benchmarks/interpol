#include <time.h>
#include "mpi.h"
#include <stdio.h>
#include "synchronisation.h"

int MPI_Init(int *argc, char ***argv)
{
	struct timespec debut, fin;

	//appel barrier

  	clock_gettime( CLOCK_REALTIME, &debut);

    int e = PMPI_Init(argc, argv);

    clock_gettime( CLOCK_REALTIME, &fin);

    //appel fonction en rust

    return e;
}

int MPI_Send(const void *buf, int count, MPI_Datatype datatype, int dest, int tag, MPI_Comm comm)
{
	struct timespec debut, fin;

  	clock_gettime( CLOCK_REALTIME, &debut);

    int e = PMPI_Send(buf, count, datatype, dest, tag, comm);

    clock_gettime( CLOCK_REALTIME, &fin);

    //appel fonction en rust

    return e;
}

int MPI_Recv(void *buf, int count, MPI_Datatype datatype, int source, int tag,
             MPI_Comm comm, MPI_Status * status)
{
	struct timespec debut, fin;

  	clock_gettime( CLOCK_REALTIME, &debut);

    int e = PMPI_Send(buf, count, datatype, source, tag, comm, status);

    clock_gettime( CLOCK_REALTIME, &fin);

    //appel fonction en rust

    return e;
}

int MPI_Wait(MPI_Request *request, MPI_Status *status)
{
	struct timespec debut, fin;

  	clock_gettime( CLOCK_REALTIME, &debut);

    int e = PMPI_Wait(request, status);

    clock_gettime( CLOCK_REALTIME, &fin);

    //appel fonction en rust

    return e;
}

int MPI_Isend(const void *buf, int count, MPI_Datatype datatype, int dest, int tag,
              MPI_Comm comm, MPI_Request *request)
{
	struct timespec debut, fin;

  	clock_gettime( CLOCK_REALTIME, &debut);

    int e = PMPI_Isend(buf, count, datatype, dest, tag, comm, request);

    clock_gettime( CLOCK_REALTIME, &fin);

    //appel fonction en rust

    return e;
}

int MPI_Irecv(void *buf, int count, MPI_Datatype datatype, int source,
              int tag, MPI_Comm comm, MPI_Request * request)
{
	struct timespec debut, fin;

  	clock_gettime( CLOCK_REALTIME, &debut);

    int e = PMPI_Irecv(buf, count, datatype, source, tag, comm, request);

    clock_gettime( CLOCK_REALTIME, &fin);

    //appel fonction en rust

    return e;
}

int MPI_Finalize( void )
{
	struct timespec debut, fin;

	//appel barrier

  	clock_gettime( CLOCK_REALTIME, &debut);

    int e = PMPI_Finalize();

    clock_gettime( CLOCK_REALTIME, &fin);

    //appel fonction en rust

    return e;
}