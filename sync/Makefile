CC=gcc
MPICC=mpicc

SRC=src
TARGET_DIR=target


RM=rm -f
MAKEDEPEND=makedepend

OPTI=-O2
CFLAGS=-Wall -g $(OPTION) $(OPTI)
LDFLAGS=-lm -fopenmp


JSON_C_DIR=/usr/local
CFLAGS+= -I$(JSON_C_DIR)/include/json-c
LDFLAGS+=-L$(JSON_C_DIR)/lib -ljson-c


TARGET=sync

all: $(TARGET)

sync: sync.c
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)


clean:
	$(RM) sync

.PHONY: clean all 




