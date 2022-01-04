# Macros
CC=mpicc
CFLAGS=-std=c11 -Wall -Wextra -g3
OFLAGS=-march=native -mtune=native -O2

SRC=src
INCLUDE=include
RS_SRC=interpol-rs/src
RS_LIB=interpol-rs/target/release

build: libinterpol.so

libinterpol.so: $(RS_LIB)/libinterpol_rs.so $(SRC)/*.c $(INCLUDE)/*.h
	$(CC) $(CFLAGS) $(OFLAGS) -I$(INCLUDE) -L$(RS_LIB) -fPIC -shared $< -o $@ -linterpol_rs

$(RS_LIB)/libinterpol_rs.so: $(RS_SRC)/*.rs
	@cd interpol-rs/ && cargo build --release

test: $(RS_SRC)/*.rs
	@cd interpol-rs/ && cargo test
	
doc: $(RS_SRC)/*.rs
	@cd interpol-rs/ && cargo doc --document-private-items --open

clean:
	@cd interpol-rs/ && cargo clean
	@rm -f libinterpol.so
