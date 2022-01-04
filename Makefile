# Macros
CC=mpicc
CFLAGS=-std=c11 -Wall -Wextra -g3
OFLAGS=-march=native -mtune=native -O2

SRC=src
INCLUDE=include
RS_SRC=interpol-rs/src
RS_LIB=interpol-rs/target/release

build: interpol-rs libinterpol.so

libinterpol.so: $(SRC)/*.c $(INCLUDE)/*.h
	$(CC) $(CFLAGS) $(OFLAGS) -I$(INCLUDE) -L$(RS_LIB) -fPIC -shared $< -o $@ -linterpol_rs

interpol-rs: $(RS_SRC)/*.rs
	@cd interpol-rs/ && cargo build --release

test: $(RS_SRC)/*.rs
	@cd interpol-rs/ && cargo test
	
docs: $(RS_SRC)/*.rs
	@cd interpol-rs/ && cargo doc --open

clean:
	@cd interpol-rs/ && cargo clean
