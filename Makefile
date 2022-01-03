# Macros
CC=gcc
CFLAGS=-std=c11 -Wall -Wextra -g3
OFLAGS=-march=native -mtune=native -O2

RS_SRC=interpol-rs/src
INTERPOL-RS_LIB=interpol-rs/target/release/

build: interpol-rs src/interpol.c src/sync.c
	$(CC) $(CFLAGS) $(OFLAGS) -I include/ -L $(INTERPOL-RS_LIB) -fPIC -shared $< -linterpol-rs 

interpol-rs: $(RS_SRC)/*.rs
	@cd interpol-rs/ && cargo build --release

clean:
	@cd interpol-rs/ && cargo clean
