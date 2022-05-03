CC=gcc
MPICC=mpicc

SRC=src
TARGET_DIR=target


RM=rm -f
MAKEDEPEND=makedepend


OPTI=-O2 -march=native -mtune=native -funroll-loops -finline-functions -ftree-vectorize
CFLAGS=-Wall -g $(OPTION) $(OPTI)
LDFLAGS=-lm -fopenmp 

TARGET=sync

all: $(TARGET)

sync: sync.c
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)


clean:
	$(RM) sync

.PHONY: clean all 




