# Macros
CC=gcc
MCC=mpicc
CFLAGS=-Wall -Wextra -g3
OFLAGS=-march=native -mtune=native -O2

INCLUDE=include
RS_SRC=interpol-rs/src
RS_LIB=interpol-rs/target/release
SRC=src

build: $(RS_LIB)/libinterpol_rs.so libinterpol.so

libinterpol.so: $(INCLUDE)/tsc.h $(SRC)/interpol.c $(INCLUDE)/interpol.h
	$(MCC) $(CFLAGS) $(OFLAGS) -I$(INCLUDE) -L$(RS_LIB) -fPIC -shared $(SRC)/interpol.c -o $@ -linterpol_rs

$(RS_LIB)/libinterpol_rs.so: $(RS_SRC)/*.rs
	@cd interpol-rs/ && cargo build --release

test: $(RS_SRC)/*.rs
	@cd interpol-rs/ && cargo test
	
doc: $(RS_SRC)/*.rs
	@cd interpol-rs/ && cargo doc --document-private-items --open

clean:
	@cd interpol-rs/ && cargo clean
	@rm -Rf $(TARGET) libinterpol.so
