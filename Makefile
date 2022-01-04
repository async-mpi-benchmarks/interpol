# Macros
CC=gcc
CFLAGS=-std=c11 -Wall -Wextra -g3
OFLAGS=-march=native -mtune=native -O2

RS_SRC=interpol-rs/src
INTERPOL-RS_LIB=interpol-rs/target/release/

build: interpol-rs src/interpol.c src/sync.c
	$(CC) $(CFLAGS) $(OFLAGS) -Iinclude/ -L$(INTERPOL-RS_LIB) -fPIC -shared src/interpol.c src/sync.c -o libinterpol.so -linterpol_rs 

interpol-rs: $(RS_SRC)/*.rs
	@cd interpol-rs/ && cargo build --release

test: $(RS_SRC)/*.rs
	@cd interpol-rs/ && cargo test
	
docs: $(RS_SRC)/*.rs
	@cd interpol-rs/ && cargo doc --open

clean:
	@cd interpol-rs/ && cargo clean
