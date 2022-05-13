# Macros
CC=mpicc
CFLAGS=-Wall -Wextra -g3 -I include -L interpol-rs/target/release -fPIC -shared
OFLAGS=-march=native -mtune=native -Os

INCLUDE=include
RS_SRC=interpol-rs/src
RS_LIB=interpol-rs/target/release
SRC=src

PWD=$(shell pwd)

.PHONY: build install uninstall test doc reset clean

build: libinterpol.so libinterpol-f.so
	@export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(PWD)/interpol-rs/target/release/:$(PWD)

install: build
	@cp libinterpol.so libinterpol-f.so interpol-rs/target/release/libinterpol_rs.so /usr/lib/

uninstall:
	@rm /usr/lib/libinterpol.so /usr/lib/libinterpol-f.so /usr/lib/libinterpol_rs.so 

libinterpol.so: $(RS_LIB)/libinterpol_rs.so $(SRC)/interpol-c.c
	$(CC) $(CFLAGS) $(OFLAGS) $(SRC)/interpol-c.c -o $@ -linterpol_rs

libinterpol-f.so: $(RS_LIB)/libinterpol_rs.so $(SRC)/interpol-f.c
	$(CC) $(CFLAGS) $(OFLAGS) $(SRC)/interpol-f.c -o $@ -linterpol_rs

$(RS_LIB)/libinterpol_rs.so: $(RS_SRC)/*.rs
	@cd interpol-rs/ && cargo build --release

test: $(RS_SRC)/*.rs
	@cd interpol-rs/ && cargo test
	
doc: $(RS_SRC)/*.rs
	@cd interpol-rs/ && cargo doc --document-private-items --open

reset:
	@rm -Rf libinterpol.so libinterpol-f.so

clean:
	@cd interpol-rs/ && cargo clean
	@rm -Rf libinterpol.so libinterpol-f.so
